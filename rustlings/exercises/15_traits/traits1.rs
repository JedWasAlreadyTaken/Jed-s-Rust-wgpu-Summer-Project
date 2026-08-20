// The trait `AppendBar` has only one function which appends "Bar" to any object
// implementing this trait.
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for the type `String`.
     fn append_bar(self) -> Self{self + "Bar"}
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {s}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), "FooBar");
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(String::from("").append_bar().append_bar(), "BarBar");
    }
}

/* What was the Problem

The AppendBar trait only declares that any implementing type must have an appendbar method, and the impl for string is meant to fill a promise but it is empty. therefore String claimed to impliment AppendBar but it didnt have the method inside it.

How does adding fn append_bar(self) -> Self fix this?
the contract of the trait is some function called append_bar, that takes self and returns Self. inside the impl for string had nothing in it, but when the function is added, the contract is now fufilled due to the signature matching the trait's shape

How does self + "Bar" fix the appending to any object
self in this case is a string moved by value and with the + operator used for String + &str, the String is consumed and the &str is appended making a new string at the end 
*/