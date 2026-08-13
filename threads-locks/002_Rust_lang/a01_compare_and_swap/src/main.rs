use std::sync::atomic::{AtomicI32, Ordering};

fn compare_and_swap(ptr: &AtomicI32, old: i32, new: i32) -> u8 {
    match ptr.compare_exchange(old, new, Ordering::SeqCst, Ordering::SeqCst) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

fn main() {
    let value = AtomicI32::new(10);

    let ret = compare_and_swap(&value, 10, 20);

    println!("ret = {ret}");
    println!("value = {}", value.load(Ordering::SeqCst));
}
