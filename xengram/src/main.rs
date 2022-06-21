mod pages;
use seed::{prelude::*, *};

fn main() {
    App::start("app", Model::init, Model::update, Model::view);
}

struct Model {
    counter: i32,
}

enum Message {
    Increment,
}

impl Model {
    pub fn init(_: Url, _: &mut impl Orders<Message>) -> Self {
        Model { counter: 0 }
    }
    pub fn update(msg: Message, model: &mut Model, _: &mut impl Orders<Message>) {
        match msg {
            Message::Increment => model.counter += 1,
        }
    }
    pub fn view(model: &Model) -> Node<Message> {
        div![
            C!["counter"],
            "This is a counter: ",
            button![model.counter, ev(Ev::Click, |_| Message::Increment),],
        ]
    }
}
