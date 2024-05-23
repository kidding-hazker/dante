src = src/main.cpp
lib = -lX11
xde = /usr/bin/Xephyr

build: $(src)
	g++ -o bin/minidesk $(src) $(lib)

run:
	make build
	xinit ./xinitrc -- $(xde) :100 -ac -screen 800x600 -host-cursor
	make clean

clean:
	rm -f bin/* core
