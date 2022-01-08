CLEAR = 

ifeq ($(OS),Windows_NT)
	CLEAR += cls
else
	UNAME_S := $(shell uname -s)
	ifeq ($(UNAME_S),Linux)
		CLEAR += clear
	endif
	ifeq ($(UNAME_S),Darwin)
		CLEAR += clear
	endif
endif

all: build clear run

clear:
	$(CLEAR)

run:
	./target/debug/tic-tac-toe.exe

build:
	cargo build