extern crate site_generator_derive;
extern crate maud;
extern crate clap;
extern crate chrono;

mod args;
mod site;
mod page;
mod pages;
mod templates;

fn main() -> std::io::Result<()> {
    let site = site::Site{ 
        pages: &[
            pages::example_post(),
            pages::index(),
            pages::about(),
            pages::unknown_page(),
            pages::BLOG_INDEX
        ],
        args: args::parse(),
    };

    site.generate_site()
}
