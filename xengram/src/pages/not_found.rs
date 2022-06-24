use crate::Message;
use seed::{prelude::*, *};

pub(crate) const url : &str = "404";

pub(crate) fn view() -> Node<Message> {
    div!["there is nothing at this address."]
}
