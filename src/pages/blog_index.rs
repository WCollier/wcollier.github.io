use maud::{Markup, html};
use chrono::NaiveDate;
use site::Site;
use page::{DATE_FORMAT, Page, Meta};
use page_generator::PageGenerator;

#[derive(Debug)]
pub(crate) struct BlogIndex;

impl BlogIndex {
    fn sort_blog_posts<'a>(&'a self, blog_posts: &'a [Page]) -> Vec<&Page> {
        let mut sorted_blog_posts_publish_dates = blog_posts
            .iter()
            .map(|blog_post| {
                let publish_date = blog_post.meta.publish_date
                    .expect("Blog post publish dates should always be defined");

                (blog_post, publish_date)
            })
            .collect::<Vec<(&Page, NaiveDate)>>();

        sorted_blog_posts_publish_dates.sort_by(|(_, a_publish_date), (_, b_publish_date)| a_publish_date.cmp(b_publish_date));

        sorted_blog_posts_publish_dates
            .into_iter()
            .map(|(blog_post, _)| blog_post)
            .collect::<Vec<&Page>>()
    }
}

impl PageGenerator for BlogIndex {
    fn meta(&self) -> Meta {
        Meta {
            title: "Posts",
            route: "posts/index",
            published: true,
            publish_date: None,
            on_navbar: true,
        }
    }

    fn template(&self, site: &Site) -> Markup {
        html! {
            ul {
                @for blog_post in self.sort_blog_posts(&site.blog_posts) {
                    @if blog_post.page_published(site.args.dev) {
                        li { 
                            @if let Some(publish_date) = blog_post.meta.publish_date {
                                (publish_date.format(DATE_FORMAT).to_string()) " - "
                            }

                            a href=(blog_post.meta.full_route()) { (blog_post.meta.title) } 
                        }
                    }
                }
            }
        }
    }
}
