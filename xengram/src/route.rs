pub struct Route<'a> {
    raw_route: &'a str,
}

impl<'a> Route<'a> {
    pub const fn new(route: &'a str) -> Route {
        Route { raw_route: route }
    }
}
