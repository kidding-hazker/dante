use x11::xlib::{
    self, Cursor, Display, SubstructureNotifyMask, SubstructureRedirectMask, Window, XCloseDisplay,
    XCreateFontCursor, XDefaultRootWindow, XDefineCursor, XEvent, XMapWindow, XNextEvent,
    XOpenDisplay, XSelectInput,
};

use crate::cursorfont::XC_LEFT_PTR;

pub struct WindowManager {
    display: *mut Display,
    root: Window,
    cursor: Cursor,
    run: bool,
}

impl WindowManager {
    pub unsafe fn init() -> Self {
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
        XMapWindow(display, root);
        XSelectInput(
            display,
            root,
            SubstructureRedirectMask | SubstructureNotifyMask,
        );

        Self {
            display,
            root,
            cursor,
            run: true,
        }
    }
    pub unsafe fn run(&mut self) {
        let mut event: XEvent = XEvent { pad: [0; 24] };
        while self.run {
            XNextEvent(self.display, &mut event);
            match event.get_type() {
                xlib::KeyPress => println!("\x1b[1;35m[log] key press\x1b[0m"),
                xlib::ButtonPress => println!("\x1b[1;35m[log] button press\x1b[0m"),
                xlib::KeyRelease => println!("\x1b[1;35m[log] key release\x1b[0m"),
                xlib::ButtonRelease => println!("\x1b[1;35m[log] button release\x1b[0m"),
                xlib::CreateNotify => println!("\x1b[1;35m[log] create notify\x1b[0m"),
                xlib::MapRequest => {
                    println!("\x1b[1;34m[log] map request\x1b[0m");
                    self.map(event.map_request)
                }
                xlib::ConfigureRequest => {
                    println!("\x1b[1;34m[log] configure request\x1b[0m");
                    self.configure(event.configure)
                }
                xlib::MotionNotify => println!("\x1b[1;35m[log] motion notify\x1b[0m"),
                xlib::UnmapNotify => {
                    println!("\x1b[1;34m[log] unmap notify\x1b[0m");
                    self.unmap(event.unmap)
                }
                xlib::DestroyNotify => {
                    println!("\x1b[1;34m[log] destroy notify\x1b[0m");
                    self.destroy(event.destroy_window)
                }
                eventtype => println!(
                    "\x1b[1;35m[log] unknown event {{ type = {} }}\x1b[0m",
                    eventtype
                ),
            }
        }
        XCloseDisplay(self.display);
    }

    pub unsafe fn configure(&mut self, event: xlib::XConfigureEvent) {
        xlib::XMoveResizeWindow(self.display, event.window, event.x, event.y, event.width as u32, event.height as u32);
    }
    pub unsafe fn map(&mut self, event: xlib::XMapRequestEvent) {
        xlib::XMapWindow(self.display, event.window);
    }
    pub unsafe fn unmap(&mut self, event: xlib::XUnmapEvent) {
        xlib::XUnmapWindow(self.display, event.window);
    }
    pub unsafe fn destroy(&mut self, event: xlib::XDestroyWindowEvent) {
        xlib::XDestroyWindow(self.display, event.window);
    }
}
