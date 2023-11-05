use site_generator_derive::navbar_link;
use route::Route;

#[derive(Copy, Clone, Debug)]
pub(crate) struct NavbarLink {
    pub(crate) title: &'static str,
    pub(crate) route: Route, 
}

#[navbar_link(title = "Home", route = "/")]
pub(crate) fn home_link() {}

#[navbar_link(title = "About", route = "/about")]
pub(crate) fn about_link() {}

#[navbar_link(title = "Posts", route = "/posts")]
pub(crate) fn posts_link() {}

#[navbar_link(title = "CV", route = "/static/cv.pdf")]
pub(crate) fn cv_link() {}
