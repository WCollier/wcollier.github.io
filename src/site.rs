use std::{fs, path::Path};
use maud::Markup;
use args::Args;
use page::{Page, PageKind, BlogPost};
use templates::{template, navbar_items};

pub(crate) struct Site<'a> {
    pub(crate) pages: &'a [Page],
    pub(crate) args: Args,
}

impl Site<'_> {
    const BLOG_BUILD_PATH: &str = "_site";

    pub fn generate_site(self) -> std::io::Result<()> {
        let navbar_items = navbar_items(&self);

        if Path::try_exists(Path::new(Self::BLOG_BUILD_PATH))? {
            fs::remove_dir_all(Self::BLOG_BUILD_PATH)?;
        }

        fs::create_dir_all(format!("{}/", Self::BLOG_BUILD_PATH))?;

        fs::create_dir_all(format!("{}/posts", Self::BLOG_BUILD_PATH))?;

        for page in self.pages {
            if let PageKind::BlogPost(BlogPost{ published: false, .. }) = page.kind {
                continue;
            }

            self.generate(&navbar_items, *page)?;
        }

        Ok(())
    }

    fn generate(&self, navbar_items: &Markup, page: Page) -> std::io::Result<()> {
        let route = format!("{}/{}.html", Self::BLOG_BUILD_PATH, page.route);
        let html = template(navbar_items, page.title, page.template(self)).into_string();

        println!("Written {route}");

        fs::write(route, html)
    }
}

