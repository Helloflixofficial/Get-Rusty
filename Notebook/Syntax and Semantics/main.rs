use std::cell::{RefCell, RefMut};
use std::rc::Rc;
fn main() {
let shared_string = Rc::new(RefCell::new("Hello".to_string()));
{
let mut hello_world: RefMut<String> = shared_string.borrow_mut();
hello_world.push_str(" World");
}
println!("{}", shared_string.take());
}
