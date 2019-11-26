#!/bin/zsh
whence rustup > /dev/null
if [ $? -eq 1 ]; then
	curl https://sh.rustup.rs -sSf | sh -s -- -y
	rustup default nightly
fi
if [ "$1" = "printf" ]; then
	echo "#[macro_use] extern crate rusty_fork; mod printf_tests; fn main(){}" > src/main.rs
	if [ "$2" = "bonus" ]; then
		cargo test --features bonus
	elif [ "$2" = "fuzz" ]; then
		cargo test fuzzer -- --ignored
	elif [ "$2" = "fuzzbonus" ] || [ "$2" = "bonusfuzz" ]; then
		cargo test fuzzer --features bonus -- --ignored
	else
		cargo test
	fi
elif [ "$1" = "libft" ]; then
	echo "#[macro_use] extern crate rusty_fork; mod ft_tests; fn main(){}" > src/main.rs
	if [ "$2" = "protection" ]; then
		cargo test
	else
		cargo test -- --ignored
	fi
elif [ "$1" = "cfg" ]; then
	echo "fn main() { println!(\"cargo:rustc-link-search=$2\");}" > build.rs
else
	echo "./run_test.sh"
	echo "\tcfg: set library directory"
	echo "\t\tusage: ./run_test.sh cfg my_libft_directory/"
	echo "\tprintf: run printf tests"
	echo "\t\tbonus: include bonus tests"
	echo "\t\tfuzz: run fuzzer"
	echo "\t\tfuzzbonus: run fuzzer with bonus"
	echo "\tlibft: run libft tests"
	echo "\t\tprotection: check for protection"
fi