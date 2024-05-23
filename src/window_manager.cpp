#include <X11/Xlib.h>
#include <X11/Xutil.h>
#include <X11/cursorfont.h>
#include <iostream>
#include <vector>
#include "frame.cpp"
using namespace std;

namespace window_manager {
    static bool run = true;
    static Display* display;
    static Window root;
    static Cursor cursor;
    static vector<Frame> frames;

    void map(XMapRequestEvent* event) {
        XMapWindow(display, event->window);
    }
    void unmap(XUnmapEvent* event) {
        XDestroyWindow(display, event->event);
    }

    void setup() {
        if ((display = XOpenDisplay(0)) == 0) {
            cout << "\e[1;31m[ERROR]: FAILED TO OPEN DISPLAY\e[0m" << endl;
            exit(-1);
        }
        if ((root = XDefaultRootWindow(display)) == 0) {
            cout << "\e[1;31m[ERROR]: FAILED TO GET ROOT WINDOW\e[0m" << endl;
            exit(-1);
        }
        if ((cursor = XCreateFontCursor(display, XC_left_ptr)) == 0) {
            cout << "\e[1;31m[ERROR]: FAILED TO CREATE CURSOR\e[0m" << endl;
            exit(-1);
        }
    
        XDefineCursor(display, root, cursor);
        XSelectInput(display, root, SubstructureRedirectMask | SubstructureNotifyMask);
    }
    void loop() {
        XEvent event;
        while (run) {
            XNextEvent(display, &event);
            switch (event.type) {
                case KeyPress:
                    cout << "\e[1;32m[LOG]: EVENT: " << event.type << ": KEY PRESSED!\e[0m" << endl;
                    break;
                case ButtonPress:
                    cout << "\e[1;32m[LOG]: EVENT: " << event.type << ": BUTTON PRESSED!\e[0m" << endl;
                    break;
                case KeyRelease:
                    cout << "\e[1;32m[LOG]: EVENT: " << event.type << ": KEY RELEASED!\e[0m" << endl;
                    break;
                case ButtonRelease:
                    cout << "\e[1;32m[LOG]: EVENT: " << event.type << ": BUTTON RELEASED!\e[0m" << endl;
                    break;
                case ConfigureRequest:
                    cout << "\e[1;32m[LOG]: EVENT: " << event.type << ": CONFIGURE\e[0m" << endl;
                    break;
                case MapRequest:
                    cout << "\e[1;32m[LOG]: EVENT: " << event.type << ": MAP WINDOW\e[0m" << endl;
                    map(&event.xmaprequest);
                    break;
                case MotionNotify:
                    cout << "\e[1;32m[LOG]: EVENT: " << event.type << ": MOTION\e[0m" << endl;
                    break;
                case UnmapNotify:
                    cout << "\e[1;32m[LOG]: EVENT: " << event.type << ": UNMAP WINDOW\e[0m" << endl;
                    unmap(&event.xunmap);
                    break;
                default:
                    cout << "\e[1;32m[LOG]: EVENT: " << event.type << "\e[0m" << endl;
                    break;
            }
        }
        XCloseDisplay(display);
    }
    void kill() {
        run = false;
    }
}
