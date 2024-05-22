//! Panic handler for Furi applications.
//! This will print the panic info to stdout and then trigger a crash.

use core::ffi::c_char;
use core::panic::PanicInfo;
use core::str;

use flipperzero_sys as sys;

#[panic_handler]
pub fn panic(panic_info: &PanicInfo<'_>) -> ! {
    // Format: "thread: 'App Name' panicked at 'panic!', panic.rs:5"
    // Note: Don't use `format!` as it pulls in 10 KiB+ of formatting code.
    unsafe {
        let thread_id = sys::furi_thread_get_current_id();
        let thread_name = if !thread_id.is_null() {
            sys::furi_thread_get_name(thread_id)
        } else {
            c"unknown".as_ptr()
        };

        sys::__wrap_printf(c"\x1b[0;31mthread: '%s' paniced".as_ptr(), thread_name);

        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            sys::__wrap_printf(c" at '%*s'".as_ptr(), s.len(), s.as_ptr() as *const c_char);
        }

        if let Some(location) = panic_info.location() {
            let file = location.file();
            let line = location.line();

            sys::__wrap_printf(
                c", %*s:%u".as_ptr(),
                file.len(),
                file.as_ptr() as *const c_char,
                line,
            );
        }

        sys::__wrap_printf(c"\x1b[0m\r\n".as_ptr());
        sys::furi_thread_stdout_flush();
        sys::furi_thread_yield(); // Allow console to flush

        sys::crash!("Rust panic")
    }
}
