mod pervasive;

mod coordination_layer;
mod betree;
mod spec;

verus! {
fn some_stuff() {}

fn some_other_stuff() {
    some_stuff();
}
}
fn wtf() {

}

fn main() {
    wtf();
    println!("Hello, world!");
}
