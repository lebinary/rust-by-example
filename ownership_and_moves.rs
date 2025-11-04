fn main() {
    let heap_int = Box::new(3i32);  // int 3 on heap, clean automatically when goes out of scope
    let stack_int = 3i32;           // int 3 on stack

    // NOTE: Box pointer and Value in heap relationship is ALWAYS 1-to-1 mapping
    let a = Box::new(3i32);
    println!("a contains: {}", a);

    let b = a;
    // println!("a contains: {}", a); //  ERROR 'a' no longer owns anything!
}
