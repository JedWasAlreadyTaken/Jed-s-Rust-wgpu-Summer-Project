// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterators() {
        let my_fav_fruits = ["banana", "custard apple", "avocado", "peach", "raspberry"];

        // TODO: Create an iterator over the array.
        let mut fav_fruits_iterator = my_fav_fruits.iter();

        assert_eq!(fav_fruits_iterator.next(), Some(&"banana"));
        assert_eq!(fav_fruits_iterator.next(), Some(&"custard apple")); // TODO: Replace `todo!()`
        assert_eq!(fav_fruits_iterator.next(), Some(&"avocado"));
        assert_eq!(fav_fruits_iterator.next(), Some(&"peach")); // TODO: Replace `todo!()`
        assert_eq!(fav_fruits_iterator.next(), Some(&"raspberry"));
        assert_eq!(fav_fruits_iterator.next(), None ); // TODO: Replace `todo!()`
    }
}

/*
What the problem was
For the mutable fav_fruits_iterator, there was a TODO to make an iterator over the array,
and in the tests there were 3 TODOs to test the expected result of the .next() method on
the fruits array. In this, I first went wrong where I called a nonexistent function named
fav_fruits_iterator(). In the tests, I made two separate mistakes: I repeated the previous
assertion's value instead of advancing to the next element in the array, and I compared
against a bare &str instead of wrapping the expected value in Some(&...), which didn't
match what .next() actually returns.

How does my_fav_fruits.iter() fix this?
Firstly, .iter() gives a helper object that allows for iteration, which needs to be
mutable, since it starts positioned before the first element, and each call to .next()
returns the current element and advances its internal position. The expected value needs
to match Option's Some variant, since .next() has to be able to say whether there's an
element here or not - Option<T> is the type used to express this, with Some(value) for
when there is something and None for when there isn't. The extra & inside Some(&"...") is
about the array's type specifically: the array holds &str values, and calling .iter() on
it produces references to each element rather than ownership of them, so each element
handed back is a reference to a &str - i.e. a &&str. This is why the expected values need
to be wrapped as Some(&"...").
*/