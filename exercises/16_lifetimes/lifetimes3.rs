// Lifetimes are also needed when structs hold references.

struct Book<'a, 'b> {
    author: &'a str,
    title: &'b str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}
