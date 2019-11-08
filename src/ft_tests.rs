use libc::{size_t, c_char, c_void};
use std::ffi::CString;

#[link(name = "ft")]
extern {
	fn ft_atoi(str: *const c_char) -> i32;
	fn ft_bzero(s: *mut c_void, n: size_t);
	fn ft_calloc(count: size_t, size: size_t) -> *mut c_void;
	fn ft_strmapi(s: *const c_char, f: extern fn(u32, c_char) -> c_char) -> *mut c_char;
	fn ft_split(s: *const c_char, c: c_char) -> *mut *mut c_char;
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
			let real = 123456;
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
			let real = 42069;
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
			let real = i32::max_value();
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
			let real = i32::min_value();
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
			let real = 0;
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
			let real = -1;
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
			let real = 123;
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
			let real = 0;
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
			let real = 123;
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
			let real = -42069;
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
			let real = 987;
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
	/*
	** atoi end
	**
	** bzero start
	*/
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
	mod ft_strmapi {
		use super::*;
		rusty_fork_test!{
			#[test]
			fn test_basic_1() {
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
			fn test_null_f() {
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