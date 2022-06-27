use seed::{prelude::*, *};

pub enum Message {
    PageTransition(Url, Option<Url>),
}
