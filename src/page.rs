use std::path::Path;
use maud::{html, Markup, PreEscaped};
use chrono::NaiveDate;
use site::Site;
use page_generator::PageGenerator;

pub(crate) const DATE_FORMAT: &str = "%Y-%m-%d";

#[derive(Copy, Clone, Debug)]
pub(crate) struct Meta {
    pub title: &'static str,
    pub route: &'static str,
    pub published: bool,
    pub publish_date: Option<NaiveDate>,
    pub on_navbar: bool,
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct Page {
    pub(crate) meta: Meta,
    pub(crate) body: &'static [&'static str],
}

impl Meta {
    pub(crate) fn full_route(&self) -> String {
        let path = Path::new(self.route);

        match (path.parent(), path.file_name()) {
            (Some(parent), Some(file_name)) if file_name == "index" =>
                format!("/{}", parent.display()),

            _ => 
                format!("/{}.html", self.route)
        }
    }
}

impl Page {
    pub(crate) fn page_published(&self, dev_mode: bool) -> bool {
        self.meta.published || dev_mode
    }
}

impl PageGenerator for Page {
    fn meta(&self) -> Meta {
        self.meta
    }

    fn template(&self, _site: &Site) -> Markup {
        html! {
            @for line in self.body {
                (PreEscaped(markdown::to_html(line)))
            }
        }
    }
}

