.PHONY: all release install clippy clean really_clean

all:
	cargo build

release:
	cargo build --release

install:
	cargo install --path .

clippy:
	cargo clippy

doc:
	cargo doc

clean:
	find . -name '*~' -delete

really_clean: clean
	rm -rf target
