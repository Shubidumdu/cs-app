fn main() {
    problem2_1();
    println!("-------------");
    problem2_2();
}

fn problem2_1() {
    let a = 0x25B9D2;
    let b = 0b1010111001001001;
    let c = 0xA8B3D;
    let d = 0b1100100010110110010110;

    println!("{:b}", a);
    println!("{:x}", b);
    println!("{:b}", c);
    println!("{:x}", d);
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
        println!("n = {0}, 2^n = {1}, hex of 2^n = {1:x}", digit, 2i32.pow(digit))
    }
}

fn get_n_from_pow(n_pow: i32) -> u32 {
    (n_pow as f64).log2() as u32
}
