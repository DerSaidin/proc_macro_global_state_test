use proc_macro_global_state::*;

proc_macro_global_state_add!("foo");

proc_macro_global_state_add!("bar");

proc_macro_global_state_add!("baz");

fn main() {
    let v = proc_macro_global_state_emit!();
    println!("Hello, world! {:?}", v);
}
