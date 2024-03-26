#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello(name: String) -> String {
        format!("Hello {name}")
    }
}

bindings::export!(Component with_types_in bindings);
