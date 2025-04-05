extern crate chrono;
extern crate clap;
extern crate comrak;
extern crate maud;
extern crate site_generator_derive;

mod args;
mod navbar_link;
mod page;
mod pages;
mod route;
mod site;
mod templates;

fn main() -> std::io::Result<()> {
    let site = site::Site {
        pages: &[
            pages::nix_links(),
            pages::how_my_static_site_generator_works(),
            pages::home_page(),
            pages::about(),
            pages::unknown_page(),
            pages::posts_index(),
        ],
        navbar_links: &[
            navbar_link::home_link(),
            navbar_link::about_link(),
            navbar_link::posts_link(),
            // navbar_link::cv_link()
        ],
        args: args::parse(),
    };

    site.generate_site()
}
