CWD = $(shell pwd)
DEP = $(CWD)/dep
RUSTC = $(DEP)/bin/rustc
LIBS = -L $(DEP)/SDL/lib

SOURCES = src/main.rs

all: $(SOURCES)
	$(RUSTC) $(LIBS) $(SOURCES) -o guava

run: all
	$(CWD)/guava
