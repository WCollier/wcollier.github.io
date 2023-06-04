use std::{fs, path::Path};
use maud::Markup;
use args::Args;
use page::Page;
use page_generator::PageGenerator;
use templates::{template, navbar_items};

pub(crate) struct Site<'a> {
    pub pages: &'a [Page],
    pub blog_posts: &'a [Page],
    pub dynamic_pages: &'a [&'static dyn PageGenerator],
    pub args: Args,
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

        for blog_post in self.blog_posts {
            if !blog_post.page_published(self.args.dev) {
                continue;
            }

            self.generate(&navbar_items, blog_post)?;
        }

        for page in self.pages {
            self.generate(&navbar_items, page)?;
        }

        for dynamic_page in self.dynamic_pages {
            self.generate(&navbar_items, dynamic_page)?;
        }

        Ok(())
    }

    fn generate(&self, navbar_items: &Markup, page: &impl PageGenerator) -> std::io::Result<()> {
        let route = format!("{}/{}.html", Self::BLOG_BUILD_PATH, page.meta().route);
        let html = template(self, navbar_items, page).into_string();

        println!("Written {route}");

        fs::write(route, html)
    }
}

