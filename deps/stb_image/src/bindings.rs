/* automatically generated by rust-bindgen */

pub const _SAL_VERSION: u32 = 20;
pub const __SAL_H_VERSION: u32 = 180000000;
pub const _USE_DECLSPECS_FOR_SAL: u32 = 0;
pub const _USE_ATTRIBUTES_FOR_SAL: u32 = 0;
pub const _CRT_PACKING: u32 = 8;
pub const _HAS_EXCEPTIONS: u32 = 1;
pub const _ARGMAX: u32 = 100;
pub const _CRT_INT_MAX: u32 = 2147483647;
pub const _CRT_FUNCTIONS_REQUIRED: u32 = 1;
pub const _ARM_WINAPI_PARTITION_DESKTOP_SDK_AVAILABLE: u32 = 0;
pub const _CRT_BUILD_DESKTOP_APP: u32 = 1;
pub const __STDC_SECURE_LIB__: u32 = 200411;
pub const __GOT_SECURE_LIB__: u32 = 200411;
pub const __STDC_WANT_SECURE_LIB__: u32 = 1;
pub const _SECURECRT_FILL_BUFFER_PATTERN: u32 = 254;
pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES: u32 = 0;
pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES_COUNT: u32 = 0;
pub const _CRT_SECURE_CPP_OVERLOAD_SECURE_NAMES: u32 = 1;
pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES_MEMORY: u32 = 0;
pub const _CRT_SECURE_CPP_OVERLOAD_SECURE_NAMES_MEMORY: u32 = 0;
pub const _CRT_INTERNAL_STDIO_SYMBOL_PREFIX: &'static [u8; 1usize] = b"\0";
pub const _CRT_INTERNAL_PRINTF_LEGACY_VSPRINTF_NULL_TERMINATION: u32 = 1;
pub const _CRT_INTERNAL_PRINTF_STANDARD_SNPRINTF_BEHAVIOR: u32 = 2;
pub const _CRT_INTERNAL_PRINTF_LEGACY_WIDE_SPECIFIERS: u32 = 4;
pub const _CRT_INTERNAL_PRINTF_LEGACY_MSVCRT_COMPATIBILITY: u32 = 8;
pub const _CRT_INTERNAL_PRINTF_LEGACY_THREE_DIGIT_EXPONENTS: u32 = 16;
pub const _CRT_INTERNAL_SCANF_SECURECRT: u32 = 1;
pub const _CRT_INTERNAL_SCANF_LEGACY_WIDE_SPECIFIERS: u32 = 2;
pub const _CRT_INTERNAL_SCANF_LEGACY_MSVCRT_COMPATIBILITY: u32 = 4;
pub const BUFSIZ: u32 = 512;
pub const _NSTREAM_: u32 = 512;
pub const _IOB_ENTRIES: u32 = 3;
pub const EOF: i32 = -1;
pub const _IOFBF: u32 = 0;
pub const _IOLBF: u32 = 64;
pub const _IONBF: u32 = 4;
pub const L_tmpnam: u32 = 260;
pub const L_tmpnam_s: u32 = 260;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const SEEK_SET: u32 = 0;
pub const FILENAME_MAX: u32 = 260;
pub const FOPEN_MAX: u32 = 20;
pub const _SYS_OPEN: u32 = 20;
pub const TMP_MAX: u32 = 2147483647;
pub const TMP_MAX_S: u32 = 2147483647;
pub const _TMP_MAX_S: u32 = 2147483647;
pub const SYS_OPEN: u32 = 20;
pub const STBI_VERSION: u32 = 1;
pub type va_list = *mut ::std::os::raw::c_char;
extern "C" {
    pub fn __va_start(arg1: *mut va_list, ...);
}
pub type __vcrt_bool = bool;
pub type wchar_t = ::std::os::raw::c_ushort;
extern "C" {
    pub fn __security_init_cookie();
}
extern "C" {
    pub fn __security_check_cookie(_StackCookie: usize);
}
extern "C" {
    pub fn __report_gsfailure(_StackCookie: usize);
}
extern "C" {
    #[link_name = "\u{1}__security_cookie"]
    pub static mut __security_cookie: usize;
}
pub type __crt_bool = bool;
extern "C" {
    pub fn _invalid_parameter_noinfo();
}
extern "C" {
    pub fn _invalid_parameter_noinfo_noreturn();
}
extern "C" {
    pub fn _invoke_watson(
        _Expression: *const wchar_t,
        _FunctionName: *const wchar_t,
        _FileName: *const wchar_t,
        _LineNo: ::std::os::raw::c_uint,
        _Reserved: usize,
    );
}
pub type errno_t = ::std::os::raw::c_int;
pub type wint_t = ::std::os::raw::c_ushort;
pub type wctype_t = ::std::os::raw::c_ushort;
pub type __time32_t = ::std::os::raw::c_long;
pub type __time64_t = ::std::os::raw::c_longlong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_data_public {
    pub _locale_pctype: *const ::std::os::raw::c_ushort,
    pub _locale_mb_cur_max: ::std::os::raw::c_int,
    pub _locale_lc_codepage: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout___crt_locale_data_public() {
    assert_eq!(
        ::std::mem::size_of::<__crt_locale_data_public>(),
        16usize,
        concat!("Size of: ", stringify!(__crt_locale_data_public))
    );
    assert_eq!(
        ::std::mem::align_of::<__crt_locale_data_public>(),
        8usize,
        concat!("Alignment of ", stringify!(__crt_locale_data_public))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__crt_locale_data_public>()))._locale_pctype as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_data_public),
            "::",
            stringify!(_locale_pctype)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__crt_locale_data_public>()))._locale_mb_cur_max as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_data_public),
            "::",
            stringify!(_locale_mb_cur_max)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__crt_locale_data_public>()))._locale_lc_codepage as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_data_public),
            "::",
            stringify!(_locale_lc_codepage)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_pointers {
    pub locinfo: *mut __crt_locale_data,
    pub mbcinfo: *mut __crt_multibyte_data,
}
#[test]
fn bindgen_test_layout___crt_locale_pointers() {
    assert_eq!(
        ::std::mem::size_of::<__crt_locale_pointers>(),
        16usize,
        concat!("Size of: ", stringify!(__crt_locale_pointers))
    );
    assert_eq!(
        ::std::mem::align_of::<__crt_locale_pointers>(),
        8usize,
        concat!("Alignment of ", stringify!(__crt_locale_pointers))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__crt_locale_pointers>())).locinfo as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_pointers),
            "::",
            stringify!(locinfo)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__crt_locale_pointers>())).mbcinfo as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_pointers),
            "::",
            stringify!(mbcinfo)
        )
    );
}
pub type _locale_t = *mut __crt_locale_pointers;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Mbstatet {
    pub _Wchar: ::std::os::raw::c_ulong,
    pub _Byte: ::std::os::raw::c_ushort,
    pub _State: ::std::os::raw::c_ushort,
}
#[test]
fn bindgen_test_layout__Mbstatet() {
    assert_eq!(
        ::std::mem::size_of::<_Mbstatet>(),
        8usize,
        concat!("Size of: ", stringify!(_Mbstatet))
    );
    assert_eq!(
        ::std::mem::align_of::<_Mbstatet>(),
        4usize,
        concat!("Alignment of ", stringify!(_Mbstatet))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_Mbstatet>()))._Wchar as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_Wchar)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_Mbstatet>()))._Byte as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_Byte)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_Mbstatet>()))._State as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_State)
        )
    );
}
pub type mbstate_t = _Mbstatet;
pub type time_t = __time64_t;
pub type rsize_t = usize;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _iobuf {
    pub _Placeholder: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout__iobuf() {
    assert_eq!(
        ::std::mem::size_of::<_iobuf>(),
        8usize,
        concat!("Size of: ", stringify!(_iobuf))
    );
    assert_eq!(
        ::std::mem::align_of::<_iobuf>(),
        8usize,
        concat!("Alignment of ", stringify!(_iobuf))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_iobuf>()))._Placeholder as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_iobuf),
            "::",
            stringify!(_Placeholder)
        )
    );
}
pub type FILE = _iobuf;
extern "C" {
    pub fn __acrt_iob_func(_Ix: ::std::os::raw::c_uint) -> *mut FILE;
}
extern "C" {
    pub fn fgetwc(_Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _fgetwchar() -> wint_t;
}
extern "C" {
    pub fn fputwc(_Character: wchar_t, _Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _fputwchar(_Character: wchar_t) -> wint_t;
}
extern "C" {
    pub fn getwc(_Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn getwchar() -> wint_t;
}
extern "C" {
    pub fn fgetws(
        _Buffer: *mut wchar_t,
        _BufferCount: ::std::os::raw::c_int,
        _Stream: *mut FILE,
    ) -> *mut wchar_t;
}
extern "C" {
    pub fn fputws(_Buffer: *const wchar_t, _Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _getws_s(_Buffer: *mut wchar_t, _BufferCount: usize) -> *mut wchar_t;
}
extern "C" {
    pub fn putwc(_Character: wchar_t, _Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn putwchar(_Character: wchar_t) -> wint_t;
}
extern "C" {
    pub fn _putws(_Buffer: *const wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ungetwc(_Character: wint_t, _Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _wfdopen(_FileHandle: ::std::os::raw::c_int, _Mode: *const wchar_t) -> *mut FILE;
}
extern "C" {
    pub fn _wfopen(_FileName: *const wchar_t, _Mode: *const wchar_t) -> *mut FILE;
}
extern "C" {
    pub fn _wfopen_s(
        _Stream: *mut *mut FILE,
        _FileName: *const wchar_t,
        _Mode: *const wchar_t,
    ) -> errno_t;
}
extern "C" {
    pub fn _wfreopen(
        _FileName: *const wchar_t,
        _Mode: *const wchar_t,
        _OldStream: *mut FILE,
    ) -> *mut FILE;
}
extern "C" {
    pub fn _wfreopen_s(
        _Stream: *mut *mut FILE,
        _FileName: *const wchar_t,
        _Mode: *const wchar_t,
        _OldStream: *mut FILE,
    ) -> errno_t;
}
extern "C" {
    pub fn _wfsopen(
        _FileName: *const wchar_t,
        _Mode: *const wchar_t,
        _ShFlag: ::std::os::raw::c_int,
    ) -> *mut FILE;
}
extern "C" {
    pub fn _wperror(_ErrorMessage: *const wchar_t);
}
extern "C" {
    pub fn _wpopen(_Command: *const wchar_t, _Mode: *const wchar_t) -> *mut FILE;
}
extern "C" {
    pub fn _wremove(_FileName: *const wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _wtempnam(_Directory: *const wchar_t, _FilePrefix: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn _wtmpnam_s(_Buffer: *mut wchar_t, _BufferCount: usize) -> errno_t;
}
extern "C" {
    pub fn _wtmpnam(_Buffer: *mut wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn _fgetwc_nolock(_Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _fputwc_nolock(_Character: wchar_t, _Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _getwc_nolock(_Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _putwc_nolock(_Character: wchar_t, _Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _ungetwc_nolock(_Character: wint_t, _Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn __stdio_common_vfwprintf(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfwprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfwprintf_p(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfwscanf(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vswprintf(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut wchar_t,
        _BufferCount: usize,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vswprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut wchar_t,
        _BufferCount: usize,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vsnwprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut wchar_t,
        _BufferCount: usize,
        _MaxCount: usize,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vswprintf_p(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut wchar_t,
        _BufferCount: usize,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vswscanf(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *const wchar_t,
        _BufferCount: usize,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
pub type fpos_t = ::std::os::raw::c_longlong;
extern "C" {
    pub fn _get_stream_buffer_pointers(
        _Stream: *mut FILE,
        _Base: *mut *mut *mut ::std::os::raw::c_char,
        _Pointer: *mut *mut *mut ::std::os::raw::c_char,
        _Count: *mut *mut ::std::os::raw::c_int,
    ) -> errno_t;
}
extern "C" {
    pub fn clearerr_s(_Stream: *mut FILE) -> errno_t;
}
extern "C" {
    pub fn fopen_s(
        _Stream: *mut *mut FILE,
        _FileName: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
    ) -> errno_t;
}
extern "C" {
    pub fn fread_s(
        _Buffer: *mut ::std::os::raw::c_void,
        _BufferSize: usize,
        _ElementSize: usize,
        _ElementCount: usize,
        _Stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn freopen_s(
        _Stream: *mut *mut FILE,
        _FileName: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
        _OldStream: *mut FILE,
    ) -> errno_t;
}
extern "C" {
    pub fn gets_s(
        _Buffer: *mut ::std::os::raw::c_char,
        _Size: rsize_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tmpfile_s(_Stream: *mut *mut FILE) -> errno_t;
}
extern "C" {
    pub fn tmpnam_s(_Buffer: *mut ::std::os::raw::c_char, _Size: rsize_t) -> errno_t;
}
extern "C" {
    pub fn clearerr(_Stream: *mut FILE);
}
extern "C" {
    pub fn fclose(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fcloseall() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fdopen(
        _FileHandle: ::std::os::raw::c_int,
        _Mode: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn feof(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ferror(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fflush(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetc(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fgetchar() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetpos(_Stream: *mut FILE, _Position: *mut fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgets(
        _Buffer: *mut ::std::os::raw::c_char,
        _MaxCount: ::std::os::raw::c_int,
        _Stream: *mut FILE,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _fileno(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _flushall() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fopen(
        _FileName: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fputc(_Character: ::std::os::raw::c_int, _Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fputchar(_Character: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputs(
        _Buffer: *const ::std::os::raw::c_char,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fread(
        _Buffer: *mut ::std::os::raw::c_void,
        _ElementSize: usize,
        _ElementCount: usize,
        _Stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn freopen(
        _FileName: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
        _Stream: *mut FILE,
    ) -> *mut FILE;
}
extern "C" {
    pub fn _fsopen(
        _FileName: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
        _ShFlag: ::std::os::raw::c_int,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fsetpos(_Stream: *mut FILE, _Position: *const fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fseek(
        _Stream: *mut FILE,
        _Offset: ::std::os::raw::c_long,
        _Origin: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fseeki64(
        _Stream: *mut FILE,
        _Offset: ::std::os::raw::c_longlong,
        _Origin: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftell(_Stream: *mut FILE) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn _ftelli64(_Stream: *mut FILE) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn fwrite(
        _Buffer: *const ::std::os::raw::c_void,
        _ElementSize: usize,
        _ElementCount: usize,
        _Stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn getc(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getchar() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _getmaxstdio() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _getw(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn perror(_ErrorMessage: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn _pclose(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _popen(
        _Command: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn putc(_Character: ::std::os::raw::c_int, _Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putchar(_Character: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn puts(_Buffer: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _putw(_Word: ::std::os::raw::c_int, _Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn remove(_FileName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rename(
        _OldFileName: *const ::std::os::raw::c_char,
        _NewFileName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _unlink(_FileName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn unlink(_FileName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rewind(_Stream: *mut FILE);
}
extern "C" {
    pub fn _rmtmp() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setbuf(_Stream: *mut FILE, _Buffer: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn _setmaxstdio(_Maximum: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setvbuf(
        _Stream: *mut FILE,
        _Buffer: *mut ::std::os::raw::c_char,
        _Mode: ::std::os::raw::c_int,
        _Size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _tempnam(
        _DirectoryName: *const ::std::os::raw::c_char,
        _FilePrefix: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tmpfile() -> *mut FILE;
}
extern "C" {
    pub fn tmpnam(_Buffer: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ungetc(_Character: ::std::os::raw::c_int, _Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _lock_file(_Stream: *mut FILE);
}
extern "C" {
    pub fn _unlock_file(_Stream: *mut FILE);
}
extern "C" {
    pub fn _fclose_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fflush_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fgetc_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fputc_nolock(
        _Character: ::std::os::raw::c_int,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fread_nolock(
        _Buffer: *mut ::std::os::raw::c_void,
        _ElementSize: usize,
        _ElementCount: usize,
        _Stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn _fread_nolock_s(
        _Buffer: *mut ::std::os::raw::c_void,
        _BufferSize: usize,
        _ElementSize: usize,
        _ElementCount: usize,
        _Stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn _fseek_nolock(
        _Stream: *mut FILE,
        _Offset: ::std::os::raw::c_long,
        _Origin: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fseeki64_nolock(
        _Stream: *mut FILE,
        _Offset: ::std::os::raw::c_longlong,
        _Origin: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _ftell_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn _ftelli64_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn _fwrite_nolock(
        _Buffer: *const ::std::os::raw::c_void,
        _ElementSize: usize,
        _ElementCount: usize,
        _Stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn _getc_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _putc_nolock(
        _Character: ::std::os::raw::c_int,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _ungetc_nolock(
        _Character: ::std::os::raw::c_int,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __p__commode() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfprintf(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfprintf_p(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _set_printf_count_output(_Value: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _get_printf_count_output() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfscanf(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _Arglist: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vsprintf(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: usize,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vsprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: usize,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vsnprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: usize,
        _MaxCount: usize,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vsprintf_p(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: usize,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vsscanf(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *const ::std::os::raw::c_char,
        _BufferCount: usize,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tempnam(
        _Directory: *const ::std::os::raw::c_char,
        _FilePrefix: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fcloseall() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fdopen(
        _FileHandle: ::std::os::raw::c_int,
        _Format: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fgetchar() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fileno(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn flushall() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputchar(_Ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getw(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putw(_Ch: ::std::os::raw::c_int, _Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rmtmp() -> ::std::os::raw::c_int;
}
pub const STBI_default: _bindgen_ty_1 = 0;
pub const STBI_grey: _bindgen_ty_1 = 1;
pub const STBI_grey_alpha: _bindgen_ty_1 = 2;
pub const STBI_rgb: _bindgen_ty_1 = 3;
pub const STBI_rgb_alpha: _bindgen_ty_1 = 4;
pub type _bindgen_ty_1 = i32;
pub type stbi_uc = ::std::os::raw::c_uchar;
pub type stbi_us = ::std::os::raw::c_ushort;
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct stbi_io_callbacks {
    pub read: ::std::option::Option<
        unsafe extern "C" fn(
            user: *mut ::std::os::raw::c_void,
            data: *mut ::std::os::raw::c_char,
            size: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub skip: ::std::option::Option<
        unsafe extern "C" fn(user: *mut ::std::os::raw::c_void, n: ::std::os::raw::c_int),
    >,
    pub eof: ::std::option::Option<
        unsafe extern "C" fn(user: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
}
#[test]
fn bindgen_test_layout_stbi_io_callbacks() {
    assert_eq!(
        ::std::mem::size_of::<stbi_io_callbacks>(),
        24usize,
        concat!("Size of: ", stringify!(stbi_io_callbacks))
    );
    assert_eq!(
        ::std::mem::align_of::<stbi_io_callbacks>(),
        8usize,
        concat!("Alignment of ", stringify!(stbi_io_callbacks))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<stbi_io_callbacks>())).read as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(stbi_io_callbacks),
            "::",
            stringify!(read)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<stbi_io_callbacks>())).skip as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(stbi_io_callbacks),
            "::",
            stringify!(skip)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<stbi_io_callbacks>())).eof as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(stbi_io_callbacks),
            "::",
            stringify!(eof)
        )
    );
}
extern "C" {
    ///
    pub fn stbi_load_from_memory(
        buffer: *const stbi_uc,
        len: ::std::os::raw::c_int,
        x: *mut ::std::os::raw::c_int,
        y: *mut ::std::os::raw::c_int,
        channels_in_file: *mut ::std::os::raw::c_int,
        desired_channels: ::std::os::raw::c_int,
    ) -> *mut stbi_uc;
}
extern "C" {
    pub fn stbi_load_from_callbacks(
        clbk: *const stbi_io_callbacks,
        user: *mut ::std::os::raw::c_void,
        x: *mut ::std::os::raw::c_int,
        y: *mut ::std::os::raw::c_int,
        channels_in_file: *mut ::std::os::raw::c_int,
        desired_channels: ::std::os::raw::c_int,
    ) -> *mut stbi_uc;
}
extern "C" {
    pub fn stbi_load_gif_from_memory(
        buffer: *const stbi_uc,
        len: ::std::os::raw::c_int,
        delays: *mut *mut ::std::os::raw::c_int,
        x: *mut ::std::os::raw::c_int,
        y: *mut ::std::os::raw::c_int,
        z: *mut ::std::os::raw::c_int,
        comp: *mut ::std::os::raw::c_int,
        req_comp: ::std::os::raw::c_int,
    ) -> *mut stbi_uc;
}
extern "C" {
    pub fn stbi_load(
        filename: *const ::std::os::raw::c_char,
        x: *mut ::std::os::raw::c_int,
        y: *mut ::std::os::raw::c_int,
        channels_in_file: *mut ::std::os::raw::c_int,
        desired_channels: ::std::os::raw::c_int,
    ) -> *mut stbi_uc;
}
extern "C" {
    pub fn stbi_load_from_file(
        f: *mut FILE,
        x: *mut ::std::os::raw::c_int,
        y: *mut ::std::os::raw::c_int,
        channels_in_file: *mut ::std::os::raw::c_int,
        desired_channels: ::std::os::raw::c_int,
    ) -> *mut stbi_uc;
}
extern "C" {
    ///
    pub fn stbi_load_16_from_memory(
        buffer: *const stbi_uc,
        len: ::std::os::raw::c_int,
        x: *mut ::std::os::raw::c_int,
        y: *mut ::std::os::raw::c_int,
        channels_in_file: *mut ::std::os::raw::c_int,
        desired_channels: ::std::os::raw::c_int,
    ) -> *mut stbi_us;
}
extern "C" {
    pub fn stbi_load_16_from_callbacks(
        clbk: *const stbi_io_callbacks,
        user: *mut ::std::os::raw::c_void,
        x: *mut ::std::os::raw::c_int,
        y: *mut ::std::os::raw::c_int,
        channels_in_file: *mut ::std::os::raw::c_int,
        desired_channels: ::std::os::raw::c_int,
    ) -> *mut stbi_us;
}
extern "C" {
    pub fn stbi_load_16(
        filename: *const ::std::os::raw::c_char,
        x: *mut ::std::os::raw::c_int,
        y: *mut ::std::os::raw::c_int,
        channels_in_file: *mut ::std::os::raw::c_int,
        desired_channels: ::std::os::raw::c_int,
    ) -> *mut stbi_us;
}
extern "C" {
    pub fn stbi_load_from_file_16(
        f: *mut FILE,
        x: *mut ::std::os::raw::c_int,
        y: *mut ::std::os::raw::c_int,
        channels_in_file: *mut ::std::os::raw::c_int,
        desired_channels: ::std::os::raw::c_int,
    ) -> *mut stbi_us;
}
extern "C" {
    pub fn stbi_loadf_from_memory(
        buffer: *const stbi_uc,
        len: ::std::os::raw::c_int,
        x: *mut ::std::os::raw::c_int,
        y: *mut ::std::os::raw::c_int,
        channels_in_file: *mut ::std::os::raw::c_int,
        desired_channels: ::std::os::raw::c_int,
    ) -> *mut f32;
}
extern "C" {
    pub fn stbi_loadf_from_callbacks(
        clbk: *const stbi_io_callbacks,
        user: *mut ::std::os::raw::c_void,
        x: *mut ::std::os::raw::c_int,
        y: *mut ::std::os::raw::c_int,
        channels_in_file: *mut ::std::os::raw::c_int,
        desired_channels: ::std::os::raw::c_int,
    ) -> *mut f32;
}
extern "C" {
    pub fn stbi_loadf(
        filename: *const ::std::os::raw::c_char,
        x: *mut ::std::os::raw::c_int,
        y: *mut ::std::os::raw::c_int,
        channels_in_file: *mut ::std::os::raw::c_int,
        desired_channels: ::std::os::raw::c_int,
    ) -> *mut f32;
}
extern "C" {
    pub fn stbi_loadf_from_file(
        f: *mut FILE,
        x: *mut ::std::os::raw::c_int,
        y: *mut ::std::os::raw::c_int,
        channels_in_file: *mut ::std::os::raw::c_int,
        desired_channels: ::std::os::raw::c_int,
    ) -> *mut f32;
}
extern "C" {
    pub fn stbi_hdr_to_ldr_gamma(gamma: f32);
}
extern "C" {
    pub fn stbi_hdr_to_ldr_scale(scale: f32);
}
extern "C" {
    pub fn stbi_ldr_to_hdr_gamma(gamma: f32);
}
extern "C" {
    pub fn stbi_ldr_to_hdr_scale(scale: f32);
}
extern "C" {
    pub fn stbi_is_hdr_from_callbacks(
        clbk: *const stbi_io_callbacks,
        user: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stbi_is_hdr_from_memory(
        buffer: *const stbi_uc,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stbi_is_hdr(filename: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stbi_is_hdr_from_file(f: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stbi_failure_reason() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn stbi_image_free(retval_from_stbi_load: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn stbi_info_from_memory(
        buffer: *const stbi_uc,
        len: ::std::os::raw::c_int,
        x: *mut ::std::os::raw::c_int,
        y: *mut ::std::os::raw::c_int,
        comp: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stbi_info_from_callbacks(
        clbk: *const stbi_io_callbacks,
        user: *mut ::std::os::raw::c_void,
        x: *mut ::std::os::raw::c_int,
        y: *mut ::std::os::raw::c_int,
        comp: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stbi_is_16_bit_from_memory(
        buffer: *const stbi_uc,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stbi_is_16_bit_from_callbacks(
        clbk: *const stbi_io_callbacks,
        user: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stbi_info(
        filename: *const ::std::os::raw::c_char,
        x: *mut ::std::os::raw::c_int,
        y: *mut ::std::os::raw::c_int,
        comp: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stbi_info_from_file(
        f: *mut FILE,
        x: *mut ::std::os::raw::c_int,
        y: *mut ::std::os::raw::c_int,
        comp: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stbi_is_16_bit(filename: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stbi_is_16_bit_from_file(f: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stbi_set_unpremultiply_on_load(flag_true_if_should_unpremultiply: ::std::os::raw::c_int);
}
extern "C" {
    pub fn stbi_convert_iphone_png_to_rgb(flag_true_if_should_convert: ::std::os::raw::c_int);
}
extern "C" {
    pub fn stbi_set_flip_vertically_on_load(flag_true_if_should_flip: ::std::os::raw::c_int);
}
extern "C" {
    pub fn stbi_zlib_decode_malloc_guesssize(
        buffer: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
        initial_size: ::std::os::raw::c_int,
        outlen: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn stbi_zlib_decode_malloc_guesssize_headerflag(
        buffer: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
        initial_size: ::std::os::raw::c_int,
        outlen: *mut ::std::os::raw::c_int,
        parse_header: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn stbi_zlib_decode_malloc(
        buffer: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
        outlen: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn stbi_zlib_decode_buffer(
        obuffer: *mut ::std::os::raw::c_char,
        olen: ::std::os::raw::c_int,
        ibuffer: *const ::std::os::raw::c_char,
        ilen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stbi_zlib_decode_noheader_malloc(
        buffer: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
        outlen: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn stbi_zlib_decode_noheader_buffer(
        obuffer: *mut ::std::os::raw::c_char,
        olen: ::std::os::raw::c_int,
        ibuffer: *const ::std::os::raw::c_char,
        ilen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_data {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_multibyte_data {
    pub _address: u8,
}