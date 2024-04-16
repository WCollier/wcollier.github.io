use maud::{html, Markup, PreEscaped};
use chrono::NaiveDate;
use site::Site;
use templates;
use route::Route;

pub(crate) type Body = &'static [&'static str];

#[derive(Copy, Clone, Debug)]
pub(crate) struct Post {
    pub(crate) published: bool,
    pub(crate) publish_date: NaiveDate,
    pub(crate) body: Body,
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum PageKind {
    StaticPage{ body: Body },
    Post(Post),
    PostsIndex,
    HomePage{ body: Body },
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct Page {
    pub(crate) kind: PageKind,
    pub(crate) title: &'static str,
    pub(crate) route: Route, 
}

impl Page {
    pub(crate) fn template(&self, site: &Site) -> Markup {
        match self.kind {
            PageKind::StaticPage{ body } => html! { (Self::body_to_markup(body)) },
            PageKind::Post(post) => html! {
                span class="published" {
                    i {
                        "Published: " (post.publish_date.format("%Y-%m-%d").to_string())
                    }
                }
                (Self::body_to_markup(post.body))
            },
            PageKind::PostsIndex => templates::posts_index_template(site),
            PageKind::HomePage{ body } => templates::home_page_template(site, body)
        }
    }

    pub(crate) fn body_to_markup(body: Body) -> Markup {
        PreEscaped(markdown::to_html(&body.join("\n")))
    }
}

impl Post {
    pub(crate) fn page_published(&self, dev_mode: bool) -> bool {
        self.published || dev_mode
    }
}
