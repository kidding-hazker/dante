src = src/main.cpp
lib = -lX11
xde = /usr/bin/Xephyr

build: $(src)
	g++ -o bin/minidesk $(src) $(lib)

run:
	make build
	xinit ./xinitrc -- $(xde) :100 -ac -screen 1280x720 -host-cursor

clean:
	rm -f bin/*
