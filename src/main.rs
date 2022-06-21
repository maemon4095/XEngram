// ------ ------
//     Start
// ------ ------

mod pages;

fn main() {
    App::start("app", pages::index::init, pages::index::update, pages::index::view);
}
