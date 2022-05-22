extern crate libc;
use std::mem;

fn show_pointer() {
    unsafe {
        let ptr = libc::malloc(mem::size_of::<i32>());
        println!("{:?}", ptr);
    }
}

fn show_int_bytes(x: i32) {
    let bytes = x.to_ne_bytes();
    for byte in bytes.into_iter() {
        print!("{:02x} ", byte);
    }
    print!("\n");
}

fn show_float_bytes(x: f32) {
    let bytes = x.to_ne_bytes();
    for byte in bytes.into_iter() {
        print!("{:02x} ", byte);
    }
    print!("\n");
}

fn main() {
    let i = 12_345;
    show_int_bytes(i);
    show_float_bytes(i as f32);
    show_pointer();
}