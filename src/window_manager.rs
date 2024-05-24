use x11::xlib::{
    self,
    Cursor,
    Display,
    Window,
    XCloseDisplay,
    XCreateFontCursor,
    XDefaultRootWindow,
    XDefineCursor,
    XEvent,
    XNextEvent,
    XOpenDisplay,
    XSelectInput,
    SubstructureNotifyMask,
    SubstructureRedirectMask,
    XMapRequestEvent,
    XMapWindow,
    XUnmapEvent,
    XDestroyWindow,
};
use crate::cursorcont::XC_left_ptr;

static mut RUNNING: bool = true;
static mut DISPLAY: *mut Display = 0 as *mut Display;
static mut ROOTWIN: Window = 0;
static mut CURSOR: Cursor = 0;

pub unsafe fn init() {
    DISPLAY = XOpenDisplay(0 as *const i8);
    if DISPLAY.is_null() {
        panic!("\x1b[1;31m[error] could not open display\x1b[0m");
    }
    ROOTWIN = XDefaultRootWindow(DISPLAY);
    if ROOTWIN == 0 {
        panic!("\x1b[1;31m[error] could not get root window\x1b[0m");
    }
    CURSOR = XCreateFontCursor(DISPLAY, XC_left_ptr);
    if CURSOR == 0 {
        panic!("\x1b[1;31m[error] could not create cursor\x1b[0m");
    }

    XDefineCursor(DISPLAY, ROOTWIN, CURSOR);
    XSelectInput(DISPLAY, ROOTWIN, SubstructureRedirectMask | SubstructureNotifyMask);
}
pub unsafe fn run() {
    let mut event: XEvent = std::mem::zeroed();
    while RUNNING {
        XNextEvent(DISPLAY, &mut event);
        let eventtype = event.get_type();

        if eventtype == xlib::KeyPress{
            println!("\x1b[1;34m[log] key press\x1b[0m");
        } else if eventtype == xlib::ButtonPress{
            println!("\x1b[1;34m[log] button press\x1b[0m");
        } else if eventtype == xlib::KeyRelease{
            println!("\x1b[1;34m[log] key release\x1b[0m");
        } else if eventtype == xlib::ButtonRelease{
            println!("\x1b[1;34m[log] button release\x1b[0m");
        } else if eventtype == xlib::ConfigureRequest{
            println!("\x1b[1;34m[log] configure request\x1b[0m");
        } else if eventtype == xlib::MapRequest{
            println!("\x1b[1;34m[log] map request\x1b[0m");
            map(event.map_request);
        } else if eventtype == xlib::MotionNotify{
            println!("\x1b[1;34m[log] motion notify\x1b[0m");
        } else if eventtype == xlib::UnmapNotify{
            println!("\x1b[1;34m[log] unmap notify\x1b[0m");
            unmap(event.unmap);
        } else {
            println!("\x1b[1;34m[log] unknown event {{ type = {} }}\x1b[0m", eventtype);
        }
    }
    XCloseDisplay(DISPLAY);
}
pub unsafe fn kill() {
    RUNNING = false;
}

unsafe fn map(event: XMapRequestEvent) {
    XMapWindow(DISPLAY, event.window);
}
unsafe fn unmap(event: XUnmapEvent) {
    XDestroyWindow(DISPLAY, event.window);
}
