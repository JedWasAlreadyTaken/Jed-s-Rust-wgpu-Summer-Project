// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.

mod delicious_snacks {
    // TODO: Add the following two `use` statements after fixing them.
    pub use self::fruits::PEAR as fruit;
     pub use self::veggies::CUCUMBER as veggies;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggies,
    );
}

/*
What the problem was
The two `use` statements in `delicious_snacks` had a TODO asking to "add them
after fixing them" — implying they started malformed (wrong path, missing
`pub`, or missing the `as` rename), while `main` already calls
`delicious_snacks::fruit` and `delicious_snacks::veggies` directly.

Why is this a problem?
`main` expects to reach the constants as `delicious_snacks::fruit` and
`delicious_snacks::veggies` — short names, directly on `delicious_snacks` — not
via the longer `delicious_snacks::fruits::PEAR` path, and not without some
mechanism exposing them out of the private inner modules `fruits`/`veggies` in
the first place.

Why does `pub use self::fruits::PEAR as fruit;` fix this?
It re-exports the private inner module's constant under a new name (`fruit`)
directly from `delicious_snacks`, without `fruits` itself needing to be `pub`.
The `as` keyword renames it on the way out; without it you'd still have to
write `delicious_snacks::fruits::PEAR`. This is a re-export — a common pattern
for controlling a crate's public API surface: `fruits` and `veggies` can stay
private implementation detail (there could be other constants in there you
don't want exposed), while `delicious_snacks` cherry-picks and renames just the
items it wants to present to the outside world.
*/
