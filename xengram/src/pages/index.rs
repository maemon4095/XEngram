use crate::Message;
use seed::{prelude::*, *};

pub(crate) fn init(_: Url, _: &mut impl Orders<Message>) -> Model {
    Model { counter: 0 }
}

pub(crate) struct Model {
    counter: i32,
}

pub(crate) enum Msg {
    Increment,
}

pub(crate) fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Message>) {
    match msg {
        Msg::Increment => model.counter += 1,
    }
}

pub(crate) fn view(model: &Model) -> Node<Message> {
    div![
        C!["counter"],
        "This is a num: ",
        button![
            model.counter,
            ev(Ev::Click, |_| Message::Index(Msg::Increment)),
        ],
    ]
}
