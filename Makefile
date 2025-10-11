default:
	rustc -C opt-level=2 lmao.rs
	rustc -C opt-level=2 lmaoc.rs
install: 
	rustc -C opt-level=2 lmao.rs
	rustc -C opt-level=2 lmaoc.rs
	cp lmao /usr/bin/
	cp lmaoc /usr/bin/
uninstall:
	rm /usr/bin/lmao /usr/bin/lmaoc
clean:
	rm lmao lmaoc	
