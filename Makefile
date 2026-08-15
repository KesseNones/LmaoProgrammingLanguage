default:
	cargo build --release

install:
	cargo build --release
	sudo cp target/release/lmao /usr/bin/
uninstall:
	rm -f /usr/bin/lmao 
clean:
	rm -rf target

