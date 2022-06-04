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


fn compare_bit_operators(a: u8, b:u8) {
    println!("{:x}", a & b);
    println!("{:x}", a | b);
    println!("{:x}", !a | !b);
    println!("{:x}", a & !b);
}

fn check_equal(x: u8, y: u8) -> bool {
    // rust 컴파일러가 엄격해서 == 없이 안되넹..
    if (x & !y) == 0 {
        true
    } else {
        false
    }
}

fn bit_shift(a: u8)  {
    println!("==============0x{0:02x}:{0}==============", a);
    println!("a => Binary: 0b{0:08b}, Hex: 0x{0:02x}", a);
    println!("a << 2 => Binary: 0b{0:08b}, Hex: 0x{0:02x}", a << 2);
    println!("a >> 3 => Binary: 0b{0:08b}, Hex: 0x{0:02x}", a >> 3);
    // rust는 알아서 unsigned에 대해 arithmetic shift를, signed에 대해서는 logical shift를 합니다.
} 

fn main() {
    // let x: *mut i32 = &mut 2;
    // let y: *mut i32 = &mut 4;
    // inplace_swap(x, y);

    // let v = vec![1, 2, 3, 4, 5];
    // reverse_array(v, 5);
    
    // mask(0x87654321);

    // println!("{:08b}", boor_or(0b11101110, 0b10001011)); // 0b11101111
    // println!("{:08b}", boor_and(0b11101110, 0b10001011)); // 0b10001010
    // println!("{:08b}", boor_xor(0b11101110, 0b10001011)); // 0b01100101

    // compare_bit_operators(0x55, 0x46);

    // println!("{}", check_equal(0x14, 0x14));

    bit_shift(0xD4);
    bit_shift(0x64);
    bit_shift(0x72);
    bit_shift(0x44);
}
