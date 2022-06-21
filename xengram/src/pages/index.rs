use seed::{prelude::*, *};

pub(crate) fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model { counter: 0 }
}

pub(crate) struct Model {
    counter: i32,
}

pub(crate) enum Msg {
    Increment,
}

pub(crate) fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
    }
}

pub(crate) fn view(model: &Model) -> Node<Msg> {
    div![
        C!["counter"],
        "This is a counter: ",
        button![model.counter, ev(Ev::Click, |_| Msg::Increment),],
    ]
}
