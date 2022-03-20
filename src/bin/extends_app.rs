fn main() {
    let _ = some_fn("text");
}

fn some_fn(name: &str) -> Box<dyn View> {
    if name == "text" {
        Box::new(TextView {})
    } else {
        Box::new(Button {})
    }
}
trait View {}

struct Button {}
struct TextView {}

impl View for Button {}

impl View for TextView {}
