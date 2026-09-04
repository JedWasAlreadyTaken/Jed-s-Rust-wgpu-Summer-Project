// In this exercise, we want to express the concept of multiple owners via the
// `Rc<T>` type. This is a model of our solar system - there is a `Sun` type and
// multiple `Planet`s. The planets take ownership of the sun, indicating that
// they revolve around the sun.

use std::rc::Rc;

#[derive(Debug)]
struct Sun;

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Hi from {self:?}!");
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rc1() {
        let sun = Rc::new(Sun);
        println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

        let mercury = Planet::Mercury(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 2 references
        mercury.details();

        let venus = Planet::Venus(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 3 references
        venus.details();

        let earth = Planet::Earth(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 4 references
        earth.details();

        let mars = Planet::Mars(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 5 references
        mars.details();

        let jupiter = Planet::Jupiter(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 6 references
        jupiter.details();

        // TODO
        let saturn = Planet::Saturn(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 7 references
        saturn.details();

        // TODO
        let uranus = Planet::Uranus(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 8 references
        uranus.details();

        // TODO
        let neptune = Planet::Neptune(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 9 references
        neptune.details();

        assert_eq!(Rc::strong_count(&sun), 9);

        drop(neptune);
        println!("reference count = {}", Rc::strong_count(&sun)); // 8 references

        drop(uranus);
        println!("reference count = {}", Rc::strong_count(&sun)); // 7 references

        drop(saturn);
        println!("reference count = {}", Rc::strong_count(&sun)); // 6 references

        drop(jupiter);
        println!("reference count = {}", Rc::strong_count(&sun)); // 5 references

        drop(mars);
        println!("reference count = {}", Rc::strong_count(&sun)); // 4 references

        // TODO
        drop(earth);
        println!("reference count = {}", Rc::strong_count(&sun)); // 3 references

        // TODO
        drop(venus);
        println!("reference count = {}", Rc::strong_count(&sun)); // 2 references

        // TODO
        drop(mercury);
        println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

        assert_eq!(Rc::strong_count(&sun), 1);
    }
}

/*
What was the problem?

Six TODOs needed filling in. The first three (saturn, uranus, neptune) originally used
Rc::new(Sun) instead of Rc::clone(&sun), and one attempt used Rc::clone(&Sun)/Rc::clone(&&Sun) -
mixing up the type name Sun (capital, a struct/unit value) with the variable sun (lowercase,
the actual Rc<Sun> created on line 39). Rc::clone needs a &Rc<Sun> argument, so neither
&Sun nor &&Sun type-checked, and even a corrected Rc::new(Sun) would have created an
entirely separate Sun with its own independent reference count, unrelated to sun's count.
The last three TODOs were missing drop(...) calls entirely, needed to bring sun's reference
count down from 4 to 1 to match the comments and the final assert_eq!.

How do the fixes work?

Rc::clone(&sun) on lines 63, 68, and 73 increments sun's existing strong reference count
and hands back another Rc<Sun> pointing at the same underlying Sun value, rather than
allocating a new one - matching the pattern already used for mercury through jupiter, and
correctly bringing the count up to 7, 8, then 9. drop(earth), drop(venus), and
drop(mercury) explicitly end those bindings' ownership one at a time, each decrementing
sun's reference count by one and bringing it down to 3, then 2, then 1 - matching the
comments and letting the final assert_eq!(Rc::strong_count(&sun), 1) pass, since every
planet that once shared ownership of sun has now been dropped except sun itself.
*/
