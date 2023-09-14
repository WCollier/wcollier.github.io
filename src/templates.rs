use maud::{html, Markup, DOCTYPE};
use site::Site;
use page::{Page, PageKind, BlogPost};

pub(crate) const DATE_FORMAT: &str = "%Y-%m-%d";

const HIGHLIGHT_JS_CSS: &str = "//cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/styles/default.min.css";

const HIGHLIGHT_JS_SCRIPT: &str = "//cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/highlight.min.js";

pub(crate) fn blog_index_template(site: &Site) -> Markup {
    let mut blog_posts = site.pages
        .iter()
        .filter_map(|page| match page.kind {
            PageKind::BlogPost(blog_post) => Some((page, blog_post)),
            _ => None,
        })
        .collect::<Vec<(&Page, BlogPost)>>();

    blog_posts.sort_by_key(|(_page, blog_post)| std::cmp::Reverse(blog_post.publish_date));

    html! {
        ul {
            @for (page, blog_post) in blog_posts {
                @if blog_post.page_published(site.args.dev) {
                    li {
                        (blog_post.publish_date.format(DATE_FORMAT).to_string()) " - "
                        a href=(page.full_route()) { (page.title) }
                    }
                }
            }
        }
    }
}

pub(crate) fn template(navbar_items: &Markup, title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en-gb" {
            head { 
                meta charset="UTF-8";
                meta content="width=device-width,initial-scale=1.0" name="viewport";
                (page_style()) 
                link rel="stylesheet" href=(HIGHLIGHT_JS_CSS) {}
                script src=(HIGHLIGHT_JS_SCRIPT) {}
                script type="text/javascript" {
                    "hljs.highlightAll();"
                }
                title { (title) " | William Collier" }
            }
            body {
                header { h1 { "William Collier's Website" } }
                navbar { (navbar_items) }
                h2 { (title) }
                (content)
                footer { 
                    "William Collier | "
                    a href="https://github.com/WCollier" { "GitHub" }
                    " | 🐧"
                }
            }
        }
    }
}

pub(crate) fn navbar_items(site: &Site) -> Markup {
    let on_navbar_pages = pages_on_navbar(site.pages);
    let mut navbar_pages = on_navbar_pages.into_iter().peekable();

    html! {
        @while let Some(on_navbar_page) = navbar_pages.next() {
            a href=(on_navbar_page.full_route()) { (on_navbar_page.title) }

            @if navbar_pages.peek().is_some() {
                " | "
            }
        }
    }
}

fn page_style() -> Markup {
    let css = "
        body{
            margin:40px auto;
            max-width:650px;
            line-height:1.6;
            font-size:18px;
            color:#444;
            padding:0 10px
        }
        h1,h2,h3{
            line-height:1.2
        }
        .published {
            font-size: medium;
        }
    ";

    html! { 
        style type="text/css" { (css) } 
    }
}

fn pages_on_navbar(pages: &[Page]) -> Vec<&Page> {
    pages
        .iter()
        .filter(|page| page.on_navbar)
        .collect::<Vec<&Page>>()
}
