trait Licensed {
    fn licensing_info(&self) -> String {
        "Default license".to_string()
    }
}

struct SomeSoftware;
struct OtherSoftware;

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// TODO: Fix the compiler error by only changing the signature of this function.
fn compare_license_types(software1:impl Licensed,  software2:impl Licensed ) -> bool {
    software1.licensing_info() == software2.licensing_info()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        assert!(compare_license_types(SomeSoftware, OtherSoftware));
    }

    #[test]
    fn compare_license_information_backwards() {
        assert!(compare_license_types(OtherSoftware, SomeSoftware));
    }
}

/*
What was the problem
Both Software1 and software2's signatures in the compare_license_types function was undefined, leading to a syntaxe error

how does impl Licensed fix this?
this means that the concrete type doesnt matter, only as long as it impliments  Licensed, so .licensing_info() definitely  exist, therefore be used later for the comparison. This is also needed, as the tests use different type combinations, therfore a single hardcoded type would fail one of them 
*/