#[allow(dead_code)]
fn hello() {
    println!("Hello, world!");
}
#[derive(Debug)]
pub trait Object {
    fn get_property(&self, name: &str) -> Option<Addr>;
    fn set_property(&mut self, name: &str, value: Addr);
}
