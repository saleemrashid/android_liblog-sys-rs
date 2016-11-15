extern crate libc;

use libc::{c_char, c_int, size_t};

#[repr(C)]
pub enum LogPriority {
    UNKNOWN = 0,
    DEFAULT,
    VERBOSE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
    FATAL,
    SILENT,
}

#[link(name = "log")]
extern "C" {
    pub fn __android_log_write(prio: c_int,
                               tag: *const c_char,
                               text: *const c_char) -> c_int;

    pub fn __android_log_print(prio: c_int,
                               tag: *const c_char,
                               ...) -> c_int;

    // pub fn __android_log_vprint(prio: c_int,
    //                            tag: *const c_char,
    //                            fmt: *const c_char,
    //                            ap: va_list) -> c_int;

    pub fn __android_log_assert(prio: c_int,
                                tag: *const c_char,
                                fmt: *const c_char,
                                ...) -> !;

    pub fn __android_log_is_loggable(prio: c_int,
                                     tag: *const c_char,
                                     default_prio: c_int) -> c_int;

    pub fn __android_log_is_loggable_len(prio: c_int,
                                         tag: *const c_char,
                                         len: size_t,
                                         default_prio: c_int) -> c_int;
}
