use std::{fs, path::Path};
use maud::Markup;
use args::Args;
use page::{Page, PageKind, BlogPost};
use templates::{master_template, navbar_items};
use navbar_link::NavbarLink;

pub(crate) struct Site<'a> {
    pub(crate) pages: &'a [Page],
    pub(crate) navbar_links: &'a [NavbarLink],
    pub(crate) args: Args,
}

impl Site<'_> {
    const BLOG_BUILD_PATH: &str = "_site";

    pub(crate) fn generate_site(self) -> std::io::Result<()> {
        let navbar_items = navbar_items(&self);

        if Path::try_exists(Path::new(Self::BLOG_BUILD_PATH))? {
            fs::remove_dir_all(Self::BLOG_BUILD_PATH)?;
        }

        fs::create_dir_all(format!("{}/", Self::BLOG_BUILD_PATH))?;

        fs::create_dir_all(format!("{}/posts", Self::BLOG_BUILD_PATH))?;

        fs::create_dir_all(format!("{}/static", Self::BLOG_BUILD_PATH))?;

        for page in self.pages {
            if let PageKind::BlogPost(BlogPost{ published: false, .. }) = page.kind {
                continue;
            }

            self.generate(&navbar_items, *page)?;
        }

        Ok(())
    }

    pub(crate) fn ordered_blog_posts(&self) -> impl Iterator<Item = (&Page, BlogPost)> {
        self.pages
            .iter()
            .filter_map(move |page| match page.kind {
                PageKind::BlogPost(blog_post) if blog_post.page_published(self.args.dev) =>
                    Some((page, blog_post)),
                _ => None
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

