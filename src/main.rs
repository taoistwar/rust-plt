use std::fmt::Debug;

fn main() {
    println!("Hello, world!");
}

trait Object: Debug {
    // fn get_property(&self, name: &str) -> Option<Addr>;
    // fn set_property(&mut self, name: &str, value: Addr);
}
