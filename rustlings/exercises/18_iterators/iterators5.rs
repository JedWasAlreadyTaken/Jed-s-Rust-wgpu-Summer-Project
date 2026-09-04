// Let's define a simple model to track Rustlings' exercise progress. Progress
// will be modelled using a hash map. The name of the exercise is the key and
// the progress is the value. Two counting functions were created to count the
// number of exercises with a given progress. Recreate this counting
// functionality using iterators. Try to not use imperative loops (for/while).

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if *val == value {
            count += 1;
        }
    }
    count
}

// TODO: Implement the functionality of `count_for` but with an iterator instead
// of a `for` loop.
fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    // `map` is a hash map with `String` keys and `Progress` values.
    // map = { "variables1": Complete, "from_str": None, … }
    map.values()
    .filter(|v| **v ==value)
    .count()
}

fn count_collection_for(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    let mut count = 0;
    for map in collection {
        for val in map.values() {
            if *val == value {
                count += 1;
            }
        }
    }
    count
}

// TODO: Implement the functionality of `count_collection_for` but with an
// iterator instead of a `for` loop.
fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    // `collection` is a slice of hash maps.
    // collection = [{ "variables1": Complete, "from_str": None, … },
    //               { "variables2": Complete, … }, … ]
   collection
   .iter()
   .flat_map(|map| map.values())
   .filter(|v| **v == value)
   .count()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_map() -> HashMap<String, Progress> {
        use Progress::*;

        let mut map = HashMap::new();
        map.insert(String::from("variables1"), Complete);
        map.insert(String::from("functions1"), Complete);
        map.insert(String::from("hashmap1"), Complete);
        map.insert(String::from("arc1"), Some);
        map.insert(String::from("as_ref_mut"), None);
        map.insert(String::from("from_str"), None);

        map
    }

    fn get_vec_map() -> Vec<HashMap<String, Progress>> {
        use Progress::*;

        let map = get_map();

        let mut other = HashMap::new();
        other.insert(String::from("variables2"), Complete);
        other.insert(String::from("functions2"), Complete);
        other.insert(String::from("if1"), Complete);
        other.insert(String::from("from_into"), None);
        other.insert(String::from("try_from_into"), None);

        vec![map, other]
    }

    #[test]
    fn count_complete() {
        let map = get_map();
        assert_eq!(count_iterator(&map, Progress::Complete), 3);
    }

    #[test]
    fn count_some() {
        let map = get_map();
        assert_eq!(count_iterator(&map, Progress::Some), 1);
    }

    #[test]
    fn count_none() {
        let map = get_map();
        assert_eq!(count_iterator(&map, Progress::None), 2);
    }

    #[test]
    fn count_complete_equals_for() {
        let map = get_map();
        let progress_states = [Progress::Complete, Progress::Some, Progress::None];
        for progress_state in progress_states {
            assert_eq!(
                count_for(&map, progress_state),
                count_iterator(&map, progress_state),
            );
        }
    }

    #[test]
    fn count_collection_complete() {
        let collection = get_vec_map();
        assert_eq!(
            count_collection_iterator(&collection, Progress::Complete),
            6,
        );
    }

    #[test]
    fn count_collection_some() {
        let collection = get_vec_map();
        assert_eq!(count_collection_iterator(&collection, Progress::Some), 1);
    }

    #[test]
    fn count_collection_none() {
        let collection = get_vec_map();
        assert_eq!(count_collection_iterator(&collection, Progress::None), 4);
    }

    #[test]
    fn count_collection_equals_for() {
        let collection = get_vec_map();
        let progress_states = [Progress::Complete, Progress::Some, Progress::None];

        for progress_state in progress_states {
            assert_eq!(
                count_collection_for(&collection, progress_state),
                count_collection_iterator(&collection, progress_state),
            );
        }
    }
}

/*
What was the problem?

count_iterator and count_collection_iterator both had only comments describing the shape
of their input, with no actual body - they needed to reproduce what count_for and
count_collection_for already did with imperative for loops, but using iterators instead.

count_for walks every value in a single HashMap<String, Progress> with a for loop,
incrementing a counter each time a value matches the target Progress. count_collection_for
does the same thing one level deeper: it loops over a slice of hash maps, and for each one,
loops over its values, incrementing the same counter on a match. Both rely on a mutable
count variable that gets updated imperatively across two nested loops in the collection
version.

count_iterator needed to produce the same result as count_for for a single HashMap,
and count_collection_iterator needed to produce the same result as count_collection_for
across a whole slice of HashMaps - both without writing an explicit for/while loop or a
manually incremented counter.

How does map.values().filter(|v| **v == value).count() fix count_iterator?

map is a single HashMap<String, Progress>. map.values() gives an iterator over &Progress,
one reference per value in the map - no flat_map is needed here, unlike the collection
version, because there's only one map to walk, not a collection of maps to flatten
together first.

.filter(|v| **v == value) works the same way as in count_collection_iterator: the closure
receives a reference to each iterator item. Since map.values() already yields &Progress,
the closure parameter v ends up typed as &&Progress - a reference to a reference. **v
dereferences it twice, back down to a plain Progress, so it can be compared against value
with ==.

.count() then consumes whatever's left after filtering and returns how many items passed,
as a usize.

So map.values().filter(|v| **v == value).count() is the iterator-based version of:
let mut count = 0;
for val in map.values() {
    if *val == value {
        count += 1;
    }
}
count

The only structural difference from count_collection_iterator is the missing flat_map
step, since count_iterator only ever walks one HashMap's values directly rather than
flattening values from many HashMaps into a single iterator first.

How does collection.iter().flat_map(|map| map.values()).filter(|v| **v == value).count() fix count_collection_iterator?

collection is a slice of hashmaps containing String keys and Progress values.

.iter() creates an iterator over the slice, giving us the &HashMap<String, Progress> for each mapping.

in the line .flat_map(|map| map.values())
|map| map.values() is a closure that:

takes a &HashMap<String, Progress> named map

returns map.values(), which is an iterator over &Progress

flat_map calls this closure for each map, then flattens all those inner iterators into one big iterator of &Progress.

this replaces the nested loop of 
for map in collection {
    for val in map.values() {
        // ...
    }
}

.filter(|v| **v == value)

|v| is a closure taking one argument v (here v: &&Progress)

**v == value keeps only those entries whose Progress equals value.

map.values() already yields &Progress, and .filter()'s closure receives a reference to each iterator item, so the closure parameter ends up being a reference to a &Progress which is actually &&Progress, which is why **v (dereferencing twice) is needed to get back to a plain Progress for the == comparison.

.count()
Consumes the iterator and returns how many items passed the filter, as a usize.

So that whole expression is the iterator-based version of:

let mut count = 0;
for map in collection {
    for val in map.values() {
        if *val == value {
            count += 1;
        }
    }
}
count
*/
