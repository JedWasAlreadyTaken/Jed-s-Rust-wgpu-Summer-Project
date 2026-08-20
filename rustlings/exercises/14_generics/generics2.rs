// This powerful wrapper provides the ability to store a positive integer value.
// TODO: Rewrite it using a generic so that it supports wrapping ANY type.
struct Wrapper<T> {
    value: T
}

// TODO: Adapt the struct's implementation to be generic over the wrapped value.
impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value}
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
 /*
 What the problem was
 The Wrapper struct held a specific type of i32 therefore not generic
 
 How Wrapper<T>{Value: T} fixes this
 this means that the type help can be decided later, so a int of i32 or a str can be held in place of T
 
 How Impl<T> Wrapper<T> fixes this
 the imple T needs its own<T> type because the new method needs to workd for any T, not one specific one. if this was specific, the new takes only one set type which isnt what the problem wants
 
 */