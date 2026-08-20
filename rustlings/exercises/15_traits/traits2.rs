trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement the trait `AppendBar` for a vector of strings.
// `append_bar` should push the string "Bar" into the vector.
impl AppendBar for Vec<String>{
    fn append_bar(mut self) -> Self{
        self.push(String::from("Bar"));
        self
    }

}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}

/*
what was the problem?
I orignally put the append_bar function inside of main which means it was a free standing nested function, and the self was not bound to anything but it should have been refering to the vector.
i also created another vec called v where the syntax was even wrong as a let cant sit in a impl block. Also the Vec<String> wasnt pushed in as i did v.push()so the self wasnt modified. Rather i should have pushed onto self not v

How Did impl solve this problem?
The AppendBar trait only declares that any implementing type must have an appendbar method, and the impl for Vec<String> is meant to fill a promise by making a new method which implements the appendBar trait of the append_bar method so calling this method now compiles as the compiler now can find the method defined for that type  
the function needs to be mutable so the self can have the mutator method of .push() to it  
the self.push is a statement so there's no value to return, as it just performs mutation, we need the singular self to give a final expression  to return the -> Self
*/