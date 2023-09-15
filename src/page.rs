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
            PageKind::StaticPage{ body } =>
                html! { (self.body_to_markup(&body)) },
            PageKind::BlogPost(blog_post) => html! {
                span class="published" {
                    i {
                        "Published: " (post.publish_date.format("%Y-%m-%d").to_string())
                    }
                }

                (self.body_to_markup(&blog_post.body))
            },
            PageKind::PostsIndex => templates::posts_index_template(site),
            PageKind::HomePage{ body } => templates::home_page_template(site, body)
        }
    }

    fn body_to_markup(&self, body: &'static [&'static str]) -> Markup {
        let body = body.join("\n");
        let options = comrak::ComrakOptions::default();
        let syntax_highlighter = comrak::plugins::syntect::SyntectAdapterBuilder
            ::new()
            .theme("base16-ocean.light")
            .build();
        let plugins = comrak::ComrakPlugins{
            render: comrak::ComrakRenderPlugins{
                codefence_syntax_highlighter: Some(&syntax_highlighter),
                heading_adapter: None,
            }
        };

        PreEscaped(comrak::markdown_to_html_with_plugins(&body, &options, &plugins))
    }
}

impl Post {
    pub(crate) fn page_published(&self, dev_mode: bool) -> bool {
        self.published || dev_mode
    }
}
