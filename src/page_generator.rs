use maud::Markup;
use site::Site;
use page::Meta;

pub(crate) trait PageGenerator {
    fn meta(&self) -> Meta;

    fn template(&self, site: &Site) -> Markup;
}

impl PageGenerator for &dyn PageGenerator {
    fn meta(&self) -> Meta {
        (*self).meta()
    }

    fn template(&self, site: &Site) -> Markup {
        (*self).template(site)
    }
}

