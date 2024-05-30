use x11::xlib::{
    self, Display, SubstructureNotifyMask, SubstructureRedirectMask, XCloseDisplay,
    XCreateFontCursor, XDefaultRootWindow, XDefineCursor, XEvent, XNextEvent, XOpenDisplay,
    XSelectInput,
};

pub mod config;
pub mod cursorfont;
pub mod frame;

use cursorfont::XC_LEFT_PTR;

fn main() {
    unsafe {
        let display = XOpenDisplay(0 as *const i8);
        if display.is_null() {
            panic!("\x1b[1;31m[error] could not open display\x1b[0m");
        }
        let root = XDefaultRootWindow(display);
        if root == 0 {
            panic!("\x1b[1;31m[error] could not get root window\x1b[0m");
        }
        let cursor = XCreateFontCursor(display, XC_LEFT_PTR);
        if cursor == 0 {
            panic!("\x1b[1;31m[error] could not create cursor\x1b[0m");
        }

        XDefineCursor(display, root, cursor);
        XSelectInput(
            display,
            root,
            SubstructureRedirectMask | SubstructureNotifyMask,
        );

        let mut event: XEvent = std::mem::zeroed();
        loop {
            XNextEvent(display, &mut event);
            let eventtype = event.get_type();

            match eventtype {
                xlib::KeyPress => println!("\x1b[1;35m[log] key press\x1b[0m"),
                xlib::ButtonPress => println!("\x1b[1;35m[log] button press\x1b[0m"),
                xlib::KeyRelease => println!("\x1b[1;35m[log] key release\x1b[0m"),
                xlib::ButtonRelease => println!("\x1b[1;35m[log] button release\x1b[0m"),
                xlib::ConfigureRequest => println!("\x1b[1;35m[log] configure request\x1b[0m"),
                xlib::MapRequest => {
                    println!("\x1b[1;34m[log] map request\x1b[0m");
                    map(display, event.map_request);
                }
                xlib::MotionNotify => println!("\x1b[1;35m[log] motion notify\x1b[0m"),
                xlib::UnmapNotify => {
                    println!("\x1b[1;34m[log] unmap notify\x1b[0m");
                    unmap(display, event.unmap);
                }
                xlib::DestroyNotify => {
                    println!("\x1b[1;34m[log] destroy notify\x1b[0m");
                    destroy(display, event.destroy_window);
                }
                _ => println!(
                    "\x1b[1;35m[log] unknown event {{ type = {} }}\x1b[0m",
                    eventtype
                ),
            }
        }
        XCloseDisplay(display);
    }
}

unsafe fn map(display: *mut Display, event: xlib::XMapRequestEvent) {
    xlib::XMapWindow(display, event.window);
}
unsafe fn unmap(display: *mut Display, event: xlib::XUnmapEvent) {
    xlib::XUnmapWindow(display, event.window);
}
unsafe fn destroy(display: *mut Display, event: xlib::XDestroyWindowEvent) {
    xlib::XDestroyWindow(display, event.window);
}
