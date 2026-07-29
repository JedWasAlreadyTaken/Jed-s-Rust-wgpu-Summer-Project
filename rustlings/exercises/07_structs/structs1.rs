struct ColorRegularStruct {
    // TODO: Add the fields that the test `regular_structs` expects.
    // What types should the fields have? What are the minimum and maximum values for RGB colors?
    green:u8,
    red:u8,
    blue:u8,
}

struct ColorTupleStruct(/* TODO: Add the fields that the test `tuple_structs` expects */
    u8,u8,u8
);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: Instantiate a regular struct.
         let green = ColorRegularStruct{ red: 0,green: 255,blue: 0};

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct.
        let green =  ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct.
         let unit_struct = UnitStruct;
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}

/*
What the problem was
`ColorRegularStruct` and `ColorTupleStruct` both started with empty/placeholder
field lists (just TODO comments), and none of the three test functions
instantiated their struct.

Why is this a problem?
The tests reference `green.red`, `green.green`, `green.blue` on a regular
struct instance, `green.0`/`.1`/`.2` on a tuple struct instance, and `{unit_struct:?}`
on a unit struct instance — none of that compiles if the structs have no fields
and are never constructed.

Why does this fix it?
Giving `ColorRegularStruct` three named `u8` fields (`red, green, blue` — `u8`
because RGB channels only need the range 0-255), giving `ColorTupleStruct`
three positional `u8` fields, and instantiating each (`ColorRegularStruct { red:
0, green: 255, blue: 0 }`, `ColorTupleStruct(0, 255, 0)`, `UnitStruct`) covers
all three struct flavors. A unit struct is useful purely as a marker/type —
something to implement a trait on when you don't need to store any data. A
tuple struct is a lightweight alternative to a regular struct when field names
would just repeat the type name; a regular struct is preferred once fields need
distinct, meaningful names.
*/
