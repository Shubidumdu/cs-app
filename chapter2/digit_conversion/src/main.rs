fn main() {
    problem2_1();
    println!("-------------");
    problem2_2();
    println!("-------------");
    problem2_3();
    println!("-------------");
    problem2_4();
}

fn problem2_1() {
    let a = 0x25B9D2;
    let b = 0b1010111001001001;
    let c = 0xA8B3D;
    let d = 0b1100100010110110010110;

    println!("0x{0:x} = 0b{0:b}", a);
    println!("0b{0:b} = 0x{0:x}", b);
    println!("0x{0:x} = 0b{0:b}", c);
    println!("0b{0:b} = 0x{0:x}", d);
}

fn problem2_2() {
    let digits: [u32; 7] = [
        5,
        23,
        get_n_from_pow(32768),
        get_n_from_pow(0x2000),
        12,
        get_n_from_pow(64),
        get_n_from_pow(0x100),
    ];

    for digit in digits.into_iter() {
        println!(
            "n = {0}, 2^n = {1}, hex of 2^n = {1:x}",
            digit,
            2i32.pow(digit)
        )
    }
}

fn get_n_from_pow(n_pow: i32) -> u32 {
    (n_pow as f64).log2() as u32
}

fn problem2_3() {
    let digits: [u32; 10] = [
        0, 158, 76, 145, 0b10101110, 0b00111100, 0b11110001, 0x75, 0xBD, 0xF5,
    ];

    for digit in digits.into_iter() {
        println!(
            "Decimal = {0}, Binary = {0:08b}, Hexadecimal = 0x{0:02x}",
            digit
        );
    }
}

fn problem2_4() {
    println!("0x605c + 0x5 = 0x{:x}", 0x605c + 0x5);
    println!("0x605c - 0x20 = 0x{:x}", 0x605c - 0x20);
    println!("0x605c + 32 = 0x{:x}", 0x605c + 32);
    println!("0x60fa + 0x605c = 0x{:x}", 0x60fa - 0x605c);
}
