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


fn show_int_bytes_binary(x: i32) {
    let bytes = x.to_ne_bytes();
    for byte in bytes.into_iter() {
        print!("{:08b} ", byte);
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

fn show_float_bytes_as_binary(x: f32) {
    let bytes = x.to_ne_bytes();
    for byte in bytes.into_iter() {
        print!("{:08b} ", byte);
    }
    print!("\n");
}


fn main() {
    // let i = 3510593.0;
    // show_int_bytes(i as i32);
    // show_float_bytes(i as f32);
    // show_pointer();
    problem_2_6();
}

fn problem_2_6() {
    let i = 2607352;
    show_int_bytes_binary(i);
    show_float_bytes_as_binary(i as f32); 
  // 0b00000000001001111100100011111000
    // 0b01001010000111110010001111100000
}
