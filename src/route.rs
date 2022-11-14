#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/single")]
    SinglePlay,
    #[at("/multi")]
    MultiPlay,
    #[not_found]
    #[at("/404")]
    NotFound,
}
