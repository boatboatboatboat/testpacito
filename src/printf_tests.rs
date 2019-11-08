use libc::c_char;
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
		fn test_basic_1() {
			let expected_output = "hello, world!";
			let expected_res = expected_output.len();
			let tester = CString::new("hello, world!").unwrap();
			let prebuf = BufferRedirect::stdout().unwrap();
			let res = unsafe {ft_printf(tester.as_ptr())};
			let mut prebuf_res = String::new();
			prebuf.into_inner().read_to_string(&mut prebuf_res).unwrap();
			assert_eq!(
				prebuf_res, expected_output,
				"\n\nft_printf({:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				tester, "yours:", prebuf_res, "libc:", expected_output
			);
			assert_eq!(
				res, expected_res as i32,
				"\n\nft_printf({:?}) return value differs:\n{: <10}{}\n{: <10}{}\n\n",
				tester, "yours:", res, "libc:", expected_res
			);
		}
		#[test]
		fn test_char_specifier() {
			let expected_output = "hello world";
			let expected_res = expected_output.len();
			let tester = CString::new("hello%cworld").unwrap();
			let prebuf = BufferRedirect::stdout().unwrap();
			let res = unsafe {ft_printf(tester.as_ptr(), b' ' as libc::c_uint)};
			let mut prebuf_res = String::new();
			prebuf.into_inner().read_to_string(&mut prebuf_res).unwrap();
			assert_eq!(
				prebuf_res, expected_output,
				"\n\nft_printf({:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				tester, "yours:", prebuf_res, "libc:", expected_output
			);
			assert_eq!(
				res, expected_res as i32,
				"\n\nft_printf({:?}) return value differs:\n{: <10}{}\n{: <10}{}\n\n",
				tester, "yours:", res, "libc:", expected_res
			);
		}
		#[test]
		fn test_escape_specifier() {
			let expected_output = "hello%world";
			let expected_res = expected_output.len();
			let tester = CString::new("hello%%world").unwrap();
			let prebuf = BufferRedirect::stdout().unwrap();
			let res = unsafe {ft_printf(tester.as_ptr())};
			let mut prebuf_res = String::new();
			prebuf.into_inner().read_to_string(&mut prebuf_res).unwrap();
			assert_eq!(
				prebuf_res, expected_output,
				"\n\nft_printf({:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				tester, "yours:", prebuf_res, "libc:", expected_output
			);
			assert_eq!(
				res, expected_res as i32,
				"\n\nft_printf({:?}) return value differs:\n{: <10}{}\n{: <10}{}\n\n",
				tester, "yours:", res, "libc:", expected_res
			);
		}
		#[test]
		fn test_intd_specifier() {
			let expected_output = "123espacito";
			let expected_res = expected_output.len();
			let tester = CString::new("%despacito").unwrap();
			let prebuf = BufferRedirect::stdout().unwrap();
			let res = unsafe {ft_printf(tester.as_ptr(), 123 as libc::c_int)};
			let mut prebuf_res = String::new();
			prebuf.into_inner().read_to_string(&mut prebuf_res).unwrap();
			assert_eq!(
				prebuf_res, expected_output,
				"\n\nft_printf({:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				tester, "yours:", prebuf_res, "libc:", expected_output
			);
			assert_eq!(
				res, expected_res as i32,
				"\n\nft_printf({:?}) return value differs:\n{: <10}{}\n{: <10}{}\n\n",
				tester, "yours:", res, "libc:", expected_res
			);
		}
		#[test]
		fn test_inti_specifier() {
			let expected_output = "hi, 123456 is a number";
			let expected_res = expected_output.len();
			let tester = CString::new("hi, %i is a number").unwrap();
			let prebuf = BufferRedirect::stdout().unwrap();
			let res = unsafe {ft_printf(tester.as_ptr(), 123456 as libc::c_int)};
			let mut prebuf_res = String::new();
			prebuf.into_inner().read_to_string(&mut prebuf_res).unwrap();
			assert_eq!(
				prebuf_res, expected_output,
				"\n\nft_printf({:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				tester, "yours:", prebuf_res, "libc:", expected_output
			);
			assert_eq!(
				res, expected_res as i32,
				"\n\nft_printf({:?}) return value differs:\n{: <10}{}\n{: <10}{}\n\n",
				tester, "yours:", res, "libc:", expected_res
			);
		}
		#[test]
		fn test_hex_specifier() {
			let expected_output = "hi, ff is a number";
			let expected_res = expected_output.len();
			let tester = CString::new("hi, %x is a number").unwrap();
			let prebuf = BufferRedirect::stdout().unwrap();
			let res = unsafe {ft_printf(tester.as_ptr(), 255 as libc::c_int)};
			let mut prebuf_res = String::new();
			prebuf.into_inner().read_to_string(&mut prebuf_res).unwrap();
			assert_eq!(
				prebuf_res, expected_output,
				"\n\nft_printf({:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				tester, "yours:", prebuf_res, "libc:", expected_output
			);
			assert_eq!(
				res, expected_res as i32,
				"\n\nft_printf({:?}) return value differs:\n{: <10}{}\n{: <10}{}\n\n",
				tester, "yours:", res, "libc:", expected_res
			);
		}
		#[test]
		fn test_hexu_specifier() {
			let expected_output = "hi, FF is a number";
			let expected_res = expected_output.len();
			let tester = CString::new("hi, %X is a number").unwrap();
			let prebuf = BufferRedirect::stdout().unwrap();
			let res = unsafe {ft_printf(tester.as_ptr(), 255 as libc::c_int)};
			let mut prebuf_res = String::new();
			prebuf.into_inner().read_to_string(&mut prebuf_res).unwrap();
			assert_eq!(
				prebuf_res, expected_output,
				"\n\nft_printf({:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				tester, "yours:", prebuf_res, "libc:", expected_output
			);
			assert_eq!(
				res, expected_res as i32,
				"\n\nft_printf({:?}) return value differs:\n{: <10}{}\n{: <10}{}\n\n",
				tester, "yours:", res, "libc:", expected_res
			);
		}
		#[test]
		fn test_string_specifier() {
			let expected_output = "if you read this you are stupid";
			let expected_res = expected_output.len();
			let tester = CString::new("if you read this you are %s").unwrap();
			let prebuf = BufferRedirect::stdout().unwrap();
			let res = unsafe {ft_printf(tester.as_ptr(), CString::new("stupid").unwrap().as_ptr() as *const c_char)};
			let mut prebuf_res = String::new();
			prebuf.into_inner().read_to_string(&mut prebuf_res).unwrap();
			assert_eq!(
				prebuf_res, expected_output,
				"\n\nft_printf({:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				tester, "yours:", prebuf_res, "libc:", expected_output
			);
			assert_eq!(
				res, expected_res as i32,
				"\n\nft_printf({:?}) return value differs:\n{: <10}{}\n{: <10}{}\n\n",
				tester, "yours:", res, "libc:", expected_res
			);
		}
		#[test]
		fn test_ctests_1() {
			let expected_output = "Hello, %world!69yeet despacito lmao brbr";
			let expected_res = expected_output.len();
			let tester = CString::new("Hello, %%world!%c%c%s brbr").unwrap();
			let prebuf = BufferRedirect::stdout().unwrap();
			let res = unsafe {ft_printf(
				tester.as_ptr(),
				b'6' as *const libc::c_char,
				b'9' as *const libc::c_char,
				CString::new("yeet despacito lmao").unwrap().as_ptr() as *const c_char
			)};
			let mut prebuf_res = String::new();
			prebuf.into_inner().read_to_string(&mut prebuf_res).unwrap();
			assert_eq!(
				prebuf_res, expected_output,
				"\n\nft_printf({:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				tester, "yours:", prebuf_res, "libc:", expected_output
			);
			assert_eq!(
				res, expected_res as i32,
				"\n\nft_printf({:?}) return value differs:\n{: <10}{}\n{: <10}{}\n\n",
				tester, "yours:", res, "libc:", expected_res
			);
		}
		
		#[test]
		fn test_ctests_2() {
			let expected_output = "dectest: -69 420 4294967295";
			let expected_res = expected_output.len();
			let tester = CString::new("dectest: %d %i %u").unwrap();
			let prebuf = BufferRedirect::stdout().unwrap();
			let res = unsafe {ft_printf(
				tester.as_ptr(),
				-69 as libc::c_int,
				420 as libc::c_int,
				0xFFFFFFFF as libc::c_uint
			)};
			let mut prebuf_res = String::new();
			prebuf.into_inner().read_to_string(&mut prebuf_res).unwrap();
			assert_eq!(
				prebuf_res, expected_output,
				"\n\nft_printf({:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				tester, "yours:", prebuf_res, "libc:", expected_output
			);
			assert_eq!(
				res, expected_res as i32,
				"\n\nft_printf({:?}) return value differs:\n{: <10}{}\n{: <10}{}\n\n",
				tester, "yours:", res, "libc:", expected_res
			);
		}
		#[test]
		fn test_ctests_3() {
			let expected_output = "hextest: 7b 1C8";
			let expected_res = expected_output.len();
			let tester = CString::new("hextest: %x %X").unwrap();
			let prebuf = BufferRedirect::stdout().unwrap();
			let res = unsafe {ft_printf(
				tester.as_ptr(),
				123 as libc::c_uint,
				456 as libc::c_uint,
			)};
			let mut prebuf_res = String::new();
			prebuf.into_inner().read_to_string(&mut prebuf_res).unwrap();
			assert_eq!(
				prebuf_res, expected_output,
				"\n\nft_printf({:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				tester, "yours:", prebuf_res, "libc:", expected_output
			);
			assert_eq!(
				res, expected_res as i32,
				"\n\nft_printf({:?}) return value differs:\n{: <10}{}\n{: <10}{}\n\n",
				tester, "yours:", res, "libc:", expected_res
			);
		}
		#[test]
		fn test_ctests_4() {
			let expected_output = "flagtest: 23456789";
			let expected_res = expected_output.len();
			let tester = CString::new("flagtest: <%4d>").unwrap();
			let prebuf = BufferRedirect::stdout().unwrap();
			let res = unsafe {ft_printf(
				tester.as_ptr(),
				23456789 as libc::c_uint,
			)};
			let mut prebuf_res = String::new();
			prebuf.into_inner().read_to_string(&mut prebuf_res).unwrap();
			assert_eq!(
				prebuf_res, expected_output,
				"\n\nft_printf({:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				tester, "yours:", prebuf_res, "libc:", expected_output
			);
			assert_eq!(
				res, expected_res as i32,
				"\n\nft_printf({:?}) return value differs:\n{: <10}{}\n{: <10}{}\n\n",
				tester, "yours:", res, "libc:", expected_res
			);
		}
		#[test]
		fn test_ctests_5() {
			let expected_output = "flagtest: <  12>";
			let expected_res = expected_output.len();
			let tester = CString::new("flagtest: <%4d>").unwrap();
			let prebuf = BufferRedirect::stdout().unwrap();
			let res = unsafe {ft_printf(
				tester.as_ptr(),
				12 as libc::c_uint,
			)};
			let mut prebuf_res = String::new();
			prebuf.into_inner().read_to_string(&mut prebuf_res).unwrap();
			assert_eq!(
				prebuf_res, expected_output,
				"\n\nft_printf({:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				tester, "yours:", prebuf_res, "libc:", expected_output
			);
			assert_eq!(
				res, expected_res as i32,
				"\n\nft_printf({:?}) return value differs:\n{: <10}{}\n{: <10}{}\n\n",
				tester, "yours:", res, "libc:", expected_res
			);
		}
		#[test]
		fn padding_1() {
			let expected_output = "<001234>";
			let expected_res = expected_output.len();
			let tester = CString::new("<%6d>").unwrap();
			let prebuf = BufferRedirect::stdout().unwrap();
			let res = unsafe {ft_printf(
				tester.as_ptr(),
				1234 as libc::c_uint,
			)};
			let mut prebuf_res = String::new();
			prebuf.into_inner().read_to_string(&mut prebuf_res).unwrap();
			assert_eq!(
				prebuf_res, expected_output,
				"\n\nft_printf({:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				tester, "yours:", prebuf_res, "libc:", expected_output
			);
			assert_eq!(
				res, expected_res as i32,
				"\n\nft_printf({:?}) return value differs:\n{: <10}{}\n{: <10}{}\n\n",
				tester, "yours:", res, "libc:", expected_res
			);
		}
		#[test]
		fn test_null_str() {
			let expected_output = "hello (null)";
			let expected_res = expected_output.len();
			let tester = CString::new("hello %s").unwrap();
			let prebuf = BufferRedirect::stdout().unwrap();
			let res = unsafe {ft_printf(
				tester.as_ptr(),
				0 as *const c_char
			)};
			let mut prebuf_res = String::new();
			prebuf.into_inner().read_to_string(&mut prebuf_res).unwrap();
			assert_eq!(
				prebuf_res, expected_output,
				"\n\nft_printf({:?}) output differs:\n{: <10}{:?}\n{: <10}{:?}\n\n",
				tester, "yours:", prebuf_res, "libc:", expected_output
			);
			assert_eq!(
				res, expected_res as i32,
				"\n\nft_printf({:?}) return value differs:\n{: <10}{}\n{: <10}{}\n\n",
				tester, "yours:", res, "libc:", expected_res
			);
		}
	}
}