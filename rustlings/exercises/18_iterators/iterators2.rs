// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
  words
    .iter()
    .map(|word|{ capitalize_first(word)})
    .collect()
}

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    words
    .iter()
    .map(|word|{ capitalize_first(word)})
    .collect()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}

/*
What the problem was
For capitalize_first, there was a TODO to complete the Some(first) arm so it would return
the input string with its first character uppercased. For capitalize_words_vector and
capitalize_words_string, both were entirely unwritten (// ???), needing to apply
capitalize_first to every element of a &[&str] slice and collect the results, first into a
Vec<String>, then into a single combined String. Along the way I also made a syntax
mistake in capitalize_words_vector: I wrote .iter() followed by map(...) on the next line
without a leading dot, which isn't valid  map(...) looked like a call to a free-standing
function rather than the .map() method chained onto the iterator.

How do these fixes work?
first.to_uppercase().collect::<String>() + chars.as_str() uppercases the already-extracted
first character (using .collect::<String>() because uppercasing a single char can expand
into more than one character in Unicode, so to_uppercase() returns an iterator rather than
a single char), then appends chars.as_str(), which is whatever's left in the chars
iterator after .next() already consumed the first character, turned back into a &str. The
+ concatenates the two pieces into one String.

capitalize_words_vector and capitalize_words_string both use the same iterator chain:
.iter() gives an iterator over the slice (yielding &&str items, which coerce
automatically to &str when passed into capitalize_first), .map(...) transforms each
element by calling capitalize_first on it, and .collect() gathers the results - into a
Vec<String> for the first function, and directly into a single String for the second,
since Rust's collect() can build either depending on the function's return type. The
words in capitalize_words_string don't need extra spacing inserted between them because
the input slice already includes " " as its own element, so simply capitalizing and
concatenating everything in order produces the correctly spaced result.
*/