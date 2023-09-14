use std::path::Path;
use maud::{html, Markup, PreEscaped};
use chrono::NaiveDate;
use site::Site;
use templates;

#[derive(Copy, Clone, Debug)]
pub(crate) struct BlogPost {
    pub(crate) published: bool,
    pub(crate) publish_date: NaiveDate,
    pub(crate) body: &'static [&'static str]
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum PageKind {
    StaticPage{ body: &'static [&'static str] },
    BlogPost(BlogPost),
    BlogIndex,
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct Page {
    pub(crate) kind: PageKind,
    pub(crate) on_navbar: bool,
    pub(crate) title: &'static str,
    pub(crate) route: &'static str
}

impl Page {
    pub(crate) fn full_route(&self) -> String {
        let path = Path::new(self.route);

        match (path.parent(), path.file_name()) {
            (Some(parent), Some(file_name)) if file_name == "index" =>
                format!("/{}", parent.display()),

            _ => 
                format!("/{}.html", self.route)
        }
    }

    pub(crate) fn template(&self, site: &Site) -> Markup {
        match self.kind {
            PageKind::StaticPage{ body } =>
                html! { (PreEscaped(markdown::to_html(&body.join("\n")))) },
            PageKind::BlogPost(blog_post) => html! {
                span class="published" {
                    i {
                        "Published: " (blog_post.publish_date.format("%Y-%m-%d").to_string())
                    }
                }
                (PreEscaped(markdown::to_html(&blog_post.body.join("\n"))))
            },
            PageKind::BlogIndex => templates::blog_index_template(site)
        }
    }
}

impl BlogPost {
    pub(crate) fn page_published(&self, dev_mode: bool) -> bool {
        self.published || dev_mode
    }
}
