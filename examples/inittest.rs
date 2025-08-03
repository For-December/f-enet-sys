
use f_enet_sys::{enet_deinitialize, enet_initialize};

fn main() {
    println!("Starting test of enet bindings...");
    if unsafe { enet_initialize() } < 0 {
        panic!("Error on enet initialization.");
    }
    println!("Enet initialized.");
    unsafe {
        enet_deinitialize();
    }
    println!("Enet deinitialized.");
}
