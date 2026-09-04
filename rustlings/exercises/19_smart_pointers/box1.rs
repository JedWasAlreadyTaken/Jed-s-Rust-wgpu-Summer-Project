// At compile time, Rust needs to know how much space a type takes up. This
// becomes problematic for recursive types, where a value can have as part of
// itself another value of the same type. To get around the issue, we can use a
// `Box` - a smart pointer used to store data on the heap, which also allows us
// to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the "cons list", a
// data structure frequently found in functional programming languages. Each
// item in a cons list contains two elements: The value of the current item and
// the next item. The last item is a value called `Nil`.

// TODO: Use a `Box` in the enum definition to make the code compile.
#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// TODO: Create an empty cons list.
fn create_empty_list() -> List {
    let emptyList =List::Nil;
    emptyList
}

// TODO: Create a non-empty cons list.
fn create_non_empty_list() -> List {
    let nonEmptyList = List::Cons(42, Box::new(List::Nil));
    nonEmptyList
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}

/*
What was the problem?

List::Cons originally held a List directly (i32, List) rather than Box<List>. Since Cons
is one of List's own variants, that made List a recursive type whose size depends on its
own size - Rust needs to know a type's exact size at compile time, but "the size of List"
would depend on "the size of List", which is impossible to resolve, so it wouldn't compile.
create_empty_list and create_non_empty_list also needed bodies to actually construct a Nil
and a Cons(42, ...) value respectively.

How does Box<List> fix this?

A Box<T> is a smart pointer - a fixed-size pointer to a value stored on the heap, regardless
of how large or recursive T is. Wrapping the inner List in Box<List> means Cons no longer
holds a List by value, it holds a pointer to one, and a pointer always has a known, fixed
size, no matter what it's pointing to. That breaks the self-referential size dependency:
the compiler can now compute List's size just from an i32 and a pointer-sized Box, without
needing to already know List's size to figure out List's size.

create_empty_list returns List::Nil directly, since an empty list is just the base case
with nothing else in it. create_non_empty_list returns List::Cons(42, Box::new(List::Nil)),
constructing one Cons node holding the value 42 and a boxed Nil as its next item - Box::new
is what actually allocates that inner List on the heap and gives back the Box<List>
pointing to it.
*/
