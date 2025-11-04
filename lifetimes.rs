fn main() {
    // SIMPLE RULE: Reference's TTL ≤ Data's TTL
    // TRICK: reference pattern should resemble a stack

    // CORRECT
    let x = 5i32;
    let y = &x;

    // INCORRECT
    let y;
    {
        let x = 5i32;
        y = &x;
    }

    // Meaning: x (with lifetime 'a) has TTL >= print_one func TTL
    fn print_one<'a>(x: &'a i32) {
        println!("`print_one`: x is {}", x);
    }

    // Mutable references are possible with lifetimes as well.
    fn add_one<'a>(x: &'a mut i32) {
        *x += 1;
    }

    // Multiple lifetimes, x and y TTLs >= function TTL
    fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("`print_multi`: x is {}, y is {}", x, y);
    }

    // Return the reference, note correct lifetime need to be passed
    fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }
}
