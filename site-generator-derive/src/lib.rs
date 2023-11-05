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
    },
    BlogIndex,
    HomePage
}

#[derive(Clone, Debug)]
struct Page {
    kind: PageKind,
    title: String,
    route: String,
    file_name: Option<String>,
}

#[derive(Debug, Default, FromMeta)]
struct PageArgs {
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
            title: page_args.title,
            route: page_args.route.unwrap_or("/".to_string()),
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
            title: blog_post_args.title,
            route: "/posts/".to_string(),
            file_name: None,
        }
    }
}

#[proc_macro_attribute]
pub fn page(args: TokenStream, input: TokenStream) -> TokenStream {
    create_page_from_args::<PageArgs>(args, input)
}

#[proc_macro_attribute]
pub fn blog_post(args: TokenStream, input: TokenStream) -> TokenStream {
    create_page_from_args::<BlogPostArgs>(args, input)
}

#[proc_macro_attribute]
pub fn blog_index(_args: TokenStream, input: TokenStream) -> TokenStream {
    let blog_index = Page{
        kind: PageKind::BlogIndex,
        title: "Posts".to_string(),
        route: "/posts".to_string(),
        file_name: Some("/index".to_string())
    };

    create_page(blog_index, input)
}

#[proc_macro_attribute]
pub fn home_page(_args: TokenStream, input: TokenStream) -> TokenStream {
    let home_page = Page{
        kind: PageKind::HomePage,
        title: "Home".to_string(),
        route: "/".to_string(),
        file_name: Some("index".to_string())
    };

    create_page(home_page, input)
}

#[derive(Debug, Default, FromMeta)]
struct NavbarLinkArgs {
    title: String,
    route: String,
}

#[proc_macro_attribute]
pub fn navbar_link(args: TokenStream, input: TokenStream) -> TokenStream {
    let fn_item = parse_macro_input!(input as ItemFn);
    let fn_ident = fn_item.sig.ident;

    match NestedMeta::parse_meta_list(proc_macro2::TokenStream::from(args)) {
        Ok(attr_args) => match NavbarLinkArgs::from_list(&attr_args) {
            Ok(NavbarLinkArgs{title, route}) => quote!{
                pub(crate) fn #fn_ident() -> NavbarLink {
                    NavbarLink{ title: #title, route: Route(#route) }
                }
            }
            .into(),
            Err(e) => TokenStream::from(e.write_errors())
        },
        Err(e) => TokenStream::from(Error::from(e).write_errors())
    }
}

fn create_page_from_args<T: FromMeta + Into<Page>>(args: TokenStream, input: TokenStream) -> TokenStream {
    match parse_page_args::<T>(args) {
        Ok(page) => create_page(page, input),
        Err(token_stream) => token_stream
    }
}

fn parse_page_args<T: FromMeta + Into<Page>>(args: TokenStream) -> Result<Page, TokenStream> {
    match NestedMeta::parse_meta_list(proc_macro2::TokenStream::from(args)) {
        Ok(attr_args) => match T::from_list(&attr_args) {
            Ok(v) => Ok(v.into()),
            Err(e) => Err(TokenStream::from(e.write_errors()))
        },
        Err(e) => Err(TokenStream::from(Error::from(e).write_errors()))
    }
}

fn create_page(page: Page, input: TokenStream) -> TokenStream {
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
        },
        PageKind::BlogIndex => quote! { PageKind::BlogIndex },
        PageKind::HomePage => quote! { PageKind::HomePage{ body: #body } },
    };
    let page_title = page.title;
    let file_name = page.file_name.unwrap_or(fn_ident.to_string());
    let route = format!("{}{}", page.route, file_name);

    quote! {
        pub(crate) fn #fn_ident() -> Page {
            let route = Route(#route);

            Page{
                kind: #kind,
                title: #page_title,
                route
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
                }) if path.is_ident("doc") => Some(comment.value().to_string()),
                _ => None,
            }
        })
        .collect()
}
