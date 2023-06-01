extern crate site_generator_derive;
extern crate maud;
extern crate clap;
extern crate chrono;

mod args;
mod site;
mod page;
mod pages;
mod page_generator;
mod templates;

fn main() -> std::io::Result<()> {
    let site = site::Site{ 
        blog_posts: pages::blog_posts(), 
        pages: pages::pages(), 
        dynamic_pages: vec![&pages::blog_index::BlogIndex],
        args: args::parse(),
    };

    site.generate_site()
}
