.PHONY: run debug test

run:
	cargo run --release -- $(ARGS)

debug:
	cargo run --release -- --debug $(ARGS)

test:
	python3 tools/make_kladov_image.py
	cargo test --release
