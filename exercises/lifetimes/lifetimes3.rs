// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.



struct Book<'a> {
    author: &'a str,

    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith"); // 这里保存了 String
    let title = String::from("Fish Flying");

    let book = Book {
        author: &name, // 这里取 `&str` 引用
        title: &title,
    };

    println!("{} by {}", book.title, book.author);
}