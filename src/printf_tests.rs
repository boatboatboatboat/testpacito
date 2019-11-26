use libc::{c_char};
use libc::printf;
use std::ffi::CString;
use std::io::Read;
use gag::BufferRedirect;

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
						unsafe { libc::setlocale(libc::LC_ALL, CString::new("en_US").unwrap().as_ptr()) };
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
									"\n{: <10}{:?}\n{: <10}{:?}\n\n{: <10}{:?}\n{: <10}{:?}\n\n",
									"ft:", ft_res,
									"lc:", lc_res,
									"ftout:", ft_buf,
									"lcout:", lc_buf
								)
							].concat();
							panic!(err_msg);
						}
					}
				}
			)+
		};
		(ignarg $($test_name:ident, $format:expr, $($arg:expr),*);+;) => {
			$(
				rusty_fork_test! {
					#[test]
					#[ignored]
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
									"\n{: <10}{:?}\n{: <10}{:?}\n\n{: <10}{:?}\n{: <10}{:?}\n\n",
									"ft:", ft_res,
									"lc:", lc_res,
									"ftout:", ft_buf,
									"lcout:", lc_buf
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
									"\n{: <10}{:?}\n{: <10}{:?}\n\n{: <10}{:?}\n{: <10}{:?}\n\n",
									"ft:", ft_res,
									"lc:", lc_res,
									"ftout:", ft_buf,
									"lcout:", lc_buf
								)
							].concat();
							panic!(err_msg);
						}
					}
				}
			)+
		};
	}
	macro_rules! csp {
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
		basic_1, "hello, world!";
		basic_2, "wie dit leest is gek";
		basic_3, "aslfjawejr[oaiweur [23u34[ 1i3po4i 1p2[i3 [po123 jl;,sadf;l";
		basic_4, "opiuytrefglk;jlghcxvbnm,.nbvkjiuoi";
		basic_empty_string, "";
		basic_5, "\x09\x0a\x0b\x0c\x0d";
		spec_null, "%";
		spec_null_width, "%1";
		spec_null_width2, "%2";
		spec_null_width3, "%9"; 
		spec_null_width4, "%12";
		spec_pc, "%1%";
		spec_pc_width2, "%2%";
		spec_pc_width3, "%9%";
		spec_pc_width4, "%12%";
		spec_pc_lj, "%-%";
		spec_pc_lj2, "%-2%";
		spec_pc_lj3, "%-4%";
		spec_pc_0pad, "%0%";
		spec_pc_0pad1, "%01%";
		spec_pc_0pad2, "%02%";
		spec_pc_0pad3, "%012%";
		spec_pc_basic, "%%";
		spec_pc_width, "%5%";
		spec_pc_lj_width, "%-5%";
		spec_pc_0pad_width, "%05%";
		spec_pc_lj_0pad_width, "%-05%";
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
		s_string, "if you read this you are %stupid", csp!("stupid");
		s_string_null, "if you read this you are %stupid", 0;
		c_test1, "Hello, %%world!%c%c%s brbr", b'6', b'9', csp!("yeet despacito lmao");
		c_test2, "%d%i%u", -69 as isize, 420, 0xffffffff as u32;
		c_test3, "hextest%x%X", 123, 456;
		c_test4, "flagtest: <%4d>", 123456;
		c_test5, "flagtest: <%4d>", 56;
		c_test6, "flagtest: <%4d>", 123;
		c_test7, "%p%p%small", 420, 69, csp!("hello world");
		wtflag1, "%-06d", 123;
		wtflag2, "%012d", 123;
		wtflag4, "%-12d", 123;
		flag_lj, "%-d", 123;
		flag_lj2, "%-d", -123 as isize;
		flag_lj_width, "%-12d", 123;
		flag_d_precision_1, "%.*d", 3, 4;
		flag_d_precision_2, "%.*.*d", 2, 3, 4;
		flag_d_precision_3, "%-.*d", 3, 4;
		flag_d_precision_4, "%0.*d", 3, 4;
		flag_d_precision_5, "%-0.*d", 3, 4;
		flag_d_precision_6, "%.*d", 3, 12;
		flag_d_precision_7, "%.*d", 3, 123;
		flag_d_precision_8, "%.*d", 3, 1234;
		flag_d_precision_9, "%.*d", 0, 1234;
		flag_d_precision_10, "%.*d", 0, 0;
		flag_d_width_1, "%*d", 2, 3;
		flag_d_width_2, "%*2d", 2, 3;
		flag_d_width_3, "%2*d", 2, 3;
		flag_d_width_4, "%*d", -4 as isize, 3;
		flag_d_width_5, "%*d", -1 as isize, 3;
		flag_d_width_6, "%*d", -4 as isize, 12;
		flag_d_lj,		"%-d",		12;
		flag_d_lj2,		"%-4d",		34;
		flag_d_lj3,		"%-10d",	56;
		flag_d_lj4,		"%-*d",		5,		78;
		flag_d_zp,		"%04d",		b'd',	910;
		flag_d_zp2,		"%-04d",	b'e',	1112;
		flag_i_precision_1, "%.*i", 3, 4;
		flag_i_precision_2, "%.*.*i", 2, 3, 4;
		flag_i_precision_3, "%-.*i", 3, 4;
		flag_i_precision_4, "%0.*i", 3, 4;
		flag_i_precision_5, "%-0.*i", 3, 4;
		flag_i_precision_6, "%.*i", 3, 12;
		flag_i_precision_7, "%.*i", 3, 123;
		flag_i_precision_8, "%.*i", 3, 1234;
		flag_i_precision_9, "%.*i", 0, 1234;
		flag_i_precision_10, "%.*i", 0, 0;
		flag_i_width_1, "%*i", 2, 3;
		flag_i_width_2, "%*2i", 2, 3;
		flag_i_width_3, "%2*i", 2, 3;
		flag_i_width_4, "%*i", -4 as isize, 3;
		flag_i_width_5, "%*i", -1 as isize, 3;
		flag_i_width_6, "%*i", -4 as isize, 12;
		flag_i_lj,		"%-i",		12;
		flag_i_lj2,		"%-4i",		34;
		flag_i_lj3,		"%-10i",	56;
		flag_i_lj4,		"%-*i",		5,		78;
		flag_i_zp,		"%04i",		b'd',	910;
		flag_i_zp2,		"%-04i",	b'e',	1112;
		flag_c_width, "%4c", b'a';
		flag_c_width2, "%1c", b'a';
		flag_c_width3, "%0c", b'a';
		flag_c_width4, "%12c", b'a';
		flag_c_width5, "%10c", b'a';
		flag_c_precision_1, "%.*c", 3, b'a';
		flag_c_precision_2, "%.*.*c", 2, 3, b'b';
		flag_c_precision_3, "%-.*c", 3, b'c';
		flag_c_precision_4, "%0.*c", 3, b'd';
		flag_c_precision_5, "%-0.*c", 3, b'e';
		flag_c_precision_6, "%.*c", 3, b'f';
		flag_c_precision_7, "%.*c", 3, b'g';
		flag_c_precision_8, "%.*c", 3, b'h';
		flag_c_precision_9, "%.*c", 0, b'i';
		flag_c_precision_10, "%.*c", 0, 0;
		flag_c_lj, "%-c", b'a';
		flag_c_lj2, "%-4c", b'b';
		flag_c_lj3, "%-10c", b'c';
		flag_c_lj4, "%-*c", 5, b'c';
		flag_c_zp, "%04c", b'd';
		flag_c_zp2, "%-04c", b'e';
		none_vararg, "%", 123;
	}
	// stolen from pft : regular
	printf_tester! {
		vararg
		spec_s_basic1, "only wear designer, %s.", csp!("esketit");
		spec_s_basic2, "roken is %s.", csp!("dodelijk");
		spec_s_basic3, "%soken is %s.", csp!("koken"), csp!("smakelijk");
		spec_s_basic5, "%s", csp!("skrt");
		spec_s_basic6, "%s", csp!("");
		spec_s_basic7, "de%sci%so", csp!("spa"), csp!("c");
		spec_s_basic8, "asdf %s asdf %s asdf", csp!("fdsa"), csp!("");
		spec_s_basic9, "asdf %s asdf %s asdf", csp!(""), csp!("fdsa");
		spec_s_basic10, "%s%s%s", csp!("abc"), csp!("defg"), csp!("hijkl");
		spec_s_width1, "%32s", csp!("abc");
		spec_s_width2, "%16s", csp!("yeet brbrb");
		spec_s_width3, "%3s", csp!("was is los");
		spec_s_width_lj1, "%-32s", csp!("abc");
		spec_s_width_lj2, "%-16s", csp!("yeet brbrb");
		spec_s_width_lj3, "%-3s", csp!("was is los");
		spec_s_precision_notrunc, "%.7s", csp!("hello");
		spec_s_precision_trunc, "%.3s", csp!("what the fuck");
		spec_s_precision_default, "%.s", csp!("abcdefgh");
		spec_s_precision_zero, "%.0s", csp!("despayeeto");
		spec_s_precision_2trunc, "^.3s%.2s", csp!("roken"), csp!("dodelijk");
		spec_s_precision_1trunc1, "^.3s%.12s", csp!("roken"), csp!("dodelijk");
		spec_s_precision_1trunc2, "^.12s%.3s", csp!("roken"), csp!("dodelijk");
		spec_s_precision_2trunc0, "^.12s%.12s", csp!("roken"), csp!("dodelijk");
		spec_s_precision_width_notrunc, "%7.5s", csp!("yolo");
		spec_s_precision_width_trunc, "%7.5s", csp!("bombastic");
		spec_s_precision_width_notrunc_lj, "%7.5-s", csp!("yolo");
		spec_s_precision_width_trunc_lj, "%7.5-s", csp!("bombastic");
		spec_s_precision_lj_width_notrunc_lj, "%7.5-s", csp!("yolo");
		spec_s_precision_lj_width_trunc_lj, "%7.5-s", csp!("bombastic");
		d_prec_fits_neg, "%.6d", -3 as isize;
	}

	const USABLE_STRINGS: [&str; 18] = [
		"hello world",
		"despacito",
		"esketit",
		"roken is dodelijk",
		"",
		"reeee",
		"heck",
		"oof",
		"-903i41po23k12;'lj3'm",
		"%s",
		"?????",
		"\n\n\n\n\n\n\n\n\n\n",
		"\t\n\t\n",
		"sdfalisefoiaweroiau weiorua oiwuer oiauweroi uawioeru aiuweoiruaoweraew",
		"e",
		"abcd",
		"AbCdEFGhIJKlMNoPqRs",
		"oof"
	];

	#[cfg(feature = "bonus")]
	const FLAGS: u32 = 16;
	#[cfg(feature = "bonus")]
	const ALL_THE_FLAGS: [char; 16] = [
		'-',
		'0',
		'.',
//		'*',
		'1',
		'2',
		'3',
		'4',
		'5',
		'6',
		'7',
		'8',
		'9',
		' ',
		'+',
		'#',
		'\''
	];

	#[cfg(not(feature = "bonus"))]
	const FLAGS: u32 = 12;
	#[cfg(not(feature = "bonus"))]
	const ALL_THE_FLAGS: [char; 12] = [
		'-',
		'0',
		'.',
//		'*',
		'1',
		'2',
		'3',
		'4',
		'5',
		'6',
		'7',
		'8',
		'9',
	];

	fn isdigit(c: char) -> bool {
		match c {
			'0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
			_ => false
		}
	}
	fn single_string_tester(format: String, arg: String) {
		#[cfg(feature = "fuzzertrace")]
		println!("ft_printf({:?}, {:?})", format, arg);
		let format = CString::new(format).unwrap();
		let arg = CString::new(arg).unwrap();
		let stdout = BufferRedirect::stdout().unwrap();
		let mut ft_buf = String::new();
		let mut lc_buf = String::new();
		let ft_res = unsafe { ft_printf(format.as_ptr(), arg.as_ptr()) };
		stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
		let stdout = BufferRedirect::stdout().unwrap();
		let lc_res = unsafe { printf(format.as_ptr(), arg.as_ptr()) };
		unsafe { libc::fflush(libc_stdhandle::stdout() )};
		stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
		if ft_buf != lc_buf {
			let err_msg = [
				format!("\n\nft_printf({:?}, {:?}", format, arg)
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
				format!("\n\nft_printf({:?}, {:?}", format, arg)
				,") return value differs:\n".to_string(),
				format!(
					"\n{: <10}{:?}\n{: <10}{:?}\n\n{: <10}{:?}\n{: <10}{:?}\n\n",
					"ft:", ft_res,
					"lc:", lc_res,
					"ftout:", ft_buf,
					"lcout:", lc_buf
				),
			].concat();
			panic!(err_msg);
		}
	}
	fn numerical_tester(format: String, arg: u32) {
		#[cfg(feature = "fuzzertrace")]
		println!("ft_printf({:?}, {:?})", format, arg);
		let format = CString::new(format).unwrap();
		let stdout = BufferRedirect::stdout().unwrap();
		let mut ft_buf = String::new();
		let mut lc_buf = String::new();
		let ft_res = unsafe { ft_printf(format.as_ptr(), arg) };
		stdout.into_inner().read_to_string(&mut ft_buf).unwrap();
		let stdout = BufferRedirect::stdout().unwrap();
		let lc_res = unsafe { printf(format.as_ptr(), arg) };
		unsafe { libc::fflush(libc_stdhandle::stdout() )};
		stdout.into_inner().read_to_string(&mut lc_buf).unwrap();
		if ft_buf != lc_buf {
			let err_msg = [
				format!("\n\nft_printf({:?}, {:?}", format, arg)
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
				format!("\n\nft_printf({:?}, {:?}", format, arg)
				,") return value differs:\n".to_string(),
				format!(
					"\n{: <10}{:?}\n{: <10}{:?}\n\n{: <10}{:?}\n{: <10}{:?}\n\n",
					"ft:", ft_res,
					"lc:", lc_res,
					"ftout:", ft_buf,
					"lcout:", lc_buf
				),
			].concat();
			panic!(err_msg);
		}
	}
	fn standalone_tester(format: String) {
		#[cfg(feature = "fuzzertrace")]
		println!("ft_printf({:?}, {:?})", format);
		let format = CString::new(format).unwrap();
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
					"\n{: <10}{:?}\n{: <10}{:?}\n\n{: <10}{:?}\n{: <10}{:?}\n\n",
					"ft:", ft_res,
					"lc:", lc_res,
					"ftout:", ft_buf,
					"lcout:", lc_buf
				),
			].concat();
			panic!(err_msg);
		}
	}
	use rand::{RngCore, thread_rng};
	fn generate_format_string(specifier: char, n: u8) -> String {
		let mut rng = thread_rng();
		let mut formatter = String::from("%");
		let mut digits = 0;
		for _ in 1..n {
			let rand = rng.next_u32() % FLAGS;
			let mut flag = ALL_THE_FLAGS[rand as usize];
			while isdigit(flag) && digits >= 3 {
				let rand = rng.next_u32() % FLAGS;
				flag = ALL_THE_FLAGS[rand as usize];
			}
			if isdigit(flag) {
				digits += 1;
			}
			formatter.push(flag);
		}
		formatter.push(specifier);
		formatter
	}
	// fuzzy test
	fn spec_s_fuzzer_standalone() {
		unsafe { libc::setlocale(libc::LC_ALL, CString::new("en_US").unwrap().as_ptr()) };
		let mut rng = thread_rng();
		for _ in 1..500 {
			for n in 1..12 {
				let rand = rng.next_u32() % 13;
				single_string_tester(
					generate_format_string('s', n),
					USABLE_STRINGS[rand as usize].to_string()
				);
			}
		}
	}
	fn spec_d_fuzzer_standalone() {
		unsafe { libc::setlocale(libc::LC_ALL, CString::new("en_US").unwrap().as_ptr()) };
		let mut rng = thread_rng();
		for _ in 1..500 {
			let rand = rng.next_u32();
			for n in 1..12 {
				numerical_tester(
					generate_format_string('d', n),
					rand
				);
			}
		}
	}
	fn spec_c_fuzzer_standalone() {
		unsafe { libc::setlocale(libc::LC_ALL, CString::new("en_US").unwrap().as_ptr()) };
		let mut rng = thread_rng();
		for _ in 1..500 {
			let rand = rng.next_u32() % 128;
			for n in 1..12 {
				numerical_tester(
					generate_format_string('c', n),
					rand
				);
			}
		}
	}
	fn spec_p_fuzzer_standalone() {
		unsafe { libc::setlocale(libc::LC_ALL, CString::new("en_US").unwrap().as_ptr()) };
		let mut rng = thread_rng();
		for _ in 1..500 {
			let rand = rng.next_u32();
			for n in 1..12 {
				numerical_tester(
					generate_format_string('p', n),
					rand
				);
			}
		}
	}
	fn spec_i_fuzzer_standalone() {
		unsafe { libc::setlocale(libc::LC_ALL, CString::new("en_US").unwrap().as_ptr()) };
		let mut rng = thread_rng();
		for _ in 1..500 {
			let rand = rng.next_u32();
			for n in 1..12 {
				numerical_tester(
					generate_format_string('i', n),
					rand
				);
			}
		}
	}
	fn spec_u_fuzzer_standalone() {
		unsafe { libc::setlocale(libc::LC_ALL, CString::new("en_US").unwrap().as_ptr()) };
		let mut rng = thread_rng();
		for _ in 1..500 {
			let rand = rng.next_u32();
			for n in 1..12 {
				numerical_tester(
					generate_format_string('u', n),
					rand
				);
			}
		}
	}
	fn spec_xlw_fuzzer_standalone() {
		unsafe { libc::setlocale(libc::LC_ALL, CString::new("en_US").unwrap().as_ptr()) };
		let mut rng = thread_rng();
		for _ in 1..500 {
			let rand = rng.next_u32();
			for n in 1..12 {
				numerical_tester(
					generate_format_string('x', n),
					rand
				);
			}
		}
	}
	fn spec_xup_fuzzer_standalone() {
		unsafe { libc::setlocale(libc::LC_ALL, CString::new("en_US").unwrap().as_ptr()) };
		let mut rng = thread_rng();
		for _ in 1..500 {
			let rand = rng.next_u32();
			for n in 1..12 {
				numerical_tester(
					generate_format_string('X', n),
					rand
				);
			}
		}
	}
	fn spec_percent_fuzzer_standalone() {
		unsafe { libc::setlocale(libc::LC_ALL, CString::new("en_US").unwrap().as_ptr()) };
		for _ in 1..500 {
			for n in 1..12 {
				standalone_tester(
					generate_format_string('%', n),
				);
			}
		}
	}
	rusty_fork_test! {
		#[test]
		#[ignore]
		fn spec_fuzzer_d_1() {
			spec_d_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_d_2() {
			spec_d_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_d_3() {
			spec_d_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_d_4() {
			spec_d_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_d_5() {
			spec_d_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_i_1() {
			spec_i_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_i_2() {
			spec_i_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_i_3() {
			spec_i_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_i_4() {
			spec_i_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_i_5() {
			spec_i_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_u_1() {
			spec_u_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_u_2() {
			spec_u_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_u_3() {
			spec_u_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_u_4() {
			spec_u_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_u_5() {
			spec_u_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_xlw_1() {
			spec_xlw_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_xlw_2() {
			spec_xlw_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_xlw_3() {
			spec_xlw_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_xlw_4() {
			spec_xlw_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_xlw_5() {
			spec_xlw_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_xup_1() {
			spec_xup_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_xup_2() {
			spec_xup_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_xup_3() {
			spec_xup_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_xup_4() {
			spec_xup_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_xup_5() {
			spec_xup_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_s_1() {
			spec_s_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_s_2() {
			spec_s_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_s_3() {
			spec_s_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_s_4() {
			spec_s_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_s_5() {
			spec_s_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_p_1() {
			spec_p_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_p_2() {
			spec_p_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_p_3() {
			spec_p_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_p_4() {
			spec_p_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_p_5() {
			spec_p_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_c_1() {
			spec_c_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_c_2() {
			spec_c_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_c_3() {
			spec_c_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_c_4() {
			spec_c_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_c_5() {
			spec_c_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_percent_1() {
			spec_percent_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_percent_2() {
			spec_percent_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_percent_3() {
			spec_percent_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_percent_4() {
			spec_percent_fuzzer_standalone();
		}
		#[test]
		#[ignore]
		fn spec_fuzzer_percent_5() {
			spec_percent_fuzzer_standalone();
		}
	}
	// intentional panics
	rusty_fork_test! {
		#[test]
		#[should_panic]
		fn should_segfault() {
			unsafe { ft_printf(0 as *const c_char ) };
		}
	}//
}