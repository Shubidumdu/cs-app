fn inplace_swap(x: *mut i32, y: *mut i32) {
    if x == y {
        return;
    }
    unsafe {
        println!("{:?}, {:?}", x.read(), y.read());
        *y ^= *x;
        *x ^= *y;
        *y ^= *x;
        println!("{:?}, {:?}", x.read(), y.read());
    }
}

fn reverse_array(mut a: Vec<i32>, cnt: usize) {
    let mut first: usize = 0;
    let mut last: usize = cnt - 1;
    
    while first <= last {
        let first_ptr: *mut i32 = &mut a[first];
        let last_ptr: *mut i32 = &mut a[last];
        inplace_swap(first_ptr, last_ptr);
        first += 1;
        last -= 1;
    }

    println!("{:?}", a);
}

fn mask(x: u32) {
    for byte in (x & 0xff).to_be_bytes().into_iter() {
        print!("{:02x}", byte);
    }
    print!("\n");
    for byte in (x ^ (!0xff)).to_be_bytes().into_iter() {
        print!("{:02x}", byte);
    }
    print!("\n");
    for byte in (x | 0xff).to_be_bytes().into_iter() {
        print!("{:02x}", byte);
    }
    print!("\n");
}

fn bis(x: u8, m: u8) -> u8 {
    bic(x, m) | m
}

fn bic(x: u8, m: u8) -> u8 {
    x & !m
}

fn boor_or(x: u8, y: u8) -> u8 {
    bis(x, y)
}

fn boor_and(x: u8, y: u8) -> u8 {
    bic(bic(bis(x, y), bic(x, y)), bic(y, x))
}

fn boor_xor(x: u8, y: u8) -> u8 {
    bis(bic(x, y), bic(y, x))
}

fn main() {
    // let x: *mut i32 = &mut 2;
    // let y: *mut i32 = &mut 4;
    // inplace_swap(x, y);

    // let v = vec![1, 2, 3, 4, 5];
    // reverse_array(v, 5);
    
    // mask(0x87654321);

    println!("{:08b}", boor_or(0b11101110, 0b10001011)); // 0b11101111
    println!("{:08b}", boor_and(0b11101110, 0b10001011)); // 0b10001010
    println!("{:08b}", boor_xor(0b11101110, 0b10001011)); // 0b01100101
}
