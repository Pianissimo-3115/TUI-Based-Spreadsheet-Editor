all: build

build:		# FOR AUTOGRADER
	cargo build --release --package spreadsheet

ext1:		# EXTENSIONS PART
	cargo run --package ext -- 999 18278

.PHONY: all build ext1