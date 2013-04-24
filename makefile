CWD = $(shell pwd)
DEP = $(CWD)/dep
RUSTC = $(DEP)/bin/rustc
LIBS = -L $(DEP)/SDL/lib -L $(DEP)/rust-opengles

SOURCES = src/main.rs

all: $(SOURCES)
	$(RUSTC) $(LIBS) $(SOURCES) -o guava

run: all
	$(CWD)/run.sh

stop:
	$(CWD)/stop.sh
