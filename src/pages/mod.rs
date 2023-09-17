use site_generator_derive::*;
use chrono::NaiveDate;
use page::{DATE_FORMAT, Page, Meta};

pub(crate) mod blog_index;

/// ### Welcome to my website
#[page(title = "Home", on_navbar = true)]
pub(crate) fn index() {}

/// ### My about page
#[page(title = "About", on_navbar = true)]
pub(crate) fn about() {}

/// ### This page cannot be found
#[page(title = "404", page_name = "404", on_navbar = false)]
pub(crate) fn unknown_page() {}

/// ### Hello
/// #### Another one here
/// ```rust
/// fn hello() {}
/// ```
#[blog_post(title = "My Blog Post", published = true, publish_date = "2023-01-01")]
pub(crate) fn example_post() {}

/// My website is generated using a custom static site generator written in Rust. There are no
/// reasons why a pre-existing generator couldn't be used, but I thought writing my own would be
/// interesting. While thinking about the code, I remembered that Rust has a feature called [rustdoc](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html) 
/// and, more importantly, the ability to generate HTML from markdown attached to various syntactical
/// elements such as function and struct declarations. With this in mind, I decided to write my own
/// static site generator that uses markdown attached to functions and a procedural macro. In
/// effect, I have created a static site generator where the pages are created in a similar fashion to rustdoc.
///
/// ### Example blog post
///
/// ```rust
/// /// ### My blog post
/// /// This is *fun*.
/// #[blog_post(title = "My Blog Post", published = true, publish_data = "2023-01-01")]
/// pub(crate) fn example_post(){}
/// ```
///
/// ### Why?
///
/// GitHub pages is a cool service and having a personal website could come into use at a later date.
/// Also, it is **free**. I can't complain about that. I decided that it would be novel to package as much of my site into a single binary. 
/// Rustdoc and [maud](https://maud.lambda.xyz) allow me to write and template pages all within `.rs` source files. 
/// Furthermore, I wanted use try GitHub Actions to see if I can use a Nix Flake to ease deployment to Github Pages.
/// Shockingly, my pipeline managed to pull down the correct dependencies, compile the site and deploy *first time*.
///
/// ### Advantages
///
/// * I can modify the site to my heart's content without having to create awkward hacks or
/// learning an unknown codebase.
///     * Complex logic or special pages (such as the blog index) are just code, not a templating.
/// language.
/// * I had fun.
/// 
/// ### Disadvantages
///
/// * I must now implement and maintain the static site generator myself :)
///
/// ### Conclusion
///
/// The purpose of this project wasn't to create a useful or necessarily practical site.
/// However, I am without constraints that a third-party solution may burden me with.
///
/// See the Git repository [here](https://www.github.com/WCollier/wcollier.github.io)
#[blog_post(title = "How my Static Site Generator works", published = true, publish_date = "2023-08-12")]
pub(crate) fn how_my_static_site_generator_works() {}
