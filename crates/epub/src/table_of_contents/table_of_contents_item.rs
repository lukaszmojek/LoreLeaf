use quick_xml::{events::attributes::Attributes, name::QName};

#[derive(Debug, Clone)]
pub struct TableOfContentsItem {
    pub path: String,
    pub anchor: Option<String>,
    pub label: String,
    pub content: Option<String>,
}

impl PartialEq for TableOfContentsItem {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path && self.label == other.label
    }
}

impl TableOfContentsItem {
    pub fn new(path: String, label: String, content: Option<String>) -> Self {
        let split_path = path.split_once("#");

        let cleaned_path = match split_path {
            Some((path_chunk, _)) => path_chunk.to_string(),
            None => path.to_string(),
        };

        let anchor = match split_path {
            Some((_, anchor_chunk)) => {
                if anchor_chunk.is_empty() {
                    None
                } else {
                    Some(anchor_chunk.to_string())
                }
            }
            None => None,
        };

        Self {
            path: cleaned_path,
            anchor,
            label,
            content,
        }
    }

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

#[cfg(test)]
mod creation {
    use super::TableOfContentsItem;

    #[test]
    fn new_should_create_item_without_anchor_when_there_is_no_hash() {
        let path = "OPS/chapter_136.xhtml".to_string();
        let expected_path = "OPS/chapter_136.xhtml".to_string();
        let label = "Epilogue".to_string();

        let sut = TableOfContentsItem::new(path, label.clone(), None);

        assert_eq!(sut.path, expected_path);
        assert_eq!(sut.label, label);
        assert_eq!(sut.anchor, None);
        assert_eq!(sut.content, None)
    }

    #[test]
    fn new_should_create_item_without_anchor_when_there_is_nothing_after_hash() {
        let path = "OPS/chapter_136.xhtml#".to_string();
        let expected_path = "OPS/chapter_136.xhtml".to_string();
        let label = "Epilogue".to_string();

        let sut = TableOfContentsItem::new(path, label.clone(), None);

        assert_eq!(sut.path, expected_path);
        assert_eq!(sut.label, label);
        assert_eq!(sut.anchor, None);
        assert_eq!(sut.content, None)
    }

    #[test]
    fn new_should_create_item_with_anchor_when_there_is_content_after_hash() {
        let path = "OPS/chapter_136.xhtml#toc_marker-1".to_string();
        let expected_path = "OPS/chapter_136.xhtml".to_string();
        let expected_anchor = Some("toc_marker-1".to_string());
        let label = "Epilogue".to_string();

        let sut = TableOfContentsItem::new(path, label.clone(), None);

        assert_eq!(sut.path, expected_path);
        assert_eq!(sut.label, label);
        assert_eq!(sut.anchor, expected_anchor);
        assert_eq!(sut.content, None)
    }
}
