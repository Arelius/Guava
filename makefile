CWD=$(shell pwd)
RUSTC = $(CWD)/dep/bin/rustc

all: src/main.rs
	@echo $(RUSTC) $(.ALLSRC) -o $(.TARGET)
