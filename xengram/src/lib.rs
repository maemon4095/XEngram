pub(crate) mod common {
    use seed::{prelude::*, *};

    pub(crate) enum Message {
        PageTransition(Url, Option<Url>),
    }
}
