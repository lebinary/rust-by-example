fn main() {
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Bob"),
        age: Box::new(20)
    }

    let Person { name: moved_name, age: ref borrowed_age } = person;

    println!("The person's name is {}", moved_name);
    println!("The person's age is {}", borrowed_age);
    //println!("The person struct is {:?}", person); ERROR: person is now only { age }, cant print
    println!("The person's age from person struct is {}", person.age);

    // NOTE:
    // - moved_name owns the String (moved from person.name)
    // - borrowed_age is a reference to person.age
    // - person.age still exists ✓
    // - person.name is gone ✗
}
