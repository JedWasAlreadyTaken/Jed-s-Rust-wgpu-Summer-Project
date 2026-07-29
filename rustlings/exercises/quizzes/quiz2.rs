// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function as described above.
    // pub fn transformer(input: ???) -> ??? { ??? }

    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String>{
       

        let mut output: Vec<String> = vec![];
        for (string, command) in &input { 
          let transformed = match command{
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_owned(),
                Command::Append(n) => {
                    let repeated = "bar".repeat(*n);
                format!("{}{}", string, repeated)
                }
            };
            output.push(transformed) 
        } 
        output
    } 
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
     use crate::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}

/*
What the problem was
`transformer` was just a signature comment (`pub fn transformer(input: ???) ->
??? { ??? }`) with no real parameter types, return type, or body — and the test
module didn't yet import `transformer` to call it.

Why is this a problem?
The test builds a `Vec<(String, Command)>` and expects a `Vec<String>` back,
with each string transformed according to its paired `Command` — without a real
signature and matching logic, none of that has anywhere to go. And without a
`use` bringing `transformer` into the test module's scope, `transformer(input)`
in the test wouldn't resolve either.

Why does this implementation fix it?
`pub fn transformer(input: Vec<(String, Command)>) -> Vec<String>` takes
ownership of the input and returns a fresh `Vec<String>`, using `match` on each
`Command` variant to decide the transformation: `Uppercase` calls
`.to_uppercase()`, `Trim` calls `.trim()` then `.to_owned()` (since `.trim()`
alone returns a borrowed `&str`, and the output needs an owned `String`), and
`Append(n)` builds `"bar".repeat(n)` and appends it with `format!`.
`use crate::my_module::transformer;` in the test module pulls the function out
of the nested module into scope (`crate::` meaning "start from the crate
root"). This combines several sections at once: enums with data and `match`
destructuring them (section 08), owned vs. borrowed strings (section 09), and
collecting into a `Vec` (section 05).
*/
