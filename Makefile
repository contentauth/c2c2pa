OS := $(shell uname)

CFLAGS = -I. -Wall 
ifeq ($(OS), Darwin)
CFLAGS += -framework Security
endif

release:
	cargo b --release
	$(CC) $(CFLAGS) src/main.c -o target/c2c2pa -lc2c2pa -L./target/release 

run: release
	./target/c2c2pa