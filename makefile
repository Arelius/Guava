CWD=$(shell pwd)
RUSTC = $(CWD)/dep/bin/rustc

SOURCES = src/main.rs

all: $(SOURCES)
	$(RUSTC) $(SOURCES) -o guava

run: all
	$(CWD)/guava
