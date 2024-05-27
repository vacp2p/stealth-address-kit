.PHONY: deps clean example

deps:
	@cargo install cross --git https://github.com/cross-rs/cross.git --rev 1511a28
clean:
	@cargo clean
example:
	@cargo run -p stealth_address_kit_example
