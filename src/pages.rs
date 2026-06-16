use crate::page::{Page, PageKind, Post};
use crate::route::Route;
use chrono::NaiveDate;
use site_generator_derive::*;

/// ### Welcome to my website
#[home_page()]
pub(crate) fn home_page() {}

/// ### My about page
#[page(title = "About")]
pub(crate) fn about() {}

/// ### This page cannot be found
#[page(title = "404", file_name = "404")]
pub(crate) fn unknown_page() {}

#[posts_index()]
pub(crate) fn posts_index() {}

/// My website is generated using a custom static site generator written in Rust. There are no
/// reasons why a pre-existing generator couldn't be used, but writing my own is fun.
/// While thinking about the code, I remembered that Rust has a feature called [rustdoc](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html)
/// and, more importantly, the ability to generate HTML from markdown attached to various syntactical
/// elements such as function and struct declarations. With this in mind, I decided to write my
/// static site generator that uses markdown attached to functions and a procedural macro. In
/// effect, I have created a static site generator where the pages are created in similarly to rustdoc.
///
/// ### Example blog post
///
/// ```rust
/// /// ### My blog post
/// /// This is *fun*.
/// #[blog_post(title = "My Blog Post", published = true, publish_date = "2023-01-01")]
/// pub(crate) fn example_post() {}
/// ```
///
/// ### Why?
///
/// GitHub Pages is a cool service and having a personal website could come into use at a later date.
/// Also, it is **free**. I can't complain about that. I decided that it would be novel to package as much of my site into a single binary.
/// Rustdoc and [maud](https://maud.lambda.xyz) allow me to template pages all within `.rs` source files.
/// Furthermore, I wanted to try GitHub Actions and to see if a Nix Flake can be used to ease deployment.
/// Shockingly, my pipeline managed to pull down the correct dependencies, compile the site, and deploy the [*first time*](https://github.com/WCollier/wcollier.github.io/actions/runs/5163982875).
///
/// ### Advantages
///
/// * I can modify the site to my heart's content without having to create awkward hacks or
/// learn an unknown codebase.
///     * Complex logic or special pages (such as the [posts index](https://wcollier.github.io/posts)) are all my code, not workarounds built on top of someone else's code.
/// * Git allows me to experiment with the contents of posts without fear.
/// * I had fun.
///
/// ### Disadvantages
///
/// * I must now implement and maintain the static site generator myself :)
///
/// ### Conclusion
///
/// The purpose of this project wasn't to create a useful or necessarily practical site.
/// I am without constraints that a third-party solution may burden me with. However, I am entirely responsible for code the base and must implement all the features I desire myself.
///
/// See the Git repository [here](https://www.github.com/WCollier/wcollier.github.io).
#[post(
    title = "How my Static Site Generator works",
    published = true,
    publish_date = "2023-09-24"
)]
pub(crate) fn how_my_static_site_generator_works() {}

/// The Nix community has a scattered collection of Wikis, tutorial sites, blogs, forums and more. Some
/// of these are official others community created. I wanted to collate as many useful and educational links in one place as possible.
///
/// Hopefully, someone else finds this useful :)
///
/// ### Lookup
///
/// I frequently use these to search for available options and to track the progress of packages.
///
/// * [NixOS Search](https://search.nixos.org/options)
/// * [Nixpkgs Search](https://search.nixos.org/packages)
/// * [Home Manager Option Search](https://home-manager-options.extranix.com/)
/// * [Noogle](https://noogle.dev/)
/// * [Packages search for NUR](https://nur.nix-community.org/)
/// * [Nixpkgs-tracker](https://nixpkgs-tracker.ocfox.me/) - Tracks which `nixpkgs` branch a PR has
/// made it to
///
/// ### Learning/Reference
///
/// * [Nix Learn](https://nixos.org/learn/) - Starting point for learning Nix on the official site
/// * [Nix Manual](https://nixos.org/manual/nix/stable)
/// * [Nixpkgs Manual](https://nixos.org/manual/nixpkgs/stable)
/// * [NixOS Manual](https://nixos.org/manual/nixos/stable)
/// * [Nix Pills](https://nixos.org/guides/nix-pills) - Series of guides for demonstrating Nix
/// * [Official NixOS Wiki](https://wiki.nixos.org/wiki/NixOS_Wiki)
/// * [Unofficial NixOs Wiki](https://nixos.wiki/wiki/Main_Page)
/// * [Zero to Nix](https://zero-to-nix.com/) - Third-party guide for learning Nix. Uses flakes.
/// * [Awesome Nix](https://github.com/nix-community/awesome-nix)
/// * [nix.dev](https://nix.dev/) - Official documentation for the Nix ecosystem
/// * [How to Learn Nix](https://ianthehenry.com/posts/how-to-learn-nix/) - Series of blog posts
/// * [Nix Learning](https://github.com/humancalico/nix-learning) - Another index like this one
/// * [The Nix Hour](https://www.youtube.com/playlist?list=PLyzwHTVJlRc8yjlx4VR4LU5A5O44og9in) - YouTube playlist about Nix
/// * [Learn Nix in Y Minutes](https://learnxinyminutes.com/docs/nix/) - Quick outline of the Nix language
///
/// ### Community
///
/// * [NixOs Discourse](https://discourse.nixos.org/) - Official Nix forum
/// * [Nix Subreddit](https://old.reddit.com/r/nixos) - Beware the design. Somehow, the custom CSS
/// on Reddit's old interface is worse than Reddit's new interface!
#[post(title = "Nix Links", published = true, publish_date = "2024-07-24")]
pub(crate) fn nix_links() {}

