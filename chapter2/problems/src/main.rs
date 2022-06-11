fn main() {
    problem2_23();
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
