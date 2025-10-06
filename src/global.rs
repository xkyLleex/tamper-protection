use std::sync::{Mutex, Arc};
use lazy_static::lazy_static;

// Global color_mode parameter protected by a Mutex
lazy_static! {
    pub static ref COLOR_MODE: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));
}

pub fn get_color_mode() -> bool {
    *COLOR_MODE.lock().unwrap()
}

pub fn set_color_mode(value: bool) {
    let mut mode = COLOR_MODE.lock().unwrap();
    *mode = value;
}