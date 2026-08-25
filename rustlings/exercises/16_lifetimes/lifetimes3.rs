// Lifetimes are also needed when structs hold references.

// TODO: Fix the compiler errors about the struct.
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}

/*
What was the problem
there was a lifetime parameter needed for the Book struct to be used inside of main, as there was a error message expecting named lifetime parameter

How did 'a fix this
by adding <'a> into the struct it can allow for lifetime usage, the same with the type declaration of author and title

*/