/// Posting on this blog is apparently an annual tradition now. To meet my own arbitrary quota for
/// posts, I have decided to create a list of things (media, technology, etc.) that I liked this
/// year. The thing don't have to be released this year, but I do have to enjoyed it this year.
///
/// I'll provide a brief summary of why I liked each.
///
/// ### Books
///
/// I have read a few books this year (as always too few). I found most of them enjoyable but I
/// found some much more enjoyable than anothers.
///
/// #### Chaos: The Truth Behind The Manson Murders
///
/// Written by Tom O'Neill. I have long been interested in the 1960s and the counter-culture
/// movement and it's art.
///
/// The book itself goes into great depth in trying to untagle a complex web of individuals and their interests.
/// The conventional Helter Skelter narrative is disregarded by *O'Neill*'s book.
/// However, by the author's own admission, a new narrative has not yet been formed.
/// There are still plenty of details obscured or unknown.
///
/// The narrative of itself follows the author's life as the piece evolves from a magazine article
/// to book. It forms an unitentional document of the evolution of the publishing industry.
/// We see how the book went from a much-delayed magazine article and, after the closure of the
/// magazine, to a book.
///
/// #### My Struggle Book 1 & 2
///
/// Author: Karl Ove Knausgaard. I don't think I have read a book that I can describe as a
/// page-turner. However, these books are the exception. Reading about the minutae of this Man's
/// life. Simultaenously banal but inthraling. Many of the details presented in the narrative are
/// hyper-specific but also very relatable. It's as-if Knausgaard manages to understand something
/// that also Humans (obvious caveats around demographics). experience but none-others (or atleast those that I have read, which is too few) have yet written.
/// Some parts of the books do lose me, mostly those relating to Norwegian/Scandinavian art. I am ignorant regarding that topic.
///
/// I am looking forward to reading the other installments in this series.
///
/// ### Podcasts
///
/// #### Weird Medieval Guys
///
/// Author: Olivia Swarthout and Aran Prince-Tappé I love podcasts. I listen to quite a few and at least one episode of a podcast per day. With
/// that being said, Weird Medieval Guys (WMG) is probably my favourite. I consistently go back and
/// re-listen to episodes of this podcast that I listened to only recently.
///
/// Their humour is much my own. The topics they cover are also consistently interesting and enjoyable.
/// Neither fact nor humour are ignored for the other. Instead a delicate balance is struck between the two.
///
/// Some of the older episodes do suffer from unfortunate audio issues. However, all recent
/// episodes are perfectly listenable.
///
/// #### Age of Napoleon
///
/// Author: Everett Rummage. Another history podcast. Much different in tone compared to WMG as Age
/// of Napoleon is almost entirely fact with little humour. The narrative spun here is really quite
/// good. The podcasts only releases narrative episodes once a month (with a Patreon special
/// episode also once a month). I look forward to the start of the month so I immerse myself back
/// into the well-researched world presented here.
///
/// However, following the narrative can become difficult (particularly in battles and log
/// compaigns) while doing other tasks. In those instances, I find myself frequently rewinding and
/// pausing my task to focus solely on the podcast.
///
/// ### Music
///
/// TODO(williamcollier): Something here
///
/// ### Technology
///
/// #### Nix
///
/// As I posted about previously on this blog, I continue to use and enjoy Nix. It can be an
/// absolute pain to work with. Furthermore, the documentation is quite poor and scattered. Despite this, I have
/// continued to expand my usage of Nix. My main PC no runs NixOS full time (as opposed to using
/// WSL on Windows 11). I now have a home server that runs NixOS. My laptop continues to run
/// NixOS.
///
/// I'm not sure what would convince my to move away from NixOS at this point. I have invested a
/// lot of time in setting things up the way I like it.
///
#[post(
    title = "Things I Liked In 2025",
    published = true,
    publish_date = "2025-12-29"
)]
pub(crate) fn things_i_liked_in_2025() {}
