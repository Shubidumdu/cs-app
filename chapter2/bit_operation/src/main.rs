fn inplace_swap(x: *mut i32, y: *mut i32) {
    unsafe {
        println!("{:?}, {:?}", x.read(), y.read());
        *y ^= *x;
        *x ^= *y;
        *y ^= *x;
        println!("{:?}, {:?}", x.read(), y.read());
    }
}

fn main() {
    let x: *mut i32 = &mut 2;
    let y: *mut i32 = &mut 4;
    inplace_swap(x, y);
}
