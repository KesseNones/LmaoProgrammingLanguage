default:
	cargo build --release

install:
	cargo build --release
	sudo cp target/release/lmao /usr/bin/
	sudo cp target/release/lmaoc /usr/bin/
uninstall:
	rm -f /usr/bin/lmao /usr/bin/lmaoc
clean:
	rm -rf target

