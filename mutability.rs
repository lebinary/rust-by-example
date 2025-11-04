#[derive(Clone, Copy)]
struct Book {
    author: &'static str, // ref to a string in read-only memory
    title: &'static str,
    year: u32,
}

fn borrow(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

fn publish(book: &mut Book) {
    book.year = 2025;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    let immuBook = Book {
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    }

    let mut muBook = immuBook; // deep copy bc of Copy trait, else its a "move"

    // Valid operations
    borrow(&immuBook);
    borrow(&muBook);
    publish(&muBook);

    // Invalid operaion: mutate an immutable
    // publish(&immuBook) ERROR: Cannot borrow an immutable object as mutable
}
