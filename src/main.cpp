#include <X11/Xlib.h>
#include "window_manager.cpp"

int main(int argc, char** argv) {
    window_manager::setup();
    window_manager::loop();
    window_manager::cleanup();

    return 0;
}
