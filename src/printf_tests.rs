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
	macro_rules! printf_tester {
		(normal $($test_name:ident, $format:expr;)+) => {
			$(
				rusty_fork_test! {
					#[test]
					fn $test_name() {
						let format = CString::new($format).unwrap();
						let stdout = BufferRedirect::stdout().unwrap();
						let mut ft_buf = String::new();
						let mut lc_buf = String::new();
						let ft_res = unsafe { ft_printf(format.as_ptr()) };
						stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
						let stdout = BufferRedirect::stdout().unwrap();
						let lc_res = unsafe { printf(format.as_ptr()) };
						unsafe { libc::fflush(libc_stdhandle::stdout() )};
						stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
						if ft_buf != lc_buf {
							let err_msg = [
								format!("\n\nft_printf({:?}", format)
								,") output differs:\n".to_string(),
								format!(
									"\n{: <10}{:?}\n{: <10}{:?}\n\n",
									"ft:", ft_buf,
									"lc:", lc_buf
								)
							].concat();
							panic!(err_msg);
						}
						if ft_res != lc_res {
							let err_msg = [
								format!("\n\nft_printf({:?}", format)
								,") return value differs:\n".to_string(),
								format!(
									"\n{: <10}{:?}\n{: <10}{:?}\n\n",
									"ft:", ft_res,
									"lc:", lc_res
								)
							].concat();
							panic!(err_msg);
						}
					}
				}
			)+
		};
		(vararg $($test_name:ident, $format:expr, $($arg:expr),*);+;) => {
			$(
				rusty_fork_test! {
					#[test]
					fn $test_name() {
						let format = CString::new($format).unwrap();
						let stdout = BufferRedirect::stdout().unwrap();
						let mut ft_buf = String::new();
						let mut lc_buf = String::new();
						let ft_res = unsafe { ft_printf(format.as_ptr() $(,$arg as usize)*) };
						stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
						let stdout = BufferRedirect::stdout().unwrap();
						let lc_res = unsafe { printf(format.as_ptr() $(,$arg as usize)*) };
						unsafe { libc::fflush(libc_stdhandle::stdout() )};
						stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
						if ft_buf != lc_buf {
							let err_msg = [
								format!("\n\nft_printf({:?}", format)
								$(
									, format!(", {:?}", $arg)
								)*
								,") output differs:\n".to_string(),
								format!(
									"\n{: <10}{:?}\n{: <10}{:?}\n\n",
									"ft:", ft_buf,
									"lc:", lc_buf
								)
							].concat();
							panic!(err_msg);
						}
						if ft_res != lc_res {
							let err_msg = [
								format!("\n\nft_printf({:?}", format)
								$(
									, format!(", {:?}", $arg)
								)*
								,") return value differs:\n".to_string(),
								format!(
									"\n{: <10}{:?}\n{: <10}{:?}\n\n",
									"ft:", ft_res,
									"lc:", lc_res
								)
							].concat();
							panic!(err_msg);
						}
					}
				}
			)+
		};
	}
	macro_rules! c_str_ptr {
		($x:expr) => {
			{
				let d = CString::new($x).unwrap();
				d.into_raw()
			}
		}
	}
	
	// test basic strings as input
	printf_tester! {
		normal
		hello_world, "hello, world!";
		generic, "wie dit leest is gek";
		random, "aslfjawejr[oaiweur [23u34[ 1i3po4i 1p2[i3 [po123 jl;,sadf;l";
		random2, "opiuytrefglk;jlghcxvbnm,.nbvkjiuoi";
		none_normal, "%";
	}
	// test basic varargs
	printf_tester! {
		vararg
		s_char, "hello,%cworld!", b'7';
		s_escape, "hello,%%world!", b'7';
		s_intd, "%despacito", 123;
		s_inti, "despac%ito", 123;
		s_hexl, "asdf%xasdf", 69;
		s_hexu, "asdf%Xasdf", 420;
		s_string, "if you read this you are %stupid", c_str_ptr!("stupid");
		s_string_null, "if you read this you are %stupid", 0;
		c_test1, "Hello, %%world!%c%c%s brbr", b'6', b'9', c_str_ptr!("yeet despacito lmao");
		c_test2, "%d%i%u", -69 as isize, 420, 0xffffffff as u32;
		c_test3, "hextest%x%X", 123, 456;
		c_test4, "flagtest: <%4d>", 123456;
		c_test5, "flagtest: <%4d>", 56;
		c_test6, "flagtest: <%4d>", 123;
		c_test7, "%p%p%small", 420, 69, c_str_ptr!("hello world");
		wtflag1, "%+-+-  +-+-  06d", 123;
		wtflag2, "%0 46d", 123;
		wtflag3, "% 046d", 123;
		wtflag4, "%+ 46d", 123;
		wtflag5, "% +46d", 123;
		flag_plus, "%+d", 123;
		flag_plus2, "%+d", -123 as isize;
		flag_plus_width, "%+46d", 123;
		flag_precision_1, "%.*d",
		none_vararg, "%", 123;
	}
	// intentional panics
	rusty_fork_test! {
		#[test]
		#[should_panic]
		fn should_segfault() {
			unsafe { ft_printf(0 as *const c_char ) };
		}
	}
}