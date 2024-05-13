use quick_xml::{events::attributes::Attributes, name::QName};

#[derive(Debug, Clone)]
pub struct TableOfContentsItem {
    pub path: String,
    pub label: String,
    pub content: Option<String>,
}

impl PartialEq for TableOfContentsItem {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path && self.label == other.label
    }
}

impl TableOfContentsItem {
    //TODO: Pass down content dir from EBook
    pub fn get_href_attribute_epub2(attributes: Attributes) -> String {
        let mut href: String = "".to_string();

        for attribute in attributes {
            let attr = attribute.unwrap();

            if attr.key.as_ref() == b"src" {
                href = String::from_utf8(attr.value.to_vec()).unwrap();
            }
        }

        "OEBPS/".to_string() + href.as_str()
    }

    //TODO: Pass down content dir from EBook
    pub fn get_href_attribute_epub3(attributes: Attributes) -> String {
        let mut href: String = "".to_string();

        for attribute in attributes {
            let attr = attribute.unwrap();

            if attr.key == QName(b"href") {
                href = String::from_utf8(attr.value.to_vec()).unwrap();
            }
        }

        "OPS/".to_string() + href.as_str()
    }
}
