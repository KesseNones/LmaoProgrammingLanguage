Execs = lmao lmaoc

default:$(Execs)

install:$(Execs)
	cp lmao /usr/bin/
	cp lmaoc /usr/bin/
uninstall:
	rm /usr/bin/lmao /usr/bin/lmaoc
clean:
	rm $(Execs)	

lmao: lmao.rs
	rustc -C opt-level=2 lmao.rs

lmaoc: lmaoc.rs
	rustc -C opt-level=2 lmaoc.rs

