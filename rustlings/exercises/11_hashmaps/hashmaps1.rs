// A basket of fruits in the form of a hash map needs to be defined. The key
// represents the name of the fruit and the value represents how many of that
// particular fruit is in the basket. You have to put at least 3 different
// types of fruits (e.g. apple, banana, mango) in the basket and the total count
// of all the fruits should be at least 5.

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // TODO: Declare the hash map.
    let mut basket = HashMap::new();

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);

    // TODO: Put more fruits in your basket.

    basket.insert(String::from("pear"), 5);
    basket.insert(String::from("apple"), 5);
    basket
}

/*
What the problem was
`basket` had no hash map declared and only one fruit (`banana`) in it — the
tests require at least 3 distinct fruit types and a total count of at least 5.

Why is this a problem?
`.insert()` needs a `HashMap` to call it on, and with only bananas, `basket.len()
>= 3` would fail even once the map existed.

Why does `let mut basket = HashMap::new();` plus more `.insert()` calls fix this?
`HashMap::new()` creates an empty map that `.insert(key, value)` can then add
entries to — here `String` keys mapped to `u32` counts. Adding `pear` and
`apple` on top of the given `banana` brings the type count to 3 and the total
count to 12, satisfying both tests. Unlike a `Vec`, a `HashMap` gives O(1)
average-time lookup by key instead of scanning — the right structure for "look
this up by name" rather than "process these in order".
*/

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
