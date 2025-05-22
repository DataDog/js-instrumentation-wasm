#[cfg(all(debug_assertions, target_arch = "wasm32"))]
#[allow(dead_code)]
pub fn debug_log(message: &str) {
    web_sys::console::log_1(&message.into());
}

#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
#[allow(dead_code)]
pub fn debug_log(message: &str) {
    println!("{}", message);
}

#[cfg(not(debug_assertions))]
#[allow(dead_code)]
pub fn debug_log(_message: &str) {}
