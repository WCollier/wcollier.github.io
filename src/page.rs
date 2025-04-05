use crate::route::Route;
use crate::site::Site;
use crate::templates;
use chrono::NaiveDate;
use maud::{html, Markup, PreEscaped};

pub(crate) type Body = &'static [&'static str];

#[derive(Copy, Clone, Debug)]
pub(crate) struct Post {
    pub(crate) published: bool,
    pub(crate) publish_date: NaiveDate,
    pub(crate) body: Body,
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum PageKind {
    StaticPage(Body),
    Post(Post),
    PostsIndex,
    HomePage(Body),
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
            PageKind::StaticPage(body) => html! { (Self::body_to_markup(body)) },
            PageKind::Post(post) => html! {
                span class="published" {
                    i {
                        "Published: " (post.format_publish_date())
                    }
                }
                (Self::body_to_markup(post.body))
            },
            PageKind::PostsIndex => templates::posts_index_template(site),
            PageKind::HomePage(body) => templates::home_page_template(site, body),
        }
    }

    pub(crate) fn body_to_markup(body: &'static [&'static str]) -> Markup {
        let body = body.join("\n");
        let options = comrak::ComrakOptions::default();
        let syntax_highlighter = comrak::plugins::syntect::SyntectAdapterBuilder::new()
            .theme("base16-ocean.light")
            .build();
        let plugins = comrak::ComrakPlugins {
            render: comrak::ComrakRenderPlugins {
                codefence_syntax_highlighter: Some(&syntax_highlighter),
                heading_adapter: None,
            },
        };
        let markdown = comrak::markdown_to_html_with_plugins(&body, &options, &plugins);

        html! {
            (PreEscaped(markdown))
        }
    }
}

impl Post {
    pub(crate) fn page_published(&self, dev_mode: bool) -> bool {
        self.published || dev_mode
    }

    pub(crate) fn format_publish_date(&self) -> String {
        self.publish_date.format("%Y-%m-%d").to_string()
    }
}
