use libc::{c_char, c_int};
use libc::printf;
use std::ffi::CString;
use std::io::Read;
use gag::BufferRedirect;

#[link(name = "ft")]
#[link(name = "ftprintf")]
extern {
	fn ft_printf(format: *const c_char, ...) -> i32;
}

pub mod printf_tests {
	use super::*;
	rusty_fork_test! {
		#[test]
		fn printf_basic_1() {
			let format = CString::new("hello, world!").unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let mut ft_buf = String::new();
			let mut lc_buf = String::new();
			let ft_res = unsafe { ft_printf(format.as_ptr()) };
			stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let lc_res = unsafe { printf(format.as_ptr()) };
			unsafe { libc::fflush(libc_stdhandle::stdout()) };
			stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
			assert_eq!(
				ft_buf, lc_buf,
				"\n\nft_printf({:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format,
				"ft:", ft_buf,
				"libc:", lc_buf
			);
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_printf({:?}) return value differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format,
				"ft:", ft_res,
				"libc:", lc_res
			);
		}
		#[test]
		fn printf_s_char() {
			let format = CString::new("hello,%cworld!").unwrap();
			let arg_01 = b' ' as libc::c_uint;
			let stdout = BufferRedirect::stdout().unwrap();
			let mut ft_buf = String::new();
			let mut lc_buf = String::new();
			let ft_res = unsafe { ft_printf(format.as_ptr(), arg_01) };
			stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let lc_res = unsafe { printf(format.as_ptr(), arg_01) };
			unsafe { libc::fflush(libc_stdhandle::stdout()) };
			stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
			assert_eq!(
				ft_buf, lc_buf,
				"\n\nft_printf({:?}, {:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_buf,
				"libc:", lc_buf
			);
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_printf({:?}, {:?}) return value differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_res,
				"libc:", lc_res
			);
		}
		#[test]
		fn printf_s_escape() {
			let format = CString::new("hello,%%world!").unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let mut ft_buf = String::new();
			let mut lc_buf = String::new();
			let ft_res = unsafe { ft_printf(format.as_ptr()) };
			stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let lc_res = unsafe { printf(format.as_ptr()) };
			unsafe { libc::fflush(libc_stdhandle::stdout()) };
			stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
			assert_eq!(
				ft_buf, lc_buf,
				"\n\nft_printf({:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format,
				"ft:", ft_buf,
				"libc:", lc_buf
			);
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_printf({:?}) return value differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format,
				"ft:", ft_res,
				"libc:", lc_res
			);
		}
		#[test]
		fn printf_s_intd() {
			let format = CString::new("%despacito").unwrap();
			let arg_01 = 123;
			let stdout = BufferRedirect::stdout().unwrap();
			let mut ft_buf = String::new();
			let mut lc_buf = String::new();
			let ft_res = unsafe { ft_printf(format.as_ptr(), arg_01) };
			stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let lc_res = unsafe { printf(format.as_ptr(), arg_01) };
			unsafe { libc::fflush(libc_stdhandle::stdout()) };
			stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
			assert_eq!(
				ft_buf, lc_buf,
				"\n\nft_printf({:?}, {:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_buf,
				"libc:", lc_buf
			);
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_printf({:?}, {:?}) return value differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_res,
				"libc:", lc_res
			);
		}
		#[test]
		fn printf_s_inti() {
			let format = CString::new("despac%ito").unwrap();
			let arg_01 = 123;
			let stdout = BufferRedirect::stdout().unwrap();
			let mut ft_buf = String::new();
			let mut lc_buf = String::new();
			let ft_res = unsafe { ft_printf(format.as_ptr(), arg_01) };
			stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let lc_res = unsafe { printf(format.as_ptr(), arg_01) };
			unsafe { libc::fflush(libc_stdhandle::stdout()) };
			stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
			assert_eq!(
				ft_buf, lc_buf,
				"\n\nft_printf({:?}, {:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_buf,
				"libc:", lc_buf
			);
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_printf({:?}, {:?}) return value differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_res,
				"libc:", lc_res
			);
		}
		#[test]
		fn printf_s_hexl() {
			let format = CString::new("asdf%xasdf").unwrap();
			let arg_01 = 123;
			let stdout = BufferRedirect::stdout().unwrap();
			let mut ft_buf = String::new();
			let mut lc_buf = String::new();
			let ft_res = unsafe { ft_printf(format.as_ptr(), arg_01) };
			stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let lc_res = unsafe { printf(format.as_ptr(), arg_01) };
			unsafe { libc::fflush(libc_stdhandle::stdout()) };
			stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
			assert_eq!(
				ft_buf, lc_buf,
				"\n\nft_printf({:?}, {:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_buf,
				"libc:", lc_buf
			);
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_printf({:?}, {:?}) return value differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_res,
				"libc:", lc_res
			);
		}
		#[test]
		fn printf_s_hexu() {
			let format = CString::new("asdf%Xasdf").unwrap();
			let arg_01 = 123;
			let stdout = BufferRedirect::stdout().unwrap();
			let mut ft_buf = String::new();
			let mut lc_buf = String::new();
			let ft_res = unsafe { ft_printf(format.as_ptr(), arg_01) };
			stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let lc_res = unsafe { printf(format.as_ptr(), arg_01) };
			unsafe { libc::fflush(libc_stdhandle::stdout()) };
			stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
			assert_eq!(
				ft_buf, lc_buf,
				"\n\nft_printf({:?}, {:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_buf,
				"libc:", lc_buf
			);
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_printf({:?}, {:?}) return value differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_res,
				"libc:", lc_res
			);
		}
		#[test]
		fn printf_s_string() {
			let format = CString::new("if you read this you are %stupid").unwrap();
			let arg_01 = CString::new("stupid").unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let mut ft_buf = String::new();
			let mut lc_buf = String::new();
			let ft_res = unsafe { ft_printf(format.as_ptr(), arg_01.as_ptr()) };
			stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let lc_res = unsafe { printf(format.as_ptr(), arg_01.as_ptr()) };
			unsafe { libc::fflush(libc_stdhandle::stdout()) };
			stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
			assert_eq!(
				ft_buf, lc_buf,
				"\n\nft_printf({:?}, {:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01.to_string_lossy(),
				"ft:", ft_buf,
				"libc:", lc_buf
			);
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_printf({:?}, {:?}) return value differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01.to_string_lossy(),
				"ft:", ft_res,
				"libc:", lc_res
			);
		}
		#[test]
		fn printf_s_string_null() {
			let format = CString::new("if you read this you are %stupid").unwrap();
			let arg_01 = 0 as *mut libc::c_void;
			let stdout = BufferRedirect::stdout().unwrap();
			let mut ft_buf = String::new();
			let mut lc_buf = String::new();
			let ft_res = unsafe { ft_printf(format.as_ptr(), arg_01) };
			stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let lc_res = unsafe { printf(format.as_ptr(), arg_01) };
			unsafe { libc::fflush(libc_stdhandle::stdout()) };
			stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
			assert_eq!(
				ft_buf, lc_buf,
				"\n\nft_printf({:?}, {:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, "NULL",
				"ft:", ft_buf,
				"libc:", lc_buf
			);
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_printf({:?}, {:?}) return value differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, "NULL",
				"ft:", ft_res,
				"libc:", lc_res
			);
		}
		#[test]
		fn printf_c_test1() {
			let format = CString::new("Hello, %%world!%c%c%s brbr").unwrap();
			let arg_01 = b'6' as libc::c_char;
			let arg_02 = b'9' as libc::c_char;
			let arg_03 = CString::new("yeet despacito lmao").unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let mut ft_buf = String::new();
			let mut lc_buf = String::new();
			let ft_res = unsafe { ft_printf(format.as_ptr(), arg_01 as c_int, arg_02 as c_int, arg_03.as_ptr()) };
			stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let lc_res = unsafe { printf(format.as_ptr(), arg_01 as c_int, arg_02 as c_int, arg_03.as_ptr()) };
			unsafe { libc::fflush(libc_stdhandle::stdout()) };
			stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
			assert_eq!(
				ft_buf, lc_buf,
				"\n\nft_printf({:?}, {:?}, {:?}, {:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01, arg_02, arg_03.to_string_lossy(),
				"ft:", ft_buf,
				"libc:", lc_buf
			);
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_printf({:?}, {:?}, {:?}, {:?}) return value differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01, arg_02, arg_03.to_string_lossy(),
				"ft:", ft_res,
				"libc:", lc_res
			);
		}
		#[test]
		fn printf_c_test2() {
			let format = CString::new("dectest: %d %i %u").unwrap();
			let arg_01 = -69 as libc::c_int;
			let arg_02 = 420 as libc::c_int;
			let arg_03 = 0xFFFFFFFF as libc::c_uint;
			let stdout = BufferRedirect::stdout().unwrap();
			let mut ft_buf = String::new();
			let mut lc_buf = String::new();
			let ft_res = unsafe { ft_printf(format.as_ptr(), arg_01, arg_02, arg_03) };
			stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let lc_res = unsafe { printf(format.as_ptr(), arg_01, arg_02, arg_03) };
			unsafe { libc::fflush(libc_stdhandle::stdout()) };
			stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
			assert_eq!(
				ft_buf, lc_buf,
				"\n\nft_printf({:?}, {:?}, {:?}, {:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01, arg_02, arg_03,
				"ft:", ft_buf,
				"libc:", lc_buf
			);
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_printf({:?}, {:?}, {:?}, {:?}) return value differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01, arg_02, arg_03,
				"ft:", ft_res,
				"libc:", lc_res
			);
		}
		#[test]
		fn printf_c_test3() {
			let format = CString::new("hextest: %x %X").unwrap();
			let arg_01 = 123 as libc::c_int;
			let arg_02 = 456 as libc::c_int;
			let stdout = BufferRedirect::stdout().unwrap();
			let mut ft_buf = String::new();
			let mut lc_buf = String::new();
			let ft_res = unsafe { ft_printf(format.as_ptr(), arg_01, arg_02) };
			stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let lc_res = unsafe { printf(format.as_ptr(), arg_01, arg_02) };
			unsafe { libc::fflush(libc_stdhandle::stdout()) };
			stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
			assert_eq!(
				ft_buf, lc_buf,
				"\n\nft_printf({:?}, {:?}, {:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01, arg_02,
				"ft:", ft_buf,
				"libc:", lc_buf
			);
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_printf({:?}, {:?}, {:?}) return value differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01, arg_02,
				"ft:", ft_res,
				"libc:", lc_res
			);
		}
		#[test]
		fn printf_c_test4() {
			let format = CString::new("hextest: %x %X").unwrap();
			let arg_01 = 123 as libc::c_int;
			let arg_02 = 456 as libc::c_int;
			let stdout = BufferRedirect::stdout().unwrap();
			let mut ft_buf = String::new();
			let mut lc_buf = String::new();
			let ft_res = unsafe { ft_printf(format.as_ptr(), arg_01, arg_02) };
			stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let lc_res = unsafe { printf(format.as_ptr(), arg_01, arg_02) };
			unsafe { libc::fflush(libc_stdhandle::stdout()) };
			stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
			assert_eq!(
				ft_buf, lc_buf,
				"\n\nft_printf({:?}, {:?}, {:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01, arg_02,
				"ft:", ft_buf,
				"libc:", lc_buf
			);
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_printf({:?}, {:?}, {:?}) return value differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01, arg_02,
				"ft:", ft_res,
				"libc:", lc_res
			);
		}
		#[test]
		fn printf_c_test5() {
			let format = CString::new("flagtest: <%4d>").unwrap();
			let arg_01 = 123 as libc::c_int;
			let stdout = BufferRedirect::stdout().unwrap();
			let mut ft_buf = String::new();
			let mut lc_buf = String::new();
			let ft_res = unsafe { ft_printf(format.as_ptr(), arg_01) };
			stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let lc_res = unsafe { printf(format.as_ptr(), arg_01) };
			unsafe { libc::fflush(libc_stdhandle::stdout()) };
			stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
			assert_eq!(
				ft_buf, lc_buf,
				"\n\nft_printf({:?}, {:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_buf,
				"libc:", lc_buf
			);
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_printf({:?}, {:?}) return value differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_res,
				"libc:", lc_res
			);
		}
		#[test]
		fn printf_c_test6() {
			let format = CString::new("flagtest: <%4d>").unwrap();
			let arg_01 = 123456 as libc::c_int;
			let stdout = BufferRedirect::stdout().unwrap();
			let mut ft_buf = String::new();
			let mut lc_buf = String::new();
			let ft_res = unsafe { ft_printf(format.as_ptr(), arg_01) };
			stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let lc_res = unsafe { printf(format.as_ptr(), arg_01) };
			unsafe { libc::fflush(libc_stdhandle::stdout()) };
			stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
			assert_eq!(
				ft_buf, lc_buf,
				"\n\nft_printf({:?}, {:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_buf,
				"libc:", lc_buf
			);
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_printf({:?}, {:?}) return value differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_res,
				"libc:", lc_res
			);
		}
		#[test]
		fn printf_c_test7() {
			let format = CString::new("%p%p%small").unwrap();
			let arg_01 = 420 as libc::c_int;
			let arg_02 = 69 as libc::c_int;
			let arg_03 = CString::new("hello world").unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let mut ft_buf = String::new();
			let mut lc_buf = String::new();
			let ft_res = unsafe { ft_printf(format.as_ptr(), arg_01, arg_02, arg_03.as_ptr()) };
			stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let lc_res = unsafe { printf(format.as_ptr(), arg_01, arg_02, arg_03.as_ptr()) };
			unsafe { libc::fflush(libc_stdhandle::stdout()) };
			stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
			assert_eq!(
				ft_buf, lc_buf,
				"\n\nft_printf({:?}, {:?}, {:?}, {:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01, arg_02, arg_03.to_string_lossy(),
				"ft:", ft_buf,
				"libc:", lc_buf
			);
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_printf({:?}, {:?}, {:?}, {:?}) return value differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01, arg_02, arg_03.to_string_lossy(),
				"ft:", ft_res,
				"libc:", lc_res
			);
		}
		#[test]
		fn printf_s_ptr() {
			let format = CString::new("%p%p%small").unwrap();
			let arg_01 = 420 as libc::c_int;
			let arg_02 = 69 as libc::c_int;
			let stdout = BufferRedirect::stdout().unwrap();
			let mut ft_buf = String::new();
			let mut lc_buf = String::new();
			let ft_res = unsafe { ft_printf(format.as_ptr(), arg_01, arg_02) };
			stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let lc_res = unsafe { printf(format.as_ptr(), arg_01, arg_02) };
			unsafe { libc::fflush(libc_stdhandle::stdout()) };
			stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
			assert_eq!(
				ft_buf, lc_buf,
				"\n\nft_printf({:?}, {:?}, {:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01, arg_02,
				"ft:", ft_buf,
				"libc:", lc_buf
			);
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_printf({:?}, {:?}, {:?}) return value differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01, arg_02,
				"ft:", ft_res,
				"libc:", lc_res
			);
		}
		#[test]
		fn printf_wtflag_1() {
			let format = CString::new("%+-+-  +-+-  06d").unwrap();
			let arg_01 = 123 as libc::c_int;
			let stdout = BufferRedirect::stdout().unwrap();
			let mut ft_buf = String::new();
			let mut lc_buf = String::new();
			let ft_res = unsafe { ft_printf(format.as_ptr(), arg_01) };
			stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let lc_res = unsafe { printf(format.as_ptr(), arg_01) };
			unsafe { libc::fflush(libc_stdhandle::stdout()) };
			stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
			assert_eq!(
				ft_buf, lc_buf,
				"\n\nft_printf({:?}, {:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_buf,
				"libc:", lc_buf
			);
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_printf({:?}, {:?}) return value differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_res,
				"libc:", lc_res
			);
		}
		#[test]
		fn printf_wtflag_2() {
			let format = CString::new("%0 46d").unwrap();
			let arg_01 = 123 as libc::c_int;
			let stdout = BufferRedirect::stdout().unwrap();
			let mut ft_buf = String::new();
			let mut lc_buf = String::new();
			let ft_res = unsafe { ft_printf(format.as_ptr(), arg_01) };
			stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let lc_res = unsafe { printf(format.as_ptr(), arg_01) };
			unsafe { libc::fflush(libc_stdhandle::stdout()) };
			stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
			assert_eq!(
				ft_buf, lc_buf,
				"\n\nft_printf({:?}, {:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_buf,
				"libc:", lc_buf
			);
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_printf({:?}, {:?}) return value differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_res,
				"libc:", lc_res
			);
		}
		#[test]
		fn printf_wtflag_3() {
			let format = CString::new("% 046d").unwrap();
			let arg_01 = 123 as libc::c_int;
			let stdout = BufferRedirect::stdout().unwrap();
			let mut ft_buf = String::new();
			let mut lc_buf = String::new();
			let ft_res = unsafe { ft_printf(format.as_ptr(), arg_01) };
			stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let lc_res = unsafe { printf(format.as_ptr(), arg_01) };
			unsafe { libc::fflush(libc_stdhandle::stdout()) };
			stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
			assert_eq!(
				ft_buf, lc_buf,
				"\n\nft_printf({:?}, {:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_buf,
				"libc:", lc_buf
			);
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_printf({:?}, {:?}) return value differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_res,
				"libc:", lc_res
			);
		}
		#[test]
		fn printf_wtflag_4() {
			let format = CString::new("%+046d").unwrap();
			let arg_01 = 123 as libc::c_int;
			let stdout = BufferRedirect::stdout().unwrap();
			let mut ft_buf = String::new();
			let mut lc_buf = String::new();
			let ft_res = unsafe { ft_printf(format.as_ptr(), arg_01) };
			stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let lc_res = unsafe { printf(format.as_ptr(), arg_01) };
			unsafe { libc::fflush(libc_stdhandle::stdout()) };
			stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
			assert_eq!(
				ft_buf, lc_buf,
				"\n\nft_printf({:?}, {:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_buf,
				"libc:", lc_buf
			);
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_printf({:?}, {:?}) return value differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_res,
				"libc:", lc_res
			);
		}
		#[test]
		fn printf_wtflag_5() {
			let format = CString::new("%+ 46d").unwrap();
			let arg_01 = 123 as libc::c_int;
			let stdout = BufferRedirect::stdout().unwrap();
			let mut ft_buf = String::new();
			let mut lc_buf = String::new();
			let ft_res = unsafe { ft_printf(format.as_ptr(), arg_01) };
			stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let lc_res = unsafe { printf(format.as_ptr(), arg_01) };
			unsafe { libc::fflush(libc_stdhandle::stdout()) };
			stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
			assert_eq!(
				ft_buf, lc_buf,
				"\n\nft_printf({:?}, {:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_buf,
				"libc:", lc_buf
			);
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_printf({:?}, {:?}) return value differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_res,
				"libc:", lc_res
			);
		}
		#[test]
		fn printf_wtflag_6() {
			let format = CString::new("% +46d").unwrap();
			let arg_01 = 123 as libc::c_int;
			let stdout = BufferRedirect::stdout().unwrap();
			let mut ft_buf = String::new();
			let mut lc_buf = String::new();
			let ft_res = unsafe { ft_printf(format.as_ptr(), arg_01) };
			stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let lc_res = unsafe { printf(format.as_ptr(), arg_01) };
			unsafe { libc::fflush(libc_stdhandle::stdout()) };
			stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
			assert_eq!(
				ft_buf, lc_buf,
				"\n\nft_printf({:?}, {:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_buf,
				"libc:", lc_buf
			);
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_printf({:?}, {:?}) return value differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_res,
				"libc:", lc_res
			);
		}
		#[test]
		fn printf_flag_plus() {
			let format = CString::new("%+d").unwrap();
			let arg_01 = 123 as libc::c_int;
			let stdout = BufferRedirect::stdout().unwrap();
			let mut ft_buf = String::new();
			let mut lc_buf = String::new();
			let ft_res = unsafe { ft_printf(format.as_ptr(), arg_01) };
			stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let lc_res = unsafe { printf(format.as_ptr(), arg_01) };
			unsafe { libc::fflush(libc_stdhandle::stdout()) };
			stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
			assert_eq!(
				ft_buf, lc_buf,
				"\n\nft_printf({:?}, {:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_buf,
				"libc:", lc_buf
			);
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_printf({:?}, {:?}) return value differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_res,
				"libc:", lc_res
			);
		}
		#[test]
		fn printf_flag_plus_width() {
			let format = CString::new("%+46d").unwrap();
			let arg_01 = 123 as libc::c_int;
			let stdout = BufferRedirect::stdout().unwrap();
			let mut ft_buf = String::new();
			let mut lc_buf = String::new();
			let ft_res = unsafe { ft_printf(format.as_ptr(), arg_01) };
			stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let lc_res = unsafe { printf(format.as_ptr(), arg_01) };
			unsafe { libc::fflush(libc_stdhandle::stdout()) };
			stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
			assert_eq!(
				ft_buf, lc_buf,
				"\n\nft_printf({:?}, {:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_buf,
				"libc:", lc_buf
			);
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_printf({:?}, {:?}) return value differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_res,
				"libc:", lc_res
			);
		}
		#[test]
		fn printf_s_none() {
			let format = CString::new("%").unwrap();
			let arg_01 = 123 as libc::c_int;
			let stdout = BufferRedirect::stdout().unwrap();
			let mut ft_buf = String::new();
			let mut lc_buf = String::new();
			let ft_res = unsafe { ft_printf(format.as_ptr(), arg_01) };
			stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
			let stdout = BufferRedirect::stdout().unwrap();
			let lc_res = unsafe { printf(format.as_ptr(), arg_01) };
			unsafe { libc::fflush(libc_stdhandle::stdout()) };
			stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
			assert_eq!(
				ft_buf, lc_buf,
				"\n\nft_printf({:?}, {:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_buf,
				"libc:", lc_buf
			);
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_printf({:?}, {:?}) return value differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				format, arg_01,
				"ft:", ft_res,
				"libc:", lc_res
			);
		}

		#[test]
		#[should_panic]
		fn test_printf_segfault() {
			unsafe { ft_printf(0 as *const c_char ) };
		}
	}
}