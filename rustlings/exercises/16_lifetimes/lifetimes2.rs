// Don't change this function.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // TODO: Fix the compiler error by moving one line.

    let string1 = String::from("long string is long");
    let result;
    let string2 = String::from("xyz");
    {
        result = longest(&string1, &string2);
    }
    println!("The longest string is '{result}'");
}
 /* 
 What the problem was
 there was an error saying that the borrowed string 2 doesnt live long enough outside the block as it is dropped at the end of the {} while still being borrowed

 how does moving the line fix this error
 dut to blocks only mattering for declarations inside of it rather than computation happening inside of it, moving the declaration of string2 allows for it to be dropped later 
I
 */