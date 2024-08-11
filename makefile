hello_expanded:
	cargo expand --bin=hello > ./src/bin/hello/main_expanded.rs

run_hello:
	cargo run --bin hello

run_guess:
	cargo run --bin guess

run_rustlog:
	cargo run --bin rustlog
