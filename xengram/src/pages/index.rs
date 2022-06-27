use crate::route::*;
use crate::Message;
use seed::{prelude::*, *};

pub const ROUTE: Route = Route::new("index");

pub fn init(_: Url, _: &mut impl Orders<Message>) -> Model {
    Model { counter: 0 }
}

pub struct Model {
    counter: i32,
}

pub enum Msg {
    Increment,
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Message>) {
    match msg {
        Msg::Increment => model.counter += 1,
    }
}

pub fn view(model: &Model) -> Node<Message> {
    div![
        C!["counter"],
        "This is a num: ",
        button![
            model.counter,
            ev(Ev::Click, |_| Message::Index(Msg::Increment)),
        ],
    ]
}
