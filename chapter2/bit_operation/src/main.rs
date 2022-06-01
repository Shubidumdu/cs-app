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

fn main() {
    // let x: *mut i32 = &mut 2;
    // let y: *mut i32 = &mut 4;
    // inplace_swap(x, y);
    let v = vec![1, 2, 3, 4, 5];
    reverse_array(v, 5);
}
