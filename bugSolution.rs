fn main() {
    let mut v = vec![1, 2, 3];
    let mut v2 = v.clone();
    let ptr = v2.as_mut_ptr();
    unsafe {
        *ptr = 10;
    }
    println!( "{:?}", v2);
} 