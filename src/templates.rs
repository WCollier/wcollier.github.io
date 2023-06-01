use maud::{html, Markup, DOCTYPE};
use site::Site;
use page::Meta;
use page_generator::PageGenerator;

pub(crate) fn template(site: &Site, navbar_items: &Markup, page: &impl PageGenerator) -> Markup {
    let meta = page.meta();

    html! {
        (DOCTYPE)
        html lang="en-gb" {
            head { 
                meta charset="UTF-8";
                meta content="width=device-width,initial-scale=1.0" name="viewport";
                (page_style()) 
                title { (meta.title) " | William Collier" }
            }
            body {
                header { h1 { "William Collier's Website" } }
                navbar { (navbar_items) }
                h2 { (meta.title) }
                @if let Some(date_time) = meta.publish_date {
                    span class="published" { i { "Published: " (date_time.format("%Y-%m-%d").to_string()) } }
                }
                (page.template(site))
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
    let on_navbar_pages = pages_on_navbar(&site.pages);
    let on_navbar_dynamic_pages = pages_on_navbar(&site.dynamic_pages);
    let mut navbar_pages = [on_navbar_pages, on_navbar_dynamic_pages]
        .concat()
        .into_iter()
        .peekable();

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

fn pages_on_navbar(pages: &[impl PageGenerator]) -> Vec<Meta> {
    pages
        .iter()
        .map(|page| page.meta())
        .filter(|meta| meta.on_navbar)
        .collect::<Vec<Meta>>()
}
