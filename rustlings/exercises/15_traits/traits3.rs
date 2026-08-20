trait Licensed {
    // TODO: Add a default implementation for `licensing_info` so that
    // implementors like the two structs below can share that default behavior
    // without repeating the function.
    // The default license information should be the string "Default license".
    fn licensing_info(&self) -> String{String::from("Default license")}
}


struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}


impl Licensed for SomeSoftware {} // Don't edit this line.
impl Licensed for OtherSoftware {} // Don't edit this line.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = "Default license";
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
/*
What was the problem?
The Licensed trait only declared the licensing_info signature, ending in ; with no default body  this forces both structs to need the licensed info from the Licensed trait, however because of the emptiness of the default, the 2 structs wont compile, as the method doesnt actually exist anywhere 

How does String::from("Default license") fix this?
Firstly we are now providing a default body inside the trait itself, so  both structs can inherit the same implementation through their empty impl rather than writing their own 

*/