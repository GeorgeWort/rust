#![feature(const_trait_impl, effects)] //~ WARN the feature `effects` is incomplete

struct S;
#[const_trait]
trait T {
    fn foo();
}

fn non_const() {}

impl const T for S {
    fn foo() { non_const() }
    //~^ ERROR cannot call non-const fn
}

fn main() {}
