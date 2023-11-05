use std::path::{Path, PathBuf};
use maud::{Render, PreEscaped, Markup};

#[derive(Copy, Clone, Debug)]
pub(crate) struct Route(pub(crate) &'static str); 

impl Route {
    pub(crate) fn file_path(&self) -> PathBuf {
        let mut path = PathBuf::from(self.0);

        match path.file_name() {
            Some(file_name) if file_name == "index" || file_name == "404" => {
                path.set_extension("html");

                path
            },
            _ => {
                path.push("index.html");

                path
            }
        }
    }
}

impl Render for Route {
    fn render(&self) -> Markup {
        let path = Path::new(self.0);

        match (path.parent(), path.file_name()) {
            (Some(parent), Some(file_name)) if file_name == "index" => 
                PreEscaped(parent.display().to_string()),
            _ => PreEscaped(self.0.to_string())
        }
    }
}
