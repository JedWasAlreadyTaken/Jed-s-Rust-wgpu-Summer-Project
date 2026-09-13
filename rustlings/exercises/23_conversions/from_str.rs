// This is similar to the previous `from_into` exercise. But this time, we'll
// implement `FromStr` and return errors instead of falling back to a default
// value. Additionally, upon implementing `FromStr`, you can use the `parse`
// method on strings to generate an object of the implementor type. You can read
// more about it in the documentation:
// https://doc.rust-lang.org/std/str/trait.FromStr.html

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<u8>()
    ParseInt(ParseIntError),
}

// TODO: Complete this `FromStr` implementation to be able to parse a `Person`
// out of a string in the form of "Mark,20".
// Note that you'll need to parse the age component into a `u8` with something
// like `"4".parse::<u8>()`.
//
// Steps:
// 1. Split the given string on the commas present in it.
// 2. If the split operation returns less or more than 2 elements, return the
//    error `ParsePersonError::BadLen`.
// 3. Use the first element from the split operation as the name.
// 4. If the name is empty, return the error `ParsePersonError::NoName`.
// 5. Parse the second element from the split operation into a `u8` as the age.
// 6. If parsing the age fails, return the error `ParsePersonError::ParseInt`.
impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(",").collect();

        if parts.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }
        let name = parts[0];
        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }
        let age = match parts[1].parse() {
            Ok(age) => age,
            Err(e) => return Err(ParsePersonError::ParseInt(e)),
        };

        Ok(Person {
            name: name.to_string(),
            age,
        })
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>();
    println!("{p:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ParsePersonError::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }

    #[test]
    fn missing_age() {
        assert!(matches!("John,".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!("John,twenty".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(",".parse::<Person>(), Err(NoName | ParseInt(_))));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(NoName | ParseInt(_)),
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!("John,32,man".parse::<Person>(), Err(BadLen));
    }
}

/*
What was the problem?

The `from_str` implementation had several small mistakes stacked on top of each other:

1. `return ParsePersonError::BadLen` and `return ParsePersonError::NoName` returned the
   error variant directly instead of wrapping it in `Err(...)`, but the function's return
   type is `Result<Self, Self::Err>`, so a bare error variant doesn't type-check.
2. Several statements were missing their trailing semicolons (`return ...`, and
   `let name = parts[0]`), which is required for statements that aren't the function's
   final expression.
3. `OK(age) => age` used a capital-O `OK`, but the actual `Result` variant is `Ok`.
4. `ParsePersonError::ParseInt` is a tuple variant that wraps a `ParseIntError` - it can't
   be returned bare, and the original `Err(_)` arm discarded the underlying parse error
   instead of keeping it to wrap.
5. The function had no final expression for the success path, so nothing was ever
   returned when parsing succeeded.

How does the fix address this?

Each error path now returns `Err(ParsePersonError::Variant)`, matching the function's
`Result` return type. The `Ok(age) => age` arm uses the correctly-cased `Ok`, and the
`Err(e) => return Err(ParsePersonError::ParseInt(e))` arm keeps the original
`ParseIntError` (bound to `e`) so it can be wrapped into `ParsePersonError::ParseInt`
instead of losing that information. Finally, once `name` and `age` are both valid, the
function's last expression constructs `Ok(Person { name: name.to_string(), age })`,
returning the successfully parsed `Person`. This is also why `"Mark,20".parse::<Person>()`
works in `main` - implementing `FromStr` is what gives `str` the `parse::<Person>()`
method.
*/
