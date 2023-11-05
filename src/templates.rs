use maud::{html, Markup, DOCTYPE};
use site::Site;
use page::{Page, BlogPost, Body};

pub(crate) const DATE_FORMAT: &str = "%Y-%m-%d";

const HIGHLIGHT_JS_CSS: &str = "//cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/styles/default.min.css";

const HIGHLIGHT_JS_SCRIPT: &str = "//cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/highlight.min.js";

pub(crate) fn blog_index_template(site: &Site) -> Markup {
    let mut blog_posts = site.ordered_blog_posts().collect::<Vec<(&Page, BlogPost)>>();

    blog_posts.sort_by_key(|(_page, blog_post)| std::cmp::Reverse(blog_post.publish_date));

    html! {
        ul {
            @for (page, blog_post) in blog_posts {
                li {
                    (blog_post.publish_date.format(DATE_FORMAT).to_string()) " - "
                    a href=(page.route) { (page.title) }
                }
            }
        }
    }
}

pub(crate) fn home_page_template(site: &Site, body: Body) -> Markup {
    let latest_blog_post = site.ordered_blog_posts().max_by_key(|(_page, blog_post)| blog_post.publish_date);

    html! {
        (Page::body_to_markup(body))

        @if let Some((page, _latest_blog_post)) = latest_blog_post {
            p {
                "Latest blog post: "
                a href=(page.route) { (page.title) }
            }
        }
    }
}

pub(crate) fn master_template(navbar_items: &Markup, title: &str, content: Markup) -> Markup {
    let full_title = format!("{title} | William Collier"); 

    html! {
        (DOCTYPE)
        html lang="en-gb" {
            head { 
                meta charset="UTF-8";
                meta content="width=device-width,initial-scale=1.0" name="viewport";
                meta property="og:title" content=(full_title);
                meta property="twitter:title" content=(full_title);
                (page_style()) 
                link rel="stylesheet" href=(HIGHLIGHT_JS_CSS) {}
                script src=(HIGHLIGHT_JS_SCRIPT) {}
                script type="text/javascript" {
                    "hljs.highlightAll();"
                }
                title { (full_title) }
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
    let mut navbar_links = site.navbar_links.iter().peekable();

    html! {
        @while let Some(navbar_link) = navbar_links.next() {
            a href=(navbar_link.route) { (navbar_link.title) }

            @if navbar_links.peek().is_some() {
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
