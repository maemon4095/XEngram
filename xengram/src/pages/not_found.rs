use crate::Message;
use seed::{prelude::*, *};

pub const URL: &str = "404";

pub fn view() -> Node<Message> {
    div!["there is nothing at this address."]
}
