// TODO: Fix the compiler error about calling a private function.
mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

  pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}

/*
What the problem was
`main` called `sausage_factory::make_sausage()` from outside the module, but
`make_sausage` had no `pub` keyword — the exercise's title problem, "calling a
private function", per the TODO comment.

Why is this a problem?
Everything in Rust is private by default, at the level of the module it's
defined in — `pub` is an opt-in, not an opt-out. A function with no `pub`
can't be called from outside its own module (and its descendants), so
`main` (outside `sausage_factory`) calling it fails to compile.

Why does marking `make_sausage` `pub` fix this?
It explicitly opts the function into being callable from outside the module,
which is exactly what `main` needs. `get_secret_recipe` stays private (no
`pub`) and that's fine — it's only ever called from inside `sausage_factory`,
by `make_sausage`, and items in the same module can always see each other
regardless of visibility. Scanning for `pub` items tells you exactly what
outside code is allowed to depend on, without needing a separate interface
file.
*/
