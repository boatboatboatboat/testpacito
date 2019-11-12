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
	fn ft_strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
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
		rusty_fork_test! {
		#[test]
		fn test_basic_1() {
			let tester = CString::new("123456").unwrap();
			let real = unsafe { atoi(tester.as_ptr()) };
			let ret = unsafe { ft_atoi(tester.as_ptr()) };
			assert_eq!(
				ret, real,
				"ft_atoi(\"{}\") returns {}, should be {}",
				tester.into_string().unwrap(), ret, real
			);
		}
		#[test]
		fn test_basic_2() {
			let tester = CString::new("42069").unwrap();
			let real = unsafe { atoi(tester.as_ptr()) };
			let ret = unsafe { ft_atoi(tester.as_ptr()) };
			assert_eq!(
				ret, real,
				"ft_atoi(\"{}\") returns {}, should be {}",
				tester.into_string().unwrap(), ret, real
			);
		}
		#[test]
		fn test_extremes_1() {
			let tester = CString::new(i32::max_value().to_string()).unwrap();
			let real = unsafe { atoi(tester.as_ptr()) };
			let ret = unsafe { ft_atoi(tester.as_ptr()) };
			assert_eq!(
				ret, real,
				"ft_atoi(\"{}\") returns {}, should be {}",
				tester.into_string().unwrap(), ret, real
			);
		}
		#[test]
		fn test_extremes_2() {
			let tester = CString::new(i32::min_value().to_string()).unwrap();
			let real = unsafe { atoi(tester.as_ptr()) };
			let ret = unsafe { ft_atoi(tester.as_ptr()) };
			assert_eq!(
				ret, real,
				"ft_atoi(\"{}\") returns {}, should be {}",
				tester.into_string().unwrap(), ret, real
			);
		}
		#[test]
		fn test_extremes_3() {
			let tester = CString::new(i64::min_value().to_string()).unwrap();
			let real = unsafe { atoi(tester.as_ptr()) };
			let ret = unsafe { ft_atoi(tester.as_ptr()) };
			assert_eq!(
				ret, real,
				"ft_atoi(\"{}\") returns {}, should be {}",
				tester.into_string().unwrap(), ret, real
			);
		}
		#[test]
		fn test_extremes_4() {
			let tester = CString::new(i64::max_value().to_string()).unwrap();
			let real = unsafe { atoi(tester.as_ptr()) };
			let ret = unsafe { ft_atoi(tester.as_ptr()) };
			assert_eq!(
				ret, real,
				"ft_atoi(\"{}\") returns {}, should be {}",
				tester.into_string().unwrap(), ret, real
			);
		}
		#[test]
		fn test_skipping_1() {
			let tester = CString::new("+123").unwrap();
			let real = unsafe { libc::atoi(tester.as_ptr()) };
			let ret = unsafe { ft_atoi(tester.as_ptr()) };
			assert_eq!(
				ret, real,
				"ft_atoi(\"{}\") returns {}, should be {}",
				tester.into_string().unwrap(), ret, real
			);
		}
		#[test]
		fn test_skipping_2() {
			let tester = CString::new("+-123").unwrap();
			let real = unsafe { libc::atoi(tester.as_ptr()) };
			let ret = unsafe { ft_atoi(tester.as_ptr()) };
			assert_eq!(
				ret, real,
				"ft_atoi(\"{}\") returns {}, should be {}",
				tester.into_string().unwrap(), ret, real
			);
		}
		#[test]
		fn test_skipping_3() {
			let tester = CString::new("       +123 123").unwrap();
			let real = unsafe { libc::atoi(tester.as_ptr()) };
			let ret = unsafe { ft_atoi(tester.as_ptr()) };
			assert_eq!(
				ret, real,
				"ft_atoi(\"{}\") returns {}, should be {}",
				tester.into_string().unwrap(), ret, real
			);
		}
		#[test]
		fn test_skipping_4() {
			let tester = CString::new("-42069").unwrap();
			let real = unsafe { libc::atoi(tester.as_ptr()) };
			let ret = unsafe { ft_atoi(tester.as_ptr()) };
			assert_eq!(
				ret, real,
				"ft_atoi(\"{}\") returns {}, should be {}",
				tester.into_string().unwrap(), ret, real
			);
		}
		#[test]
		fn test_skipping_5() {
			let tester = CString::new("\t\n\r\x0b\x0c 987").unwrap();
			let real = unsafe { ft_atoi(tester.as_ptr()) };
			let ret = unsafe { ft_atoi(tester.as_ptr()) };
			assert_eq!(
				ret, real,
				"ft_atoi(\"{}\") returns {}, should be {}",
				tester.into_string().unwrap().escape_default().collect::<String>(), ret, real
			);
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
		rusty_fork_test! {
			#[test]
			fn strlen_test() {
				let input = CString::new("").unwrap();
				let ft_res = unsafe {
					ft_strlen(input.as_ptr());
				};
				let lc_res = unsafe {
					libc::strlen(input.as_ptr());
				};
				assert_eq!(
					ft_res, lc_res,
					"ft_strchr({:?}) result differs (ft, lc)",
					input.to_string_lossy()
				);
			}
			#[test]
			fn strlen_test2() {
				let input = CString::new("a").unwrap();
				let ft_res = unsafe {
					ft_strlen(input.as_ptr());
				};
				let lc_res = unsafe {
					libc::strlen(input.as_ptr());
				};
				assert_eq!(
					ft_res, lc_res,
					"ft_strchr({:?}) result differs (ft, lc)",
					input.to_string_lossy()
				);
			}
			#[test]
			fn strlen_test3() {
				let input = CString::new("ab").unwrap();
				let ft_res = unsafe {
					ft_strlen(input.as_ptr());
				};
				let lc_res = unsafe {
					libc::strlen(input.as_ptr());
				};
				assert_eq!(
					ft_res, lc_res,
					"ft_strchr({:?}) result differs (ft, lc)",
					input.to_string_lossy()
				);
			}
			#[test]
			fn strlen_test4() {
				let input = CString::new("dfnalksdjflkajsdklfjaslkdfjloeiuwroiwu").unwrap();
				let ft_res = unsafe {
					ft_strlen(input.as_ptr());
				};
				let lc_res = unsafe {
					libc::strlen(input.as_ptr());
				};
				assert_eq!(
					ft_res, lc_res,
					"ft_strchr({:?}) result differs (ft, lc)",
					input.to_string_lossy()
				);
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
			fn opt_test_putendl_fd() {
				/* this is optional */
				unsafe { ft_putendl_fd(0 as *const c_char, 1) };
			}
			
		}
	}
	
	#[cfg(test)]
	mod ft_strmapi {
		use super::*;
		rusty_fork_test!{
			#[test]
			fn strmapi_basic_1() {
				extern fn test_f(idx: u32, _chr: c_char) -> c_char {
					(b'0' + (idx as u8)) as i8
				}
				let res = unsafe {
					ft_strmapi(
						CString::new("helloworld").unwrap().as_ptr(),
						test_f
					)
				};
				let res = unsafe {CString::from_raw(res).to_string_lossy().into_owned()};
				assert_eq!(
					"0123456789", res,
					"\n\nft_strmapi({:?}) output differs:\nyours: {:?}\nlibc:  {:?}\n\n",
					"helloworld", res, "0123456789"
				);
			}
			#[test]
			fn strmapi_basic_2() {
				extern fn test_f(idx: u32, chr: c_char) -> c_char {
					chr + (idx as i8)
				}
				let res = unsafe {
					ft_strmapi(
						CString::new("aaaaaaaaaa").unwrap().as_ptr(),
						test_f
					)
				};
				let res = unsafe {CString::from_raw(res).to_string_lossy().into_owned()};
				assert_eq!(
					"abcdefghij", res,
					"\n\nft_strmapi({:?}) output differs:\nyours: {:?}\nlibc:  {:?}\n\n",
					"aaaaaaaaaa", res, "abcdefghij"
				);
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
		rusty_fork_test! {
			#[test]
			fn strtrim_test() {
				let s1 = CString::new("1127498172938despacito129019274").unwrap();
				let set = CString::new("1234567890").unwrap();
				let comparator = "despacito";
				let res = unsafe { ft_strtrim(s1.as_ptr(), set.as_ptr()) };
				assert_eq!(
					unsafe { CString::from_raw(res).to_owned().to_string_lossy() }, comparator,
					"ft_strrchr({:?}, {:?}) result differs (ft, lc)",
					s1.to_string_lossy(), set.to_string_lossy()
				);
			}
			#[test]
			fn strtrim_test2() {
				let s1 = CString::new("123321").unwrap();
				let set = CString::new("123123").unwrap();
				let comparator = "";
				let res = unsafe { ft_strtrim(s1.as_ptr(), set.as_ptr()) };
				assert_eq!(
					unsafe { CString::from_raw(res).to_owned().to_string_lossy() }, comparator,
					"ft_strrchr({:?}, {:?}) result differs (ft, lc)",
					s1.to_string_lossy(), set.to_string_lossy()
				);
			}
			#[test]
			fn strtrim_wtf1() {
				let s1 = CString::new("98731132.056465478643210.23156487").unwrap();
				let set = CString::new("98731132.056465478643210.23156487").unwrap();
				let comparator = "";
				let res = unsafe { ft_strtrim(s1.as_ptr(), set.as_ptr()) };
				assert_eq!(
					unsafe { CString::from_raw(res).to_owned().to_string_lossy() }, comparator,
					"ft_strrchr({:?}, {:?}) result differs (ft, lc)",
					s1.to_string_lossy(), set.to_string_lossy()
				);
			}
			#[test]
			fn strtrim_wtf2() {
				let s1 = CString::new("1127498172938despacito129019274").unwrap();
				let set = CString::new("1127498172938despacito129019274").unwrap();
				let comparator = "";
				let res = unsafe { ft_strtrim(s1.as_ptr(), set.as_ptr()) };
				assert_eq!(
					unsafe { CString::from_raw(res).to_owned().to_string_lossy() }, comparator,
					"ft_strrchr({:?}, {:?}) result differs (ft, lc)",
					s1.to_string_lossy(), set.to_string_lossy()
				);
			}
		}
	}
	
	#[cfg(test)]
	mod ft_strrchr {
		use super::*;
		rusty_fork_test! {
			#[test]
			fn strrchr_int_min() {
				let cs = CString::new("despacito").unwrap();
				let input = i32::min_value();
				let ft_res = unsafe {
					ft_strrchr(cs.as_ptr(), input)
				};
				let lc_res = unsafe {
					libc::strrchr(cs.as_ptr(), input)
				};
				assert_eq!(
					ft_res, lc_res,
					"ft_strrchr({:?}, {}) result differs (ft, lc)",
					cs.to_string_lossy(), input
				);
			}
			#[test]
			fn strrchr_int_max() {
				let cs = CString::new("despacito").unwrap();
				let input = i32::max_value();
				let ft_res = unsafe {
					ft_strrchr(cs.as_ptr(), input)
				};
				let lc_res = unsafe {
					libc::strrchr(cs.as_ptr(), input)
				};
				assert_eq!(
					ft_res, lc_res,
					"ft_strrchr({:?}, {}) result differs (ft, lc)",
					cs.to_string_lossy(), input
				);
			}
			#[test]
			fn strrchr_nullterm() {
				let cs = CString::new("despacito").unwrap();
				let input = 0;
				let ft_res = unsafe {
					ft_strrchr(cs.as_ptr(), input)
				};
				let lc_res = unsafe {
					libc::strrchr(cs.as_ptr(), input)
				};
				assert_eq!(
					ft_res, lc_res,
					"ft_strrchr({:?}, {}) result differs (ft, lc)",
					cs.to_string_lossy(), input
				);
			}
			#[test]
			fn strrchr_basic1() {
				let cs = CString::new("despacaito").unwrap();
				let input = b'a' as i32;
				let ft_res = unsafe {
					ft_strrchr(cs.as_ptr(), input)
				};
				let lc_res = unsafe {
					libc::strrchr(cs.as_ptr(), input)
				};
				assert_eq!(
					ft_res, lc_res,
					"ft_strrchr({:?}, {}) result differs (ft, lc)",
					cs.to_string_lossy(), input
				);
			}
			#[test]
			fn strrchr_basic2() {
				let cs = CString::new("desopacito").unwrap();
				let input = b'o' as i32;
				let ft_res = unsafe {
					ft_strrchr(cs.as_ptr(), input)
				};
				let lc_res = unsafe {
					libc::strrchr(cs.as_ptr(), input)
				};
				assert_eq!(
					ft_res, lc_res,
					"ft_strrchr({:?}, {}) result differs (ft, lc)",
					cs.to_string_lossy(), input
				);
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
						ft_strncmp(lhs.as_ptr(), rhs.as_ptr(), int);
					};
					let lc_res = unsafe {
						libc::strncmp(lhs.as_ptr(), rhs.as_ptr(), int);
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
						ft_strncmp(lhs.as_ptr(), rhs.as_ptr(), int);
					};
					let lc_res = unsafe {
						libc::strncmp(lhs.as_ptr(), rhs.as_ptr(), int);
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
						ft_strncmp(lhs.as_ptr(), rhs.as_ptr(), int);
					};
					let lc_res = unsafe {
						libc::strncmp(lhs.as_ptr(), rhs.as_ptr(), int);
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
						ft_strncmp(lhs.as_ptr(), rhs.as_ptr(), int);
					};
					let lc_res = unsafe {
						libc::strncmp(lhs.as_ptr(), rhs.as_ptr(), int);
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
						ft_strncmp(lhs.as_ptr(), rhs.as_ptr(), int);
					};
					let lc_res = unsafe {
						libc::strncmp(lhs.as_ptr(), rhs.as_ptr(), int);
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
						ft_strncmp(lhs.as_ptr(), rhs.as_ptr(), int);
					};
					let lc_res = unsafe {
						libc::strncmp(lhs.as_ptr(), rhs.as_ptr(), int);
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
						ft_strncmp(lhs.as_ptr(), rhs.as_ptr(), int);
					};
					let lc_res = unsafe {
						libc::strncmp(lhs.as_ptr(), rhs.as_ptr(), int);
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
						ft_strncmp(lhs.as_ptr(), rhs.as_ptr(), int);
					};
					let lc_res = unsafe {
						libc::strncmp(lhs.as_ptr(), rhs.as_ptr(), int);
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
						ft_memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int);
					};
					let lc_res = unsafe {
						memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int);
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
						ft_memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int);
					};
					let lc_res = unsafe {
						memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int);
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
						ft_memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int);
					};
					let lc_res = unsafe {
						memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int);
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
						ft_memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int);
					};
					let lc_res = unsafe {
						memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int);
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
						ft_memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int);
					};
					let lc_res = unsafe {
						memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int);
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
						ft_memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int);
					};
					let lc_res = unsafe {
						memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int);
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
						ft_memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int);
					};
					let lc_res = unsafe {
						memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int);
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
						ft_memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int);
					};
					let lc_res = unsafe {
						memcmp(lhs.as_ptr() as *const c_void, rhs.as_ptr() as *const c_void, int);
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
						ft_strnstr(haystack.as_ptr(), needle.as_ptr(), int);
					};
					let lc_res = unsafe {
						strnstr(haystack.as_ptr(), needle.as_ptr(), int);
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
						ft_strnstr(haystack.as_ptr(), needle.as_ptr(), int);
					};
					let lc_res = unsafe {
						strnstr(haystack.as_ptr(), needle.as_ptr(), int);
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
						ft_strnstr(haystack.as_ptr(), needle.as_ptr(), int);
					};
					let lc_res = unsafe {
						strnstr(haystack.as_ptr(), needle.as_ptr(), int);
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
						ft_strnstr(haystack.as_ptr(), needle.as_ptr(), int);
					};
					let lc_res = unsafe {
						strnstr(haystack.as_ptr(), needle.as_ptr(), int);
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
						ft_strnstr(haystack.as_ptr(), needle.as_ptr(), int);
					};
					let lc_res = unsafe {
						strnstr(haystack.as_ptr(), needle.as_ptr(), int);
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
						ft_strnstr(haystack.as_ptr(), needle.as_ptr(), int);
					};
					let lc_res = unsafe {
						strnstr(haystack.as_ptr(), needle.as_ptr(), int);
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
						ft_strnstr(haystack.as_ptr(), needle.as_ptr(), int);
					};
					let lc_res = unsafe {
						strnstr(haystack.as_ptr(), needle.as_ptr(), int);
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
	mod ft_strchr {
		use super::*;
		rusty_fork_test! {
			#[test]
			fn strchr_int_min() {
				let cs = CString::new("despacito").unwrap();
				let input = i32::min_value();
				let ft_res = unsafe {
					ft_strchr(cs.as_ptr(), input)
				};
				let lc_res = unsafe {
					strchr(cs.as_ptr(), input)
				};
				assert_eq!(
					ft_res, lc_res,
					"ft_strchr({:?}, {}) result differs (ft, lc)",
					cs.to_string_lossy(), input
				);
			}
			#[test]
			fn strchr_int_max() {
				let cs = CString::new("despacito").unwrap();
				let input = i32::max_value();
				let ft_res = unsafe {
					ft_strchr(cs.as_ptr(), input)
				};
				let lc_res = unsafe {
					libc::strchr(cs.as_ptr(), input)
				};
				assert_eq!(
					ft_res, lc_res,
					"ft_strchr({:?}, {}) result differs (ft, lc)",
					cs.to_string_lossy(), input
				);
			}
			#[test]
			fn strchr_nullterm() {
				let cs = CString::new("despacito").unwrap();
				let input = 0;
				let ft_res = unsafe {
					ft_strchr(cs.as_ptr(), input)
				};
				let lc_res = unsafe {
					libc::strchr(cs.as_ptr(), input)
				};
				assert_eq!(
					ft_res, lc_res,
					"ft_strchr({:?}, {}) result differs (ft, lc)",
					cs.to_string_lossy(), input
				);
			}
			#[test]
			fn strchr_basic1() {
				let cs = CString::new("despacaito").unwrap();
				let input = b'a' as i32;
				let ft_res = unsafe {
					ft_strchr(cs.as_ptr(), input)
				};
				let lc_res = unsafe {
					libc::strchr(cs.as_ptr(), input)
				};
				assert_eq!(
					ft_res, lc_res,
					"ft_strchr({:?}, {}) result differs (ft, lc)",
					cs.to_string_lossy(), input
				);
			}
			#[test]
			fn strchr_basic2() {
				let cs = CString::new("desopacito").unwrap();
				let input = b'o' as i32;
				let ft_res = unsafe {
					ft_strchr(cs.as_ptr(), input)
				};
				let lc_res = unsafe {
					libc::strchr(cs.as_ptr(), input)
				};
				assert_eq!(
					ft_res, lc_res,
					"ft_strchr({:?}, {}) result differs (ft, lc)",
					cs.to_string_lossy(), input
				);
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
					ft_memchr(cs.as_ptr() as *const c_void, input, 10);
				};
				let lc_res = unsafe {
					memchr(cs.as_ptr() as *const c_void, input, 10);
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