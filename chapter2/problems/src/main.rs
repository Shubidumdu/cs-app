fn main() {
    // problem2_23();
    // problem2_27(3999999999, 3999999999);
    // problem2_32(33, 32);
    problem2_42();
}

fn fun1(w: u32) -> i32 {
    ((w << 24) >> 24) as i32
}

fn fun2(w: u32) -> i32 {
    (w << 24) as i32 >> 24
}

fn problem2_23() {
    let w1 = 0x00000076;
    println!("{:08x}", fun1(w1));
    println!("{:08x}", fun2(w1));

    let w2 = 0x87654321;
    println!("{:08x}", fun1(w2));
    println!("{:08x}", fun2(w2));

    let w3 = 0x000000C9;
    println!("{:08x}", fun1(w3));
    println!("{:08x}", fun2(w3));

    let w4 = 0xEDCBA987;
    println!("{:08x}", fun1(w4));
    println!("{:08x}", fun2(w4));
}

fn problem2_27(x: u32, y: u32) {
    let a = match x.checked_add(y) {
        Some(_) => true,
        None => false
    };

    println!("{a}");
}

fn problem2_32(x: u32, y: u32) {
    let a = match x.checked_sub(y) {
        Some(_) => true,
        None => false
    };

    println!("{a}");
}

fn problem2_42() {
    println!("{}", div16(-33));
    println!("{}", div16(32));
    println!("{}", div16(48));
}

fn problem2_44() {
}

fn div16(x: i32) -> i32 {
    let bias = (x >> 31) & 0xF;
    (x + bias) >> 4
}
