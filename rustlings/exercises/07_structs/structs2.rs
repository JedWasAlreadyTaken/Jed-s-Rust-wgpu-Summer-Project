#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_order() {
        let order_template = create_order_template();

        // TODO: Create your own order using the update syntax and template above!
         let your_order = Order {
        name: String::from("Hacker in Rust"),
        count: 1,
        ..order_template
    };

        assert_eq!(your_order.name, "Hacker in Rust");
        assert_eq!(your_order.year, order_template.year);
        assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(your_order.made_by_email, order_template.made_by_email);
        assert_eq!(your_order.item_number, order_template.item_number);
        assert_eq!(your_order.count, 1);
    }
}

/*
What the problem was
`your_order` needed to be built from `order_template` with only `name` and
`count` changed, but the starting point was just a TODO comment — writing out
all seven fields by hand for a struct that's mostly a copy of an existing one
is exactly what struct update syntax is for.

Why is this a problem?
`Order` has seven fields. Constructing `your_order` field-by-field, copying five
of `order_template`'s values verbatim, is verbose and easy to get wrong (miss a
field, typo a value) compared to saying "same as the template, except these
two".

Why does `Order { name: ..., count: 1, ..order_template }` fix this?
The `..order_template` at the end fills in every field not explicitly listed by
copying/moving it from `order_template`. This keeps construction concise when
deriving a new value from an existing one with only a few fields changed. It
can move fields out of the source struct if they're not `Copy` — here
`order_template.name` is a `String`, but since `your_order` overrides `name`
itself, `order_template`'s `name` field is never touched, so `order_template`
stays fully usable afterward for the assertions.
*/
