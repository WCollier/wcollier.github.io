extern crate darling;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ExprLit, ItemFn, Lit, Meta, MetaNameValue};
use darling::{Error, FromMeta, ast::NestedMeta};

#[derive(Clone, Debug)]
enum PageKind {
    StaticPage,
    BlogPost{
        published: bool,
        publish_date: String,
    }
}

#[derive(Clone, Debug)]
struct Page {
    kind: PageKind,
    on_navbar: Option<bool>,
    title: String,
    route: Option<String>,
    file_name: Option<String>,
}

#[derive(Debug, Default, FromMeta)]
struct PageArgs {
    on_navbar: Option<bool>,
    title: String,
    route: Option<String>,
    file_name: Option<String>,
}

#[derive(Default, FromMeta)]
struct BlogPostArgs {
    title: String,
    published: bool,
    publish_date: String,
}

impl From<PageArgs> for Page {
    fn from(page_args: PageArgs) -> Page {
        Page {
            kind: PageKind::StaticPage,
            on_navbar: page_args.on_navbar,
            title: page_args.title,
            route: page_args.route,
            file_name: page_args.file_name
        }
    }
}

impl From<BlogPostArgs> for Page {
    fn from(blog_post_args: BlogPostArgs) -> Page {
        Page {
            kind: PageKind::BlogPost{
                published: blog_post_args.published,
                publish_date: blog_post_args.publish_date,
            },
            on_navbar: Some(false),
            title: blog_post_args.title,
            route: Some("posts/".to_string()),
            file_name: None,
        }
    }
}

#[proc_macro_attribute]
pub fn page(args: TokenStream, input: TokenStream) -> TokenStream {
    create_page::<PageArgs>(args, input)
}

#[proc_macro_attribute]
pub fn blog_post(args: TokenStream, input: TokenStream) -> TokenStream {
    create_page::<BlogPostArgs>(args, input)
}

fn create_page<T: FromMeta + Into<Page>>(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(proc_macro2::TokenStream::from(args)) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(Error::from(e).write_errors()); }
    };
    let page = match T::from_list(&attr_args) {
        Ok(v) => v.into(),
        Err(e) => { return TokenStream::from(e.write_errors()); }
    };
    let fn_item = parse_macro_input!(input as ItemFn);
    let fn_ident = fn_item.sig.ident;
    let body = parse_doc_comments(&fn_item.attrs);
    let body = quote! { &[#(#body,)*] };
    let kind = match page.kind {
        PageKind::StaticPage => quote! { PageKind::StaticPage{ body: #body} },
        PageKind::BlogPost{ published, publish_date } => {
            quote! {
                {
                    let publish_date = NaiveDate::parse_from_str(&#publish_date, crate::templates::DATE_FORMAT)
                        .ok()
                        .expect("Could not parse date time");

                    PageKind::BlogPost(BlogPost{
                        published: #published,
                        publish_date: publish_date,
                        body: #body
                    })
                }
            }
        }
    };
    let on_navbar = page.on_navbar.unwrap_or_default();
    let page_title = page.title;
    let file_name = page.file_name.unwrap_or(fn_ident.to_string());
    let route = format!("{}{}", page.route.unwrap_or_default(), file_name);

    quote! {
        pub(crate) fn #fn_ident() -> Page {
            Page{
                kind: #kind,
                on_navbar: #on_navbar,
                title: #page_title,
                route: #route
            }
        }
    }
    .into()
}

fn parse_doc_comments(attrs: &[syn::Attribute]) -> Vec<String> {
    attrs
        .iter()
        .filter_map(|attr| {
            match &attr.meta {
                Meta::NameValue(MetaNameValue {
                    value: Expr::Lit(ExprLit { lit: Lit::Str(comment), .. }),
                    path,
                    ..
                }) if path.is_ident("doc") => Some(comment.value().trim().to_string()),
                _ => None,
            }
        })
        .collect()
}
