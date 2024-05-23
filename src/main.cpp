#include "window_manager.cpp"

int main(int argc, char** argv) {
    window_manager::setup();
    window_manager::loop();
    return 0;
}
