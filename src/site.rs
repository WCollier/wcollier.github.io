use crate::args::Args;
use crate::navbar_link::NavbarLink;
use crate::page::{Page, PageKind, Post};
use crate::templates::{master_template, navbar_items};
use maud::Markup;
use std::{fs, path::Path};

pub(crate) struct Site<'a> {
    pub(crate) pages: &'a [Page],
    pub(crate) navbar_links: &'a [NavbarLink],
    pub(crate) args: Args,
}

impl Site<'_> {
    const BLOG_BUILD_PATH: &'static str = "_site";

    pub(crate) fn generate_site(self) -> std::io::Result<()> {
        let navbar_items = navbar_items(&self);
        let copy_options = fs_extra::dir::CopyOptions::new().overwrite(true);

        if Path::try_exists(Path::new(Self::BLOG_BUILD_PATH))? {
            fs::remove_dir_all(Self::BLOG_BUILD_PATH)?;
        }

        fs::create_dir_all(format!("{}/", Self::BLOG_BUILD_PATH))?;

        fs::create_dir_all(format!("{}/posts", Self::BLOG_BUILD_PATH))?;

        fs_extra::dir::copy("static", Self::BLOG_BUILD_PATH, &copy_options)
            .expect("Failed to create static directory");

        println!("Written static directory");

        for page in self.pages {
            if let PageKind::Post(Post {
                published: false, ..
            }) = page.kind
            {
                continue;
            }

            self.generate(&navbar_items, *page)?;
        }

        Ok(())
    }

    pub(crate) fn ordered_posts(&self) -> impl Iterator<Item = (&Page, Post)> {
        self.pages.iter().filter_map(move |page| match page.kind {
            PageKind::Post(post) if post.page_published(self.args.dev) => Some((page, post)),
            _ => None,
        })
    }

    fn generate(&self, navbar_items: &Markup, page: Page) -> std::io::Result<()> {
        let html = master_template(navbar_items, page.title, page.template(self)).into_string();
        let route = page.route.file_path();

        if let Some(parent) = route.parent() {
            let route = format!("{}{}", Self::BLOG_BUILD_PATH, route.display());

            println!("Written {}", route);

            fs::create_dir_all(format!("{}{}", Self::BLOG_BUILD_PATH, parent.display()))?;

            fs::write(route, html)?;
        }

        Ok(())
    }
}
