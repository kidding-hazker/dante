pub mod window_manager;
pub mod cursorcont;

fn main() {
    unsafe {
        window_manager::init();
        window_manager::run();
    }
}
