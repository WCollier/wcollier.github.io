use maud::{html, Markup, DOCTYPE};
use site::Site;
use page::{Page, Post, Body};

pub(crate) const DATE_FORMAT: &str = "%Y-%m-%d";

pub(crate) fn posts_index_template(site: &Site) -> Markup {
    let mut posts = site.ordered_posts().collect::<Vec<(&Page, Post)>>();

    posts.sort_by_key(|(_page, post)| std::cmp::Reverse(post.publish_date));

    html! {
        ul {
            @for (page, post) in posts {
                li {
                    (post.publish_date.format(DATE_FORMAT).to_string()) " - "
                    a href=(page.route) { (page.title) }
                }
            }
        }
    }
}

pub(crate) fn home_page_template(site: &Site, body: Body) -> Markup {
    let latest_post = site.ordered_posts().max_by_key(|(_page, post)| post.publish_date);

    html! {
        (Page::body_to_markup(body))

        @if let Some((page, _latest_post)) = latest_post {
            p {
                "Latest post: "
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
