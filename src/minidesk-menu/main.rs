use x11::xlib;

fn main() {
    unsafe {
        let display = xlib::XOpenDisplay(0 as *const i8);
        if display.is_null() {
            panic!("\x1b[1;31m[error] could not open display\x1b[0m");
        }
        let root = xlib::XDefaultRootWindow(display);
        if root == 0 {
            panic!("\x1b[1;31m[error] could not get root window\x1b[0m");
        }
        let main = xlib::XCreateSimpleWindow(
            display,
            root,
            80,
            0,
            400,
            300,
            46,
            0x000000,
            0xffffff,
        );
        let mut event = xlib::XEvent { pad: [0; 24] };
        xlib::XMapWindow(display, main);

        while xlib::XNextEvent(display, &mut event) != 0 {
            println!("{:?}", event);
        }

        xlib::XDestroyWindow(display, main);
        xlib::XUnmapWindow(display, main);
        xlib::XCloseDisplay(display);
    }
}
