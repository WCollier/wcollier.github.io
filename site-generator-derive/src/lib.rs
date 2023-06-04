extern crate darling;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ExprLit, ItemFn, Lit, Meta, MetaNameValue};
use darling::{Error, FromMeta, ast::NestedMeta};

#[derive(Debug, Default, FromMeta)]
struct PageArgs {
    title: String,
    route: Option<String>,
    page_name: Option<String>,
    published: Option<bool>,
    publish_date: Option<String>,
    on_navbar: Option<bool>,
}

#[derive(Default, FromMeta)]
struct BlogPostArgs {
    title: String,
    published: bool,
    publish_date: String,
}

impl From<BlogPostArgs> for PageArgs {
    fn from(blog_post_args: BlogPostArgs) -> PageArgs {
        PageArgs {
            title: blog_post_args.title,
            route: Some("posts/".to_string()),
            page_name: None,
            published: Some(blog_post_args.published),
            publish_date: Some(blog_post_args.publish_date),
            on_navbar: Some(false),
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

fn create_page<T: FromMeta + Into<PageArgs>>(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(proc_macro2::TokenStream::from(args)) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(Error::from(e).write_errors()); }
    };
    let PageArgs{title, published, route, page_name, publish_date, on_navbar} = match T::from_list(&attr_args) {
        Ok(v) => v.into(),
        Err(e) => { return TokenStream::from(e.write_errors()); }
    };
    let fn_item = parse_macro_input!(input as ItemFn);
    let fn_ident = fn_item.sig.ident;
    let body = parse_doc_comments(&fn_item.attrs);
    let route = format!("{}{}", route.unwrap_or_default(), page_name.unwrap_or(fn_ident.to_string()));
    let published = published.unwrap_or(true);
    let publish_date = match publish_date {
        Some(publish_date) => {
            quote! { 
                {
                    let publish_date = NaiveDate::parse_from_str(&#publish_date, DATE_FORMAT)
                        .ok()
                        .expect("Could not parse date time");

                    Some(publish_date)
                }
            }
        }
        None => quote! { None }
    };
    let on_navbar = on_navbar.unwrap_or(false);

    quote! {
        pub(crate) fn #fn_ident() -> Page {
            Page{
                meta: Meta{
                    title: #title,
                    route: #route,
                    published: #published,
                    publish_date: #publish_date,
                    on_navbar: #on_navbar,
                },
                body: &[#(#body,)*],
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
