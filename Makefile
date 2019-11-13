# PREFIX :=

all: redify
redify: release

release:
	cargo build --release

clean:
	rm -rf target/

install: all
	install target/release/redify

re: clean all
