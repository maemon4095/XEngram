use crate::Message;
use seed::{prelude::*, *};

pub(crate) fn view() -> Node<Message> {
    div!["there's nothing at this address."]
}
