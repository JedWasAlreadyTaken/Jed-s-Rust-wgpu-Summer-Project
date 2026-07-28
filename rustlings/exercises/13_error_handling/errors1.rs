// TODO: This function refuses to generate text to be printed on a nametag if
// you pass it an empty string. It'd be nicer if it explained what the problem
// was instead of just returning `None`. Thankfully, Rust has a similar
// construct to `Option` that can be used to express error conditions. Change
// the function signature and body to return `Result<String, String>` instead
// of `Option<String>`.
fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // Empty names aren't allowed
        Err("Empty names aren't allowed".to_string())
    } else {
        Ok(format!("Hi! My name is {name}"))
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),
            Ok("Hi! My name is Beyoncé"),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new())
                .as_ref()
                .map_err(|e| e.as_str()),
            Err("Empty names aren't allowed"),
        );
    }
}

/*
What the Problem was
Without the Result method, the function could only return None for errors, so no information was given on what was wrong

fn generate_nametag_text(name: String) -> Option<String> {
    if name.is_empty() {
        None  // No explanation!
    } else {
        Some(format!("Hi! My name is {name}"))
    }
}

Why is this a Problem?
Option<T> is designed for cases where a value might not exist , but it doesnt explain why it's missing. When error messages are needed, option is insufficient as it doesnt carry data, so callers dont know what went wrong or how to fix it in this instance the comment that empty names aren't allowed isnt shown

Why does Result fix this?

fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err("Empty names aren't allowed")
    } else {
        Ok(format!("Hi! My name is {name}"))
    }
}

Result<T, E> provides two variants:
OK(T): Success case: Successful data Carried 
Err(E): Failed case: The error information Carried

Key differences:

Err("Empty names aren't allowed") carries the error message as a String
Callers can now see and handle specific error messages
The function explains what went wrong instead of silently returning None


Comparisons between Result<T, E> and Option<T>:

| Use Option<T>                           | Use Result<T, E>                               |
| --------------------------------------- | ---------------------------------------------- |
| Value might not exist                   | Operation might fail                           |
| No reason needed                        | Need to explain why                            |
| Simple presence/absence                 | Error context matters                          |
| Example: get(id) - item might not exist | Example: parse() - could fail for many reasons |
*/
