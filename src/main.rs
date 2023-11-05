extern crate site_generator_derive;
extern crate maud;
extern crate clap;
extern crate chrono;

mod args;
mod site;
mod page;
mod pages;
mod templates;
mod navbar_link;
mod route;

fn main() -> std::io::Result<()> {
    let site = site::Site{ 
        pages: &[
            pages::how_my_static_site_generator_works(),
            pages::home_page(),
            pages::about(),
            pages::unknown_page(),
            pages::blog_index()
        ],
        navbar_links: &[
            navbar_link::home_link(),
            navbar_link::about_link(),
            navbar_link::posts_link(),
            navbar_link::cv_link()
        ],
        args: args::parse(),
    };

    site.generate_site()
}
