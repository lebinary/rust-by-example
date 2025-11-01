use std::convert::From;

fn main() {
    // conversion between primitives: use Cast
    let bool_to_int: bool = true;
    println!("Convert Bool to Int: {}", bool_to_int as i32);

    // conversion between primitives-nonprimitives: use Traits (From, Into)
    let my_str = "Hello World";
    let my_string = String::from(my_str);
    println!("Convert from str to String: {}", my_string);

    // converstion between nonprimitives or custom types: use Traits (From, Into)
    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(v: i32) -> Self {
            Number { value: v }
        }
    }

    let an_integer: i32 = 12;
    let from_number = Number::from(an_integer);
    println!(
        "Convert from int32 {i} to custom type Number {from:?}",
        i = an_integer,
        from = from_number
    );

    let into_number: Number = an_integer.into();
    println!(
        "impl From also automatically impl Into: {into:?}. IMPORTANT: the opposite direction is not true",
        into = into_number
    )
}
