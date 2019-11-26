use libc::{size_t, c_char, c_void, c_int};
use std::ffi::CString;

#[link(name = "ft")]
extern {
	fn ft_atoi(str: *const c_char) -> i32;
	fn atoi(str: *const c_char) -> i32;
	fn ft_bzero(s: *mut c_void, n: size_t);
	fn ft_calloc(count: size_t, size: size_t) -> *mut c_void;
	fn ft_strmapi(s: *const c_char, f: extern fn(u32, c_char) -> c_char) -> *mut c_char;
	fn ft_split(s: *const c_char, c: c_char) -> *mut *mut c_char;
	fn ft_memset(s: *mut c_void, c: i32, l: size_t) -> *mut c_void;
	fn memset(s: *mut c_void, c: i32, l: size_t) -> *mut c_void;
	fn ft_strrchr(s: *const c_char, c: i32) -> *mut c_char;
	fn ft_strchr(s: *const c_char, c: i32) -> *mut c_char;
	fn ft_strnstr(haystack: *const c_char, needle: *const c_char, len: size_t) -> *mut c_char;
	fn strnstr(haystack: *const c_char, needle: *const c_char, len: size_t) -> *mut c_char;
	fn strchr(s: *const c_char, c: i32) -> *mut c_char;
	fn ft_strjoin(lhs: *const c_char, rhs: *const c_char) -> *mut c_char;
	fn ft_substr(input: *const c_char, start: u32, len: usize) -> *mut c_char;
	fn ft_strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
	fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
	fn ft_memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
	fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
	fn ft_strtrim(s1: *const c_char, set: *const c_char) -> *mut c_char;
	fn ft_strlen(s: *const c_char) -> size_t;
	fn ft_putchar_fd(c: c_char, fd: c_int);
	fn ft_putendl_fd(s: *const c_char, fd: c_int); // actually *mut c_char
	fn ft_putstr_fd(s: *const c_char, fd: c_int); // actually *mut c_char
	fn ft_putnbr_fd(n: c_int, fd: c_int);
	fn ft_tolower(chr: i32) -> i32;
	fn ft_toupper(chr: i32) -> i32;
	fn ft_isalnum(chr: i32) -> i32;
	fn ft_isalpha(chr: i32) -> i32;
	fn ft_isdigit(chr: i32) -> i32;
	fn ft_isascii(chr: i32) -> i32;
	fn ft_isprint(chr: i32) -> i32;
	fn ft_itoa(n: i32) -> *mut c_char;
	fn ft_memchr(s: *const c_void, c: c_int, n: size_t) -> *mut c_void;
	fn memchr(s: *const c_void, c: c_int, n: size_t) -> *mut c_void;
}

pub mod ft_tests {
	use super::*;

