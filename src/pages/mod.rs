use site_generator_derive::*;
use chrono::NaiveDate;
use page::{Page, PageKind, BlogPost};

pub(crate) const BLOG_INDEX: Page = Page {
    kind: PageKind::BlogIndex,
    on_navbar: true,
    title: "Posts",
    route: "posts/index"
};

/// ### Welcome to my website
#[page(title = "Home", on_navbar = true)]
pub(crate) fn index() {}

/// ### My about page
#[page(title = "About", on_navbar = true)]
pub(crate) fn about() {}

/// ### This page cannot be found
#[page(title = "404", file_name = "404")]
pub(crate) fn unknown_page() {}

/// ### Hello
/// #### Another one here
/// ```rust
/// fn hello() {}
/// ```
#[blog_post(title = "My Blog Post", published = true, publish_date = "2023-01-01")]
pub(crate) fn example_post() {}
