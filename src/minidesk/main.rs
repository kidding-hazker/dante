pub mod window_manager;
pub mod cursorfont;
pub mod frame;

fn main() {
    unsafe {
        let mut window_manager = window_manager::WindowManager::init();
        window_manager.run();
    }
}