	#[cfg(test)]
	mod ft_atoi {
		use super::*;
		fn atoi_tester<'a>(input: &'a str) {
			let tester = CString::new(input).unwrap();
			let ft_res = unsafe { ft_atoi(tester.as_ptr()) };
			let lc_res = unsafe { atoi(tester.as_ptr()) };
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_atoi({:?}) result differs:\nlibft: {:?}\n libc: {:?}\n\n",
				input, ft_res, lc_res
			);
		}
		rusty_fork_test! {
			#[test]
			fn atoi_basic_1() {
				atoi_tester("123456");
			}
			#[test]
			fn atoi_basic_2() {
				atoi_tester("42069");
			}
			#[test]
			fn atoi_extremes_1() {
				atoi_tester(i32::max_value().to_string().as_str());
			}
			#[test]
			fn atoi_extremes_2() {
				atoi_tester(i32::min_value().to_string().as_str());
			}
			#[test]
			fn test_extremes_3() {
				atoi_tester(i64::min_value().to_string().as_str());
			}
			#[test]
			fn test_extremes_4() {
				atoi_tester(i64::max_value().to_string().as_str());
			}
			#[test]
			fn test_skipping_1() {
				atoi_tester("+123");
			}
			#[test]
			fn test_skipping_2() {
				atoi_tester("+-123");
			}
			#[test]
			fn test_skipping_3() {
				atoi_tester("       +123 123");
			}
			#[test]
			fn test_skipping_4() {
				atoi_tester("\n-42069");
			}
			#[test]
			fn test_skipping_5() {
				atoi_tester("\t\n\r\x0b\x0c 987");
			}
			
			#[test]
			#[should_panic]
			fn test_null() {
				unsafe { ft_atoi(0 as *const i8) };
			}
		}
	}

	mod ft_bzero {
		use super::*;
		#[test]
		fn test_basic_1() {
			let mut real: Box<[c_char; 10]> = Box::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
			let compare: [c_char; 10] = [0, 0, 0, 0, 0, 0, 7, 8, 9, 10];
			unsafe { ft_bzero(real.as_mut_ptr() as *mut c_void, 6) };
			assert!(
				real.iter().zip(compare.iter()).all(|(a, b)| a==b),
				"\n\ngiven a char A[10] = {{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}},\n\
				using ft_bzero(A, 6)\n\
				your bzero turns it into\n{:?}\ninstead of\n\
				{:?}\n\n",
				real, compare
			)
		}
		#[test]
		fn test_basic_2() {
			let mut real: Box<[c_char; 10]> = Box::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
			let compare: [c_char; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
			unsafe { ft_bzero(real.as_mut_ptr() as *mut c_void, 0) };
			assert!(
				real.iter().zip(compare.iter()).all(|(a, b)| a==b),
				"\n\ngiven a char A[10] = {{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}},\n\
				using ft_bzero(A, 0)\n\
				your bzero turns it into\n{:?}\ninstead of\n\
				{:?}\n\n",
				real, compare
			)
		}
		rusty_fork_test! {
			#[test]
			#[should_panic]
			fn test_null() {
				unsafe { ft_bzero(0 as *mut c_void, 420); }
			}
		}
	}
	
	mod ft_memset {
		use super::*;
		rusty_fork_test! {
			#[test]
			fn memset_basic_1() {
				let mut real: Box<[c_char; 10]> = Box::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
				let mut compare: Box<[c_char; 10]> = Box::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
				unsafe { memset(compare.as_mut_ptr() as *mut c_void, 6, 9) };
				let pres = unsafe { ft_memset(real.as_mut_ptr() as *mut c_void, 6, 9) };
				assert!(
					real.iter().zip(compare.iter()).all(|(a, b)| a==b),
					"\n\ngiven a char A[10] = {{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}},\n\
					using ft_memset(A, 6, 9)\n\
					your memset turns it into\n{:?}\ninstead of\n\
					{:?}\n\n",
					real, compare
				);
				assert_eq!(
					real.as_mut_ptr(), pres as *mut i8,
					"your ft_memset does not return dest"
				);
			}
			#[test]
			fn memset_basic_2() {
				let mut real: Box<[c_char; 10]> = Box::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
				let mut compare: Box<[c_char; 10]> = Box::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
				unsafe { libc::memset(compare.as_mut_ptr() as *mut c_void, 42, 0) };
				let pres = unsafe { ft_memset(real.as_mut_ptr() as *mut c_void, 42, 0) };
				assert!(
					real.iter().zip(compare.iter()).all(|(a, b)| a == b),
					"\n\ngiven a char A[10] = {{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}},\n\
					using ft_memset(A, 42, 0)\n\
					your bzero turns it into\n{:?}\ninstead of\n\
					{:?}\n\n",
					real, compare
				);
				assert_eq!(
					real.as_mut_ptr(), pres as *mut i8,
					"your ft_memset does not return dest"
				);
			}
			#[test]
			#[should_panic]
			fn memset_null() {
				unsafe { ft_memset(0 as *mut c_void, 69, 1); }
			}
		}
	}

	#[cfg(test)]
	mod ft_calloc {
		use super::*;
		#[test]
		fn test_basic_1() {
			let res = unsafe { ft_calloc(2, 2) };
			assert_eq!(
				res.is_null(), false,
				"ft_calloc(2, 2) returned null"
			);
		}
		#[test]
		fn test_basic_2() {
			let res = unsafe { ft_calloc(usize::max_value(), 1) };
			assert_eq!(
				res.is_null(), true,
				"ft_calloc(LONG_MAX, 1) returned a non-null value"
			)
		}
	}
	
	#[cfg(test)]
	mod ft_ctype {
		use super::*;
		#[test]
		fn test_isalpha() {
			for i in -1000..1000 {
				let ft_res = unsafe { ft_isalpha(i) };
				let lc_res = unsafe { libc::isalpha(i) };
				assert_eq!(
					lc_res, ft_res,
					"ft_isalpha({}) returned the wrong value (lc, ft)",
					i
				)
			}
		}
		#[test]
		fn test_isalnum() {
			for i in -1000..1000 {
				let ft_res = unsafe { ft_isalnum(i) };
				let lc_res = unsafe { libc::isalnum(i) };
				assert_eq!(
					lc_res, ft_res,
					"ft_isalnum({}) returned the wrong value (lc, ft)",
					i
				)
			}
		}
		#[test]
		fn test_isdigit() {
			for i in -1000..1000 {
				let ft_res = unsafe { ft_isdigit(i) };
				let lc_res = unsafe { libc::isdigit(i) };
				assert_eq!(
					lc_res, ft_res,
					"ft_isdigit({}) returned the wrong value (lc, ft)",
					i
				)
			}
		}
		#[test]
		fn test_isascii() {
			fn comparator(chr: i32) -> bool {
				chr >= 0x0 && chr <= 0x7f
			}
			for i in -1000..1000 {
				let res = unsafe { ft_isascii(i) };
				assert_eq!(
					res != 0, comparator(i),
					"ft_isascii({}) returned the wrong value",
					i
				)
			}
		}
		#[test]
		fn test_isprint() {
			for i in -1000..1000 {
				let ft_res = unsafe { ft_isprint(i) };
				let lc_res = unsafe { libc::isprint(i) };
				assert_eq!(
					lc_res, ft_res,
					"ft_isprint({}) returned the wrong value (lc, ft)",
					i
				)
			}
		}
		#[test]
		fn test_toupper() {
			for i in -1000..1000 {
				let ft_res = unsafe { ft_toupper(i) };
				let lc_res = unsafe { libc::toupper(i) };
				assert_eq!(
					lc_res, ft_res,
					"ft_toupper({}) returned the wrong value (lc, ft)",
					i
				)
			}
		}
		#[test]
		fn test_tolower() {
			for i in -1000..1000 {
				let ft_res = unsafe { ft_tolower(i) };
				let lc_res = unsafe { libc::tolower(i) };
				assert_eq!(
					lc_res, ft_res,
					"ft_toupper({}) returned the wrong value (lc, ft)",
					i
				)
			}
		}
	}
	
	#[cfg(test)]
	mod ft_strlen {
		use super::*;
		fn strlen_tester(input: &str) {
			let tester = CString::new(input).unwrap();
			let ft_res = unsafe { ft_strlen(tester.as_ptr()) };
			let lc_res = unsafe { libc::strlen(tester.as_ptr()) };
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_strlen({:?}) result differs:\nlibft: {:?}\n libc: {:?}\n\n",
				input, ft_res, lc_res
			);
		}
		rusty_fork_test! {
			#[test]
			fn strlen_empty_str() {
				strlen_tester("");
			}
			#[test]
			fn strlen_test1() {
				strlen_tester("a");
			}
			#[test]
			fn strlen_test2() {
				strlen_tester("ab");
			}
			#[test]
			fn strlen_test3() {
				strlen_tester("dfnalksdjflkajsdklfjaslkdfjloeiuwroiwu");
			}
		}
	}
	#[cfg(test)]
	mod ft_substr {
		use super::*;
		fn substr_tester(input: &str, start: u32, len: usize) {
			let tester = CString::new(input).unwrap();
			let ft_res = unsafe { CString::from_raw(ft_substr(tester.as_ptr(), start, len)).to_owned() };
			let expected: String = input.chars().skip(start as usize).take(len).collect();
			assert_eq!(
				ft_res.to_string_lossy(), expected,
				"\n\nft_substr({:?}, {:?}, {:?}) result differs\nyours: {:?}\nmouli: {:?}\n\n",
				input, start, len,
				ft_res.to_string_lossy(), expected
			);
		}
		rusty_fork_test! {
			#[test]
			fn substr_test_1() {
				substr_tester("hello world", 3, 6);
			}
			#[test]
			fn substr_test_2() {
				substr_tester("qw;oeejqweoqpwoweipoqweiopqiwe", 6, 15);
			}
			#[test]
			fn substr_test_3() {
				substr_tester("hello world", 1, 9);
			}
			#[test]
			fn substr_rangetest_1() {
				for start in 0..30 {
					for len in 0..30 {
						substr_tester("despayeeto", start, len);
					}
				}
			}
			#[test]
			fn substr_rangetest_2() {
				for start in 0..100 {
					for len in 0..100 {
						println!("start: {}, len: {}", start, len);
						substr_tester("de3123123123123125555spayaeedfasdfasdfasdfto", start, len);
					}
				}
			}
		}
	}
	#[cfg(test)]
	mod ft_put_fd {
		use super::*;
		use std::io::Read;
		use gag::BufferRedirect;
		rusty_fork_test! {
			#[test]
			fn test_putchar_fd() {
				let mut ft_res = String::new();
				let mut lc_res = String::new();
				for i in 0..=127 {
					let prebuf = BufferRedirect::stdout().unwrap();
					unsafe { ft_putchar_fd(i, 1) };
					prebuf.into_inner().read_to_string(&mut ft_res).unwrap();
					let prebuf = BufferRedirect::stdout().unwrap();
					print!("{}", i as u8 as char);
					use std::io::Write;
					std::io::stdout().flush().unwrap();
					prebuf.into_inner().read_to_string(&mut lc_res).unwrap();
					assert_eq!(
						ft_res, lc_res,
						"ft_putchar_fd({}, {}) result differs (ft, lc)",
						i, 1
					);
					ft_res.clear();
					lc_res.clear();
				}
			}
			#[test]
			fn test_putchar_fd_err() {
				let mut ft_res = String::new();
				let mut lc_res = String::new();
				for i in 0..=127 {
					let prebuf = BufferRedirect::stderr().unwrap();
					unsafe { ft_putchar_fd(i, 2) };
					prebuf.into_inner().read_to_string(&mut ft_res).unwrap();
					let prebuf = BufferRedirect::stderr().unwrap();
					eprint!("{}", i as u8 as char);
					use std::io::Write;
					std::io::stderr().flush().unwrap();
					prebuf.into_inner().read_to_string(&mut lc_res).unwrap();
					assert_eq!(
						ft_res, lc_res,
						"ft_putchar_fd({}, {}) result differs (ft, lc)",
						i, 2
					);
					ft_res.clear();
					lc_res.clear();
				}
			}
			#[test]
			fn test_putnbr_fd() {
				let mut ft_res = String::new();
				let mut lc_res = String::new();
				for i in -100..100 {
					let prebuf = BufferRedirect::stdout().unwrap();
					unsafe { ft_putnbr_fd(i, 1) };
					prebuf.into_inner().read_to_string(&mut ft_res).unwrap();
					let prebuf = BufferRedirect::stdout().unwrap();
					print!("{}", i);
					use std::io::Write;
					std::io::stdout().flush().unwrap();
					prebuf.into_inner().read_to_string(&mut lc_res).unwrap();
					assert_eq!(
						ft_res, lc_res,
						"ft_putnbr_fd({}, {}) result differs (ft, lc)",
						i, 1
					);
					ft_res.clear();
					lc_res.clear();
				}
			}
			#[test]
			fn test_putnbr_fd_err() {
				let mut ft_res = String::new();
				let mut lc_res = String::new();
				for i in -100..100 {
					let prebuf = BufferRedirect::stderr().unwrap();
					unsafe { ft_putnbr_fd(i, 2) };
					prebuf.into_inner().read_to_string(&mut ft_res).unwrap();
					let prebuf = BufferRedirect::stderr().unwrap();
					eprint!("{}", i);
					use std::io::Write;
					std::io::stderr().flush().unwrap();
					prebuf.into_inner().read_to_string(&mut lc_res).unwrap();
					assert_eq!(
						ft_res, lc_res,
						"ft_putnbr_fd({}, {}) result differs (ft, lc)",
						i, 2
					);
					ft_res.clear();
					lc_res.clear();
				}
			}
			#[test]
			fn test_putnbr_fd_min() {
				let mut ft_res = String::new();
				let mut lc_res = String::new();
				let i = i32::min_value();
				{
					let prebuf = BufferRedirect::stdout().unwrap();
					unsafe { ft_putnbr_fd(i, 1) };
					prebuf.into_inner().read_to_string(&mut ft_res).unwrap();
					let prebuf = BufferRedirect::stdout().unwrap();
					print!("{}", i);
					use std::io::Write;
					std::io::stdout().flush().unwrap();
					prebuf.into_inner().read_to_string(&mut lc_res).unwrap();
					assert_eq!(
						ft_res, lc_res,
						"ft_putnbr_fd({}, {}) result differs (ft, lc)",
						i, 1
					);
					ft_res.clear();
					lc_res.clear();
				}
			}
			#[test]
			fn test_putnbr_fd_max() {
				let mut ft_res = String::new();
				let mut lc_res = String::new();
				let i = i32::max_value();
				{
					let prebuf = BufferRedirect::stdout().unwrap();
					unsafe { ft_putnbr_fd(i, 1) };
					prebuf.into_inner().read_to_string(&mut ft_res).unwrap();
					let prebuf = BufferRedirect::stdout().unwrap();
					print!("{}", i);
					use std::io::Write;
					std::io::stdout().flush().unwrap();
					prebuf.into_inner().read_to_string(&mut lc_res).unwrap();
					assert_eq!(
						ft_res, lc_res,
						"ft_putnbr_fd({}, {}) result differs (ft, lc)",
						i, 1
					);
					ft_res.clear();
					lc_res.clear();
				}
			}
			#[test]
			fn test_putstr_fd() {
				let mut ft_res = String::new();
				let input = "hello, world!";
				let prebuf = BufferRedirect::stdout().unwrap();
				unsafe { ft_putstr_fd(CString::new(input).unwrap().as_ptr(), 1) };
				prebuf.into_inner().read_to_string(&mut ft_res).unwrap();
				assert_eq!(
					ft_res, input,
					"ft_putstr_fd({}, {}) result differs (ft, lc)",
					input, 1
				);
			}
			#[test]
			fn test_putstr_fd_err() {
				let mut ft_res = String::new();
				let input = "hello, world!";
				let prebuf = BufferRedirect::stderr().unwrap();
				unsafe { ft_putstr_fd(CString::new(input).unwrap().as_ptr(), 2) };
				prebuf.into_inner().read_to_string(&mut ft_res).unwrap();
				assert_eq!(
					ft_res, input,
					"ft_putstr_fd({}, {}) result differs (ft, lc)",
					input, 2
				);
			}
			#[test]
			fn test_putstr_fd_2() {
				let mut ft_res = String::new();
				let input = "";
				let prebuf = BufferRedirect::stdout().unwrap();
				unsafe { ft_putstr_fd(CString::new(input).unwrap().as_ptr(), 1) };
				prebuf.into_inner().read_to_string(&mut ft_res).unwrap();
				assert_eq!(
					ft_res, input,
					"ft_putstr_fd({}, {}) result differs (ft, lc)",
					input, 1
				);
			}
			#[test]
			#[ignore]
			fn opt_test_putstr_fd() {
				/* this is optional */
				unsafe { ft_putstr_fd(0 as *const c_char, 1) };
			}
			#[test]
			fn test_putendl_fd() {
				let mut ft_res = String::new();
				let input = String::from("hello, world!");
				let prebuf = BufferRedirect::stdout().unwrap();
				unsafe { ft_putendl_fd(CString::new(input.clone()).unwrap().as_ptr(), 1) };
				prebuf.into_inner().read_to_string(&mut ft_res).unwrap();
				let mut comparator = input.clone();
				comparator.push('\n');
				assert_eq!(
					ft_res, comparator,
					"ft_putendl_fd({}, {}) result differs (ft, lc)",
					input, 1
				);
			}
			#[test]
			fn test_putendl_fd_err() {
				let mut ft_res = String::new();
				let input = String::from("hello, world!");
				let prebuf = BufferRedirect::stderr().unwrap();
				unsafe { ft_putendl_fd(CString::new(input.clone()).unwrap().as_ptr(), 2) };
				prebuf.into_inner().read_to_string(&mut ft_res).unwrap();
				let mut comparator = input.clone();
				comparator.push('\n');
				assert_eq!(
					ft_res, comparator,
					"ft_putendl_fd({}, {}) result differs (ft, lc)",
					input, 2
				);
			}
			#[test]
			fn test_putendl_fd_2() {
				let mut ft_res = String::new();
				let input = String::from("");
				let prebuf = BufferRedirect::stdout().unwrap();
				unsafe { ft_putendl_fd(CString::new(input.clone()).unwrap().as_ptr(), 1) };
				prebuf.into_inner().read_to_string(&mut ft_res).unwrap();
				let mut comparator = input.clone();
				comparator.push('\n');
				assert_eq!(
					ft_res, comparator,
					"ft_putendl_fd({}, {}) result differs (ft, lc)",
					input, 1
				);
			}
			#[test]
			#[ignore]
			fn opt_test_putendl_fd() {
				/* this is optional */
				unsafe { ft_putendl_fd(0 as *const c_char, 1) };
			}
			
		}
	}
	
	#[cfg(test)]
	mod ft_strmapi {
		use super::*;
		extern fn testfn_retidx(idx: u32, _chr: c_char) -> c_char {
			(b'0' + (idx as u8)) as i8
		}
		extern fn testfn_retmod(idx: u32, chr: c_char) -> c_char {
			chr + (idx as i8)
		}
		fn strmapi_tester(
			input: &str,
			f_input: extern fn(u32, c_char) -> c_char,
			expected: &str
		) {
			let res = unsafe {
				ft_strmapi(
					CString::new(input).unwrap().as_ptr(),
					f_input
				)
			};
			let res = unsafe {
				CString::from_raw(res).to_string_lossy().into_owned()
			};
			assert_eq! (
				res, expected,
				"\n\nft_strmapi({:?}, test_f) output differs:\nyours: {:?}\n real: {:?}\n\n",
				input, res, expected
			);
		}
		rusty_fork_test! {
			#[test]
			fn strmapi_basic_1() {
				strmapi_tester("helloworld", testfn_retidx, "0123456789");
			}
			#[test]
			fn strmapi_basic_2() {
				strmapi_tester("aaaaaaaaaa", testfn_retmod, "abcdefghij");
			}
			#[test]
			fn opt_strmapi_null_f() {
				/* passing this test is not required */
				let res = unsafe {
					ft_strmapi(
						CString::new("helloworld").unwrap().as_ptr(),
						#[allow(invalid_value)]
						std::mem::transmute::<u64, extern fn(u32, c_char) -> c_char>(0)
					)
				};
				assert_eq!(
					res, 0 as *mut i8,
					"your strmapi does not return null on null arg"
				);
			}
		}
	}
	
	#[cfg(test)]
	mod ft_strtrim {
		use super::*;
		fn strtrim_tester(input: &str, set: &str, expected: &str) {
			let s1 = CString::new(input).unwrap();
			let s2 = CString::new(set).unwrap();
			let res = unsafe { 
				CString::from_raw(ft_strtrim(s1.as_ptr(), s2.as_ptr()))
			};
			assert_eq! (
				res.to_string_lossy(), expected,
				"\n\nft_strmapi({:?}, {:?}) output differs:\nlibft: {:?}\nmouli: {:?}\n\n",
				input, set,
				res, expected
			)
		}
		rusty_fork_test! {
			#[test]
			fn strtrim_test() {
				strtrim_tester(
					"09243709813098123098132098despacito120793091832098132",
					"1234567890",
					"despacito"
				);
			}
			#[test]
			fn strtrim_test2() {
				strtrim_tester(
					"123321",
					"123123",
					""
				);
			}
			#[test]
			fn strtrim_test3() {
				strtrim_tester(
					"123WHAT TH3 FUCK321",
					"123123",
					"WHAT TH3 FUCK"
				);
			}
			#[test]
			fn strtrim_test4() {
				strtrim_tester(
					"if you read this you are stupid",
					"if you read this you are stupid",
					""
				);
			}
			#[test]
			fn strtrim_test5() {
				strtrim_tester(
					"despayeeto",
					"",
					"despayeeto"
				);
			}
			#[test]
			fn strtrim_test6() {
				strtrim_tester(
					"",
					"123",
					""
				);
			}
		}
	}
	
	#[cfg(test)]
	mod ft_strrchr {
		use super::*;
		fn strrchr_tester(input: &str, c: i32) {
			let tester = CString::new(input).unwrap();
			let ft_res = unsafe { ft_strrchr(tester.as_ptr(), c) };
			let lc_res = unsafe { libc::strrchr(tester.as_ptr(), c) };
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_strrchr({:?} at ptr {:?}, {:?}) result differs\nlibft: {:?}\n libc: {:?}",
				input, input.as_ptr(), c, 
				ft_res, lc_res
			)
		}
		rusty_fork_test! {
			#[test]
			fn strrchr_int_min() {
				strrchr_tester("despacito", i32::min_value());
			}
			#[test]
			fn strrchr_int_max() {
				strrchr_tester("despacito", i32::max_value());
			}
			#[test]
			fn strrchr_nullterm() {
				strrchr_tester("despacito", 0);
			}
			#[test]
			fn strrchr_basic1() {
				strrchr_tester("despacaito", b'a' as i32);
			}
			#[test]
			fn strrchr_basic2() {
				strrchr_tester("despopacito", b'o' as i32);
			}
			#[test]
			fn strrchr_rangetest1() {
				for i in -1000..1000 {
					strrchr_tester(
						"qwiouwhra.nfsd.,mlhasdfoiurewlkasd.m, vchiogdhioewrr.knar.,m  lkoihaiohwarwaerer",
						i
					);
				}
			}
			#[test]
			fn strrchr_rangetest2() {
				for i in -1000..1000 {
					strrchr_tester(
						"qew[ppoqwe[poqwe[poqwe[poqwe[;lladm,nzcx,mn zm,nxc ,nmz\nxsxsxsxsxs",
						i
					);
				}
			}
		}
	}
	#[cfg(test)]
	mod ft_strncmp {
		use super::*;
		rusty_fork_test! {
			#[test]
			fn strncmp_rangetest1() {
				let lhs = CString::new("despacito").unwrap();
				let rhs = CString::new("despacita").unwrap();
				for int in 0..42 {
					let ft_res = unsafe {
						ft_strncmp(lhs.as_ptr(), rhs.as_ptr(), int)
					};
					let lc_res = unsafe {
						strncmp(lhs.as_ptr(), rhs.as_ptr(), int)
					};
					assert_eq!(
						ft_res, lc_res,
						"ft_strncmp({:?}, {:?}, {}) result differs (ft, lc)",
						lhs.to_string_lossy(), rhs.to_string_lossy(), int
					);
				}
			}
			#[test]
			fn strncmp_rangetest2() {
				let lhs = CString::new("despacito").unwrap();
				let rhs = CString::new("despacito").unwrap();
				for int in 0..42 {
					let ft_res = unsafe {
						ft_strncmp(lhs.as_ptr(), rhs.as_ptr(), int)
					};
					let lc_res = unsafe {
						strncmp(lhs.as_ptr(), rhs.as_ptr(), int)
					};
					assert_eq!(
						ft_res, lc_res,
						"ft_strncmp({:?}, {:?}, {}) result differs (ft, lc)",
						lhs.to_string_lossy(), rhs.to_string_lossy(), int
					);
				}
			}
			#[test]
			fn strncmp_rangetest3() {
				let lhs = CString::new("despacito").unwrap();
				let rhs = CString::new("despac").unwrap();
				for int in 0..42 {
					let ft_res = unsafe {
						ft_strncmp(lhs.as_ptr(), rhs.as_ptr(), int)
					};
					let lc_res = unsafe {
						strncmp(lhs.as_ptr(), rhs.as_ptr(), int)
					};
					assert_eq!(
						ft_res, lc_res,
						"ft_strncmp({:?}, {:?}, {}) result differs (ft, lc)",
						lhs.to_string_lossy(), rhs.to_string_lossy(), int
					);
				}
			}
			#[test]
			fn strncmp_rangetest4() {
				let lhs = CString::new("despacito").unwrap();
				let rhs = CString::new("despacitoof").unwrap();
				for int in 0..42 {
					let ft_res = unsafe {
						ft_strncmp(lhs.as_ptr(), rhs.as_ptr(), int)
					};
					let lc_res = unsafe {
						strncmp(lhs.as_ptr(), rhs.as_ptr(), int)
					};
					assert_eq!(
						ft_res, lc_res,
						"ft_strncmp({:?}, {:?}, {}) result differs (ft, lc)",
						lhs.to_string_lossy(), rhs.to_string_lossy(), int
					);
				}
			}
			#[test]
			fn strncmp_rangetest5() {
				let lhs = CString::new("").unwrap();
				let rhs = CString::new("").unwrap();
				for int in 0..42 {
					let ft_res = unsafe {
						ft_strncmp(lhs.as_ptr(), rhs.as_ptr(), int)
					};
					let lc_res = unsafe {
						strncmp(lhs.as_ptr(), rhs.as_ptr(), int)
					};
					assert_eq!(
						ft_res, lc_res,
						"ft_strncmp({:?}, {:?}, {}) result differs (ft, lc)",
						lhs.to_string_lossy(), rhs.to_string_lossy(), int
					);
				}
			}
			#[test]
			fn strncmp_rangetest6() {
				let lhs = CString::new("yeet").unwrap();
				let rhs = CString::new("").unwrap();
				for int in 0..42 {
					let ft_res = unsafe {
						ft_strncmp(lhs.as_ptr(), rhs.as_ptr(), int)
					};
					let lc_res = unsafe {
						strncmp(lhs.as_ptr(), rhs.as_ptr(), int)
					};
					assert_eq!(
						ft_res, lc_res,
						"ft_strncmp({:?}, {:?}, {}) result differs (ft, lc)",
						lhs.to_string_lossy(), rhs.to_string_lossy(), int
					);
				}
			}
			#[test]
			fn strncmp_rangetest7() {
				let lhs = CString::new("").unwrap();
				let rhs = CString::new("yeet").unwrap();
				for int in 0..42 {
					let ft_res = unsafe {
						ft_strncmp(lhs.as_ptr(), rhs.as_ptr(), int)
					};
					let lc_res = unsafe {
						strncmp(lhs.as_ptr(), rhs.as_ptr(), int)
					};
					assert_eq!(
						ft_res, lc_res,
						"ft_strncmp({:?}, {:?}, {}) result differs (ft, lc)",
						lhs.to_string_lossy(), rhs.to_string_lossy(), int
					);
				}
			}
			#[test]
			fn strncmp_rangetest8() {
				let lhs = CString::new("brbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbr").unwrap();
				let rhs = CString::new("brbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbr").unwrap();
				for int in 0..42 {
					let ft_res = unsafe {
						ft_strncmp(lhs.as_ptr(), rhs.as_ptr(), int)
					};
					let lc_res = unsafe {
						strncmp(lhs.as_ptr(), rhs.as_ptr(), int)
					};
					assert_eq!(
						ft_res, lc_res,
						"ft_strncmp({:?}, {:?}, {}) result differs (ft, lc)",
						lhs.to_string_lossy(), rhs.to_string_lossy(), int
					);
				}
			}
			#[test]
			fn strncmp_rangetest9() {
				let lhs = CString::new("MZIRIBMZIRIBMZE123").unwrap();
				let rhs = CString::new("MZIRIBMZE").unwrap();
				for int in 0..42 {
					let ft_res = unsafe {
						ft_strncmp(lhs.as_ptr(), rhs.as_ptr(), int)
					};
					let lc_res = unsafe {
						strncmp(lhs.as_ptr(), rhs.as_ptr(), int)
					};
					assert_eq!(
						ft_res, lc_res,
						"ft_strncmp({:?}, {:?}, {}) result differs (ft, lc)",
						lhs.to_string_lossy(), rhs.to_string_lossy(), int
					);
				}
			}
		}
	}
	mod ft_memcmp {
		use super::*;
		rusty_fork_test! {
			#[test]
			fn memcmp_rangetest1() {
				let lhs = CString::new("despacito").unwrap();
				let rhs = CString::new("despacita").unwrap();
				for int in 0..42 {
					let ft_res = unsafe {
						ft_memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int)
					};
					let lc_res = unsafe {
						memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int)
					};
					assert_eq!(
						ft_res, lc_res,
						"ft_memcmp({:?}, {:?}, {}) result differs (ft, lc)",
						lhs.to_string_lossy(), rhs.to_string_lossy(), int
					);
				}
			}
			#[test]
			fn memcmp_rangetest2() {
				let lhs = CString::new("despacito").unwrap();
				let rhs = CString::new("despacito").unwrap();
				for int in 0..42 {
					let ft_res = unsafe {
						ft_memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int)
					};
					let lc_res = unsafe {
						memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int)
					};
					assert_eq!(
						ft_res, lc_res,
						"ft_memcmp({:?}, {:?}, {}) result differs (ft, lc)",
						lhs.to_string_lossy(), rhs.to_string_lossy(), int
					);
				}
			}
			#[test]
			fn memcmp_rangetest3() {
				let lhs = CString::new("despacito").unwrap();
				let rhs = CString::new("despac").unwrap();
				for int in 0..42 {
					let ft_res = unsafe {
						ft_memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int)
					};
					let lc_res = unsafe {
						memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int)
					};
					assert_eq!(
						ft_res, lc_res,
						"ft_memcmp({:?}, {:?}, {}) result differs (ft, lc)",
						lhs.to_string_lossy(), rhs.to_string_lossy(), int
					);
				}
			}
			#[test]
			fn memcmp_rangetest4() {
				let lhs = CString::new("despacito").unwrap();
				let rhs = CString::new("despacitoof").unwrap();
				for int in 0..42 {
					let ft_res = unsafe {
						ft_memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int)
					};
					let lc_res = unsafe {
						memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int)
					};
					assert_eq!(
						ft_res, lc_res,
						"ft_memcmp({:?}, {:?}, {}) result differs (ft, lc)",
						lhs.to_string_lossy(), rhs.to_string_lossy(), int
					);
				}
			}
			#[test]
			fn memcmp_rangetest5() {
				let lhs = CString::new("").unwrap();
				let rhs = CString::new("").unwrap();
				for int in 0..42 {
					let ft_res = unsafe {
						ft_memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int)
					};
					let lc_res = unsafe {
						memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int)
					};
					assert_eq!(
						ft_res, lc_res,
						"ft_memcmp({:?}, {:?}, {}) result differs (ft, lc)",
						lhs.to_string_lossy(), rhs.to_string_lossy(), int
					);
				}
			}
			#[test]
			fn memcmp_rangetest6() {
				let lhs = CString::new("yeet").unwrap();
				let rhs = CString::new("").unwrap();
				for int in 0..42 {
					let ft_res = unsafe {
						ft_memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int)
					};
					let lc_res = unsafe {
						memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int)
					};
					assert_eq!(
						ft_res, lc_res,
						"ft_memcmp({:?}, {:?}, {}) result differs (ft, lc)",
						lhs.to_string_lossy(), rhs.to_string_lossy(), int
					);
				}
			}
			#[test]
			fn memcmp_rangetest7() {
				let lhs = CString::new("").unwrap();
				let rhs = CString::new("yeet").unwrap();
				for int in 0..42 {
					let ft_res = unsafe {
						ft_memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int)
					};
					let lc_res = unsafe {
						memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int)
					};
					assert_eq!(
						ft_res, lc_res,
						"ft_memcmp({:?}, {:?}, {}) result differs (ft, lc)",
						lhs.to_string_lossy(), rhs.to_string_lossy(), int
					);
				}
			}
			#[test]
			fn memcmp_rangetest8() {
				let lhs = CString::new("brbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbr").unwrap();
				let rhs = CString::new("brbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbrbr").unwrap();
				for int in 0..42 {
					let ft_res = unsafe {
						ft_memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int)
					};
					let lc_res = unsafe {
						memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int)
					};
					assert_eq!(
						ft_res, lc_res,
						"ft_memcmp({:?}, {:?}, {}) result differs (ft, lc)",
						lhs.to_string_lossy(), rhs.to_string_lossy(), int
					);
				}
			}
		}
	}

	#[cfg(test)]
	mod ft_strnstr {
		use super::*;

		rusty_fork_test! {
			#[test]
			fn strnstr_test1() {
				let haystack = CString::new("despacito").unwrap();
				let needle = CString::new("").unwrap();
				for int in 0..42 {
					let ft_res = unsafe {
						ft_strnstr(haystack.as_ptr(), needle.as_ptr(), int)
					};
					let lc_res = unsafe {
						strnstr(haystack.as_ptr(), needle.as_ptr(), int)
					};
					assert_eq!(
						ft_res, lc_res,
						"ft_strnstr({:?}, {:?}, {}) result differs (ft, lc)",
						haystack.to_string_lossy(), needle.to_string_lossy(), int
					);
				}
			}
			#[test]
			fn strnstr_test2() {
				let haystack = CString::new("hello, world").unwrap();
				let needle = CString::new("lo").unwrap();
				for int in 0..42 {
					let ft_res = unsafe {
						ft_strnstr(haystack.as_ptr(), needle.as_ptr(), int)
					};
					let lc_res = unsafe {
						strnstr(haystack.as_ptr(), needle.as_ptr(), int)
					};
					assert_eq!(
						ft_res, lc_res,
						"ft_strnstr({:?}, {:?}, {}) result differs (ft, lc)",
						haystack.to_string_lossy(), needle.to_string_lossy(), int
					);
				}
			}
			#[test]
			fn strnstr_test3() {
				let haystack = CString::new("bololo, haha").unwrap();
				let needle = CString::new("lo").unwrap();
				for int in 0..42 {
					let ft_res = unsafe {
						ft_strnstr(haystack.as_ptr(), needle.as_ptr(), int)
					};
					let lc_res = unsafe {
						strnstr(haystack.as_ptr(), needle.as_ptr(), int)
					};
					assert_eq!(
						ft_res, lc_res,
						"ft_strnstr({:?}, {:?}, {}) result differs (ft, lc)",
						haystack.to_string_lossy(), needle.to_string_lossy(), int
					);
				}
			}
			#[test]
			fn strnstr_test4() {
				let haystack = CString::new("lkdgjalkdfjglkadfjgklajdfgkjadfg").unwrap();
				let needle = CString::new("lo").unwrap();
				for int in 0..42 {
					let ft_res = unsafe {
						ft_strnstr(haystack.as_ptr(), needle.as_ptr(), int)
					};
					let lc_res = unsafe {
						strnstr(haystack.as_ptr(), needle.as_ptr(), int)
					};
					assert_eq!(
						ft_res, lc_res,
						"ft_strnstr({:?}, {:?}, {}) result differs (ft, lc)",
						haystack.to_string_lossy(), needle.to_string_lossy(), int
					);
				}
			}
			#[test]
			fn strnstr_test5() {
				let haystack = CString::new("").unwrap();
				let needle = CString::new("").unwrap();
				for int in 0..42 {
					let ft_res = unsafe {
						ft_strnstr(haystack.as_ptr(), needle.as_ptr(), int)
					};
					let lc_res = unsafe {
						strnstr(haystack.as_ptr(), needle.as_ptr(), int)
					};
					assert_eq!(
						ft_res, lc_res,
						"ft_strnstr({:?}, {:?}, {}) result differs (ft, lc)",
						haystack.to_string_lossy(), needle.to_string_lossy(), int
					);
				}
			}
			#[test]
			fn strnstr_test6() {
				let haystack = CString::new("").unwrap();
				let needle = CString::new("").unwrap();
				for int in 0..42 {
					let ft_res = unsafe {
						ft_strnstr(haystack.as_ptr(), needle.as_ptr(), int)
					};
					let lc_res = unsafe {
						strnstr(haystack.as_ptr(), needle.as_ptr(), int)
					};
					assert_eq!(
						ft_res, lc_res,
						"ft_strnstr({:?}, {:?}, {}) result differs (ft, lc)",
						haystack.to_string_lossy(), needle.to_string_lossy(), int
					);
				}
			}
			#[test]
			fn strnstr_test7() {
				let haystack = CString::new("aaapaapaap").unwrap();
				let needle = CString::new("aap").unwrap();
				for int in 0..42 {
					let ft_res = unsafe {
						ft_strnstr(haystack.as_ptr(), needle.as_ptr(), int)
					};
					let lc_res = unsafe {
						strnstr(haystack.as_ptr(), needle.as_ptr(), int)
					};
					assert_eq!(
						ft_res, lc_res,
						"ft_strnstr({:?}, {:?}, {}) result differs (ft, lc)",
						haystack.to_string_lossy(), needle.to_string_lossy(), int
					);
				}
			}
			#[test]
			fn strnstr_test8() {
				let haystack = CString::new("MZIRIBMZIRIBMZE123").unwrap();
				let needle = CString::new("MZIRIBMZE").unwrap();
				for int in 0..42 {
					let ft_res = unsafe {
						ft_strnstr(haystack.as_ptr(), needle.as_ptr(), int)
					};
					let lc_res = unsafe {
						strnstr(haystack.as_ptr(), needle.as_ptr(), int)
					};
					assert_eq!(
						ft_res, lc_res,
						"ft_strnstr({:?}, {:?}, {}) result differs (ft, lc)",
						haystack.to_string_lossy(), needle.to_string_lossy(), int
					);
				}
			}
		}
	}
	
	#[cfg(test)]
	mod ft_itoa {
		use super::*;
		rusty_fork_test! {
			#[test]
			fn itoa_int_min() {
				let input = i32::min_value();
				for i in input..(input + 1024) {
					let ft_res = unsafe {
						ft_itoa(i)
					};
					let lc_res = i.to_string();
					assert_eq!(
						unsafe { CString::from_raw(ft_res).to_string_lossy() }, lc_res,
						"itoa({}) result differs (ft, lc)",
						i
					);
				}
			}
			#[test]
			fn itoa_int_max() {
				let input = i32::max_value();
				for i in (input - 1024)..input {
					let ft_res = unsafe {
						ft_itoa(i)
					};
					let lc_res = i.to_string();
					assert_eq!(
						unsafe { CString::from_raw(ft_res).to_string_lossy() }, lc_res,
						"itoa({}) result differs (ft, lc)",
						i
					);
				}
			}
			#[test]
			fn itoa_test1() {
				for i in -1024..1024 {
					let ft_res = unsafe {
						ft_itoa(i)
					};
					let lc_res = i.to_string();
					assert_eq!(
						unsafe { CString::from_raw(ft_res).to_string_lossy() }, lc_res,
						"itoa({}) result differs (ft, lc)",
						i
					);
				}
			}
			#[test]
			fn itoa_test0() {
				let i = 0;
				{
					let ft_res = unsafe {
						ft_itoa(i)
					};
					let lc_res = i.to_string();
					assert_eq!(
						unsafe { CString::from_raw(ft_res).to_string_lossy() }, lc_res,
						"itoa({}) result differs (ft, lc)",
						i
					);
				}
			}
		}
	}
	#[cfg(test)]
	mod ft_strjoin {
		use super::*;
		fn strjoin_tester(lhs: &str, rhs: &str, expected: &str) {
			let arg_01 = CString::new(lhs).unwrap();
			let arg_02 = CString::new(rhs).unwrap();
			let ft_res = unsafe { CString::from_raw(ft_strjoin(arg_01.as_ptr(), arg_02.as_ptr())) };
			assert_eq!(
				ft_res.to_string_lossy(), expected,
				"\n\nft_strjoin({:?}, {:?}) result differs\nyours: {:?}\nmouli: {:?}\n\n",
				lhs, rhs,
				ft_res, expected
			);
		}
		rusty_fork_test! {
			#[test]
			fn strjoin_test_1() {
				strjoin_tester("hello", " world", "hello world");
			}
			#[test]
			fn strjoin_test_2() {
				strjoin_tester("fa;lsdf;aksdf;lk;", "eirjoi1u298102938", "fa;lsdf;aksdf;lk;eirjoi1u298102938");
			}
			#[test]
			fn strjoin_test_3() {
				strjoin_tester("", "", "");
			}
			#[test]
			fn strjoin_test_4() {
				strjoin_tester("", "ab", "ab");
			}
			#[test]
			fn strjoin_test_5() {
				strjoin_tester("ab", "", "ab");
			}
			#[test]
			#[ignore]
			fn opt_strjoin_null_lhs() {
				let valid = CString::new("hello, world").unwrap();
				unsafe { ft_strjoin(0 as *const c_char, valid.as_ptr()); }
			}
			#[test]
			#[ignore]
			fn opt_strjoin_null_rhs() {
				let valid = CString::new("hello, world").unwrap();
				unsafe { ft_strjoin(valid.as_ptr(), 0 as *const c_char); }
			}
			#[test]
			#[ignore]
			fn opt_strjoin_null_both() {
				unsafe { ft_strjoin(0 as *const c_char, 0 as *const c_char); }
			}
		}
	}
	
	#[cfg(test)]
	mod ft_strchr {
		use super::*;
		fn strchr_tester(input: &str, c: i32) {
			let tester = CString::new(input).unwrap();
			let ft_res = unsafe { ft_strchr(tester.as_ptr(), c) };
			let lc_res = unsafe { libc::strchr(tester.as_ptr(), c) };
			assert_eq!(
				ft_res, lc_res,
				"\n\nft_strchr({:?} at ptr {:?}, {:?}) result differs\nlibft: {:?}\n libc: {:?}",
				input, input.as_ptr(), c, 
				ft_res, lc_res
			)
		}
		rusty_fork_test! {
			#[test]
			fn strchr_int_min() {
				strchr_tester("poiwrlwker,dsfa", i32::min_value());
			}
			#[test]
			fn strchr_int_max() {
				strchr_tester("v;kansdflalkrj", i32::max_value());
			}
			#[test]
			fn strchr_nullterm() {
				strchr_tester("2193i124-9u1ih42[i12op4j", 0);
			}
			#[test]
			fn strchr_basic1() {
				strchr_tester("if you read this you are stupid", b'e' as i32);
			}
			#[test]
			fn strchr_basic2() {
				strchr_tester("if you read this you are stupid x2", b'u' as i32);
			}
			#[test]
			fn strchr_rangetest1() {
				for i in -1000..1000 {
					strchr_tester("hello, world", i);
				}
			}
			#[test]
			fn strchr_rangetest2() {
				for i in -1000..1000 {
					strchr_tester("pqoirwojkkwqklrjlkbneq;;wlmnqwmnm,m,mewqmewrlkkkl;jlj", i);
				}
			}
		}
	}

	#[cfg(test)]
	mod ft_memchr {
		use super::*;
		rusty_fork_test! {
			#[test]
			fn memchr_int_min() {
				let cs = CString::new("despacito111111").unwrap();
				let input = i32::min_value();
				let ft_res = unsafe {
					ft_memchr(cs.as_ptr() as *const c_void, input, 10)
				};
				let lc_res = unsafe {
					memchr(cs.as_ptr() as *const c_void, input, 10)
				};
				assert_eq!(
					ft_res, lc_res,
					"ft_memchr({:?}, {}) result differs (ft, lc)",
					cs.to_string_lossy(), input
				);
			}
			#[test]
			fn memchr_int_max() {
				let cs = CString::new("despacito111111").unwrap();
				let input = i32::max_value();
				let ft_res = unsafe {
					ft_memchr(cs.as_ptr() as *const c_void, input, 10)
				};
				let lc_res = unsafe {
					libc::memchr(cs.as_ptr() as *const c_void, input, 10)
				};
				assert_eq!(
					ft_res, lc_res,
					"ft_memchr({:?}, {}) result differs (ft, lc)",
					cs.to_string_lossy(), input
				);
			}
			#[test]
			fn memchr_nullterm() {
				let cs = CString::new("despacit1111o").unwrap();
				let input = 0;
				let ft_res = unsafe {
					ft_memchr(cs.as_ptr() as *const c_void, input, 10)
				};
				let lc_res = unsafe {
					libc::memchr(cs.as_ptr() as *const c_void, input, 10)
				};
				assert_eq!(
					ft_res, lc_res,
					"ft_memchr({:?}, {}) result differs (ft, lc)",
					cs.to_string_lossy(), input
				);
			}
			#[test]
			fn memchr_basic1() {
				let cs = CString::new("des1111111pacaito").unwrap();
				let input = b'a' as i32;
				let ft_res = unsafe {
					ft_memchr(cs.as_ptr() as *const c_void, input, 16)
				};
				let lc_res = unsafe {
					libc::memchr(cs.as_ptr() as *const c_void, input, 16)
				};
				assert_eq!(
					ft_res, lc_res,
					"ft_memchr({:?}, {}) result differs (ft, lc)",
					cs.to_string_lossy(), input
				);
			}
			#[test]
			fn memchr_basic2() {
				let cs = CString::new("deso111111pacito").unwrap();
				let input = b'o' as i32;
				let ft_res = unsafe {
					ft_memchr(cs.as_ptr() as *const c_void, input, 10)
				};
				let lc_res = unsafe {
					libc::memchr(cs.as_ptr() as *const c_void, input, 10)
				};
				assert_eq!(
					ft_res, lc_res,
					"ft_memchr({:?}, {}) result differs (ft, lc)",
					cs.to_string_lossy(), input
				);
			}
			#[test]
			fn memchr_zerolen() {
				let cs = CString::new("deso111111pacito").unwrap();
				let input = b'o' as i32;
				let ft_res = unsafe {
					ft_memchr(cs.as_ptr() as *const c_void, input, 0)
				};
				let lc_res = unsafe {
					libc::memchr(cs.as_ptr() as *const c_void, input, 0)
				};
				assert_eq!(
					ft_res, lc_res,
					"ft_memchr({:?}, {}) result differs (ft, lc)",
					cs.to_string_lossy(), input
				);
			}
		}
	}
	
	#[cfg(test)]
	mod ft_split {
		use super::*;
		rusty_fork_test! {
			#[test]
			fn test_basic() {
				let res = unsafe {
					ft_split(CString::new("hello world").unwrap().as_ptr(), b' ' as c_char)
				};
				let expected = vec![
					"hello",
					"world"
				];
				unsafe {
					let mut offset = 0;
					while !(*res.offset(offset) as *mut c_char).is_null() {
						let act = CString::from_raw(*res.offset(offset) as *mut c_char)
							.to_string_lossy()
							.into_owned();
						assert_eq!(
							expected[offset as usize], act,
							"\n\nsplit result differs,\nexpected: {:?}\n got: {:?}\n\n",
							expected[offset as usize], act
						);
						offset += 1;
					}
				}
			}
			#[test]
			fn test_basic2() {
				let res = unsafe {
					ft_split(CString::new("hello world ").unwrap().as_ptr(), b' ' as c_char)
				};
				let expected = vec![
					"hello",
					"world"
				];
				unsafe {
					let mut offset = 0;
					while !(*res.offset(offset) as *mut c_char).is_null() {
						let act = CString::from_raw(*res.offset(offset) as *mut c_char)
							.to_string_lossy()
							.into_owned();
						assert_eq!(
							expected[offset as usize], act,
							"\n\nsplit result differs,\nexpected: {:?}\n got: {:?}\n\n",
							expected[offset as usize], act
						);
						offset += 1;
					}
				}
			}
			#[test]
			fn test_empty() {
				unsafe {
					let res = ft_split(CString::new(",,,,").unwrap().as_ptr(), b',' as c_char);
					assert!(
						!res.offset(0).is_null(),
						"string of only delimiters has non-empty result"
					);
				}
			}
			#[test]
			fn test_empty1() {
				unsafe {
					let res = ft_split(CString::new("").unwrap().as_ptr(), b',' as c_char);
					assert!(
						!res.offset(0).is_null(),
						"empty string has non-empty result"
					);
				}
			}
			#[test]
			fn test_empty2() {
				unsafe {
					let res = ft_split(CString::new("").unwrap().as_ptr(), b'z' as c_char);
					assert!(
						!res.offset(0).is_null(),
						"empty string has non-empty result"
					);
					assert!(
						(*res.offset(0)).is_null(),
						"first value is NOT null"
					);
				}
			}
			#[test]
			fn test_empty3() {
				unsafe {
					let res = ft_split(CString::new("").unwrap().as_ptr(), b'z' as c_char);
					assert!(
						!res.offset(0).is_null(),
						"empty string has non-empty result"
					);
					let expected = Vec::<&str>::new();
					let mut offset = 0;
					while !(*res.offset(offset) as *mut c_char).is_null() {
						let act = CString::from_raw(*res.offset(offset) as *mut c_char)
							.to_string_lossy()
							.into_owned();
						assert_eq!(
							expected[offset as usize], act,
							"\n\nsplit result differs,\nexpected: {:?}\n got: {:?}\n\n",
							expected[offset as usize], act
						);
						offset += 1;
					}
				}
			}
			#[test]
			fn test_nullterm() {
				/* dvoort test */
				unsafe {
					let res = ft_split(CString::new("").unwrap().as_ptr(), b'\x00' as c_char);
					assert!(
						!res.offset(0).is_null(),
						"null-terminator delim on empty string returns non-empty"
					);
				}
			}
			#[test]
			fn test_nullterm1() {
				let res = unsafe {
					ft_split(CString::new("hello world").unwrap().as_ptr(), b'\x00' as c_char)
				};
				let expected = vec![
					"hello world"
				];
				unsafe {
					let mut offset = 0;
					while !(*res.offset(offset) as *mut c_char).is_null() {
						let act = CString::from_raw(*res.offset(offset) as *mut c_char)
							.to_string_lossy()
							.into_owned();
						assert_eq!(
							expected[offset as usize], act,
							"\n\nsplit result differs,\nexpected: {:?}\n got: {:?}\n\n",
							expected[offset as usize], act
						);
						offset += 1;
					}
				}
			}
		}
	}
}