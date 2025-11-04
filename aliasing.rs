struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 }

    let borrow_p1 = &point;
    let borrow_p2 = &point;

    println!("Point has coordinates: ({}, {}, {})", borrow_p1.x, borrow_p2.y, point.z);
    // let mut_borrow = &mut point; ERROR: cant mutable reference because immutable references are in scope
    println!("Scope of immutable reference ends here: ({}, {}, {})", borrow_p1.x, borrow_p2.y, point.z);

    let mutable_borrow = &mut point;
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;
    // let borrow_p3 = &point; ERROR: cant immutable reference because mutable reference is in scope
    println!("Scope of mutable reference ends here: ({}, {}, {})", mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);
}
