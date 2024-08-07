.PHONY: deps clean example
deps:
	@cargo install cross --git https://github.com/cross-rs/cross.git --rev 1511a28
	@cargo install cbindgen

clean:
	@cargo clean

example:
	@cargo run --release -p stealth_address_kit_example

bench:
	@cargo bench --all-features
	cp -r target/criterion/** benchmarks/

expand:
	@cargo expand --all-features -p stealth_address_kit > expanded.rs

generate_c_bindings: expand
	@cbindgen --output stealth_address_kit.h --lang c expanded.rs

generate_nim_bindings: expand
	@nbindgen --output stealth_address_kit.nim expanded.rs
	@sed -i.bak "s/= int/= uint8/g" 'stealth_address_kit.nim'
