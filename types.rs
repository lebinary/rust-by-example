fn main() {
    /* Primitives */
    let logical: bool = true;

    let a_float: f64 = 1.0;
    let an_integer: i32 = 5; // or let an_integer = 5i32;

    let default_float = 1.0; // f64
    let default_integer = 5; // i32

    let to_be_inferred: i64 = 64;
    let mut inferred = to_be_inferred; // i64 inferred
                                       // inferred = 32i32; // ERROR: expects i64
    inferred = 32i64; // WORKS

    let mut mutable = 12;
    let mutable = true; // WORKS: overwrite by shadowing

    /* Compound types: Arrays and Tuples */
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];

    // tuples is collection values of different types
    let my_tuple = (1u32, 2u64, true, 4i32, -5f32);

    /* Custom types */
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let p: Point = Point { x: -1, y: 2 };
    println!("print point debug {:?}", p);
}
