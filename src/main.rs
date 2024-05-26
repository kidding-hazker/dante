pub mod window_manager;
pub mod cursorcont;
pub mod frame;
pub mod config;

fn main() {
    unsafe {
        window_manager::init();
        window_manager::run();
    }
}
