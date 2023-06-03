use site_generator_derive::*;
use chrono::NaiveDate;
use page::{DATE_FORMAT, Page, Meta};

pub(crate) mod blog_index;

pub(crate) fn pages() -> Vec<Page> {
    vec![index(), about()]
}

pub(crate) fn blog_posts() -> Vec<Page> {
    vec![example_post()]
}

/// ### Welcome to my website
#[page(title = "Home", on_navbar = true)]
pub(crate) fn index() {}

/// ### My about page
#[page(title = "About", on_navbar = true)]
pub(crate) fn about() {}

/// ### Hello
/// #### Another one here
/// ```rust
/// fn hello() {}
/// ```
#[blog_post(title = "My Blog Post", published = true, publish_date = "2023-01-01")]
pub(crate) fn example_post() {}
