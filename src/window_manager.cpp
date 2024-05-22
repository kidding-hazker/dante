#include <X11/Xlib.h>
#include <iostream>
using namespace std;

namespace window_manager {
    static Display* display;
    static Window root;

    void setup() {
        root = XDefaultRootWindow(display);
        XSelectInput(display, root, SubstructureRedirectMask | SubstructureNotifyMask);
        XSync(display, 0);
        XGrabButton(display, Button1, 0, root, 0, ButtonPressMask, GrabModeAsync, GrabModeAsync, 0, 0);
    }
    void loop() {
        XEvent event;
        while (1) {
            XNextEvent(display, &event);
            switch (event.type) {
                case ButtonPress:
                    XAllowEvents(display, ReplayPointer, CurrentTime);
                    XSync(display, 0);
                    cout << "\e[1;33m[LOG]: EVENT: " << event.type << ": BUTTON PRESSED!\e[0m" << endl;
                    break;
                default:
                    cout << "\e[1;33m[LOG]: EVENT: " << event.type << "\e[0m" << endl;
                    break;
            }
            XSync(display, 0);
        }
    }
    void cleanup() {
        XCloseDisplay(display);
    }
}
