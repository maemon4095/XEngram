mod lib;
mod pages;
use seed::{prelude::*, *};

fn main() {
    App::start("app", init, update, view);
}

struct Model {
    page: Page,
}
#[non_exhaustive]
enum Message {
    Common(lib::common::Message),
    Index(pages::index::Msg),
}

#[non_exhaustive]
enum Page {
    Index(pages::index::Model),
    NotFound(),
}

impl Page {
    fn init(url: Url, orders: &mut impl Orders<Message>) -> Page {
        Page::Index(pages::index::init(url, orders))
    }

    fn not_found() -> Page {
        Page::NotFound()
    }
}

fn init(url: Url, orders: &mut impl Orders<Message>) -> Model {
    use pages::*;
    let page = Page::Index(index::init(url, orders));
    seed::log(url);
    Model { page }
}
fn update(msg: Message, model: &mut Model, orders: &mut impl Orders<Message>) {
    use pages::*;
    use Message::*;

    match (msg, &mut model.page) {
        (Common(message), ref mut p) => {
            use lib::common::Message::*;
            match message {
                PageTransition(url, _) => model.page = Page::init(url, orders),
            }
        }
        (Index(msg), Page::Index(ref mut m)) => {
            index::update(msg, m, orders);
        }
        _ => model.page = Page::not_found(),
    }
}
fn view(model: &Model) -> Node<Message> {
    use pages::*;
    match model.page {
        Page::Index(ref m) => index::view(m),
        _ => not_found::view(),
    }
}
