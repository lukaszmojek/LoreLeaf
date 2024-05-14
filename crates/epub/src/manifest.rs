use std::rc::Rc;

use quick_xml::{events::Event, name::QName, Reader};

/// Struct to represent all items from the manifest
pub struct BookManifest {
    /// Each epub needs to have a list of manifest items
    /// Consider hiding that behind a pub method
    pub items: Vec<Rc<ManifestItem>>,
}

impl BookManifest {
    pub fn from_opf(opf_content: &str) -> BookManifest {
        let mut reader = Reader::from_str(opf_content);
        reader.trim_text(true);

        let mut buf = Vec::new();
        let mut manifest_items: Vec<Rc<ManifestItem>> = vec![];

        while let Ok(event) = reader.read_event_into(&mut buf) {
            match event {
                Event::Start(ref e) | Event::Empty(ref e) => {
                    if let b"item" = e.name().as_ref() {
                        BookManifest::recreate_manifest_entry(e, &mut manifest_items);
                    }
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }

        BookManifest {
            items: manifest_items,
        }
    }

    //TODO: Consider moving to ManifestItem
    fn recreate_manifest_entry(
        e: &quick_xml::events::BytesStart<'_>,
        manifest_items: &mut Vec<Rc<ManifestItem>>,
    ) {
        let mut id = "".to_string();
        let mut href = "".to_string();
        let mut media_type = "".to_string();

        for attribute_result in e.attributes() {
            let attribute = attribute_result.unwrap();
            match attribute.key {
                QName(b"id") => {
                    id = String::from_utf8(attribute.value.into_owned()).unwrap();
                }
                QName(b"href") => {
                    href = String::from_utf8(attribute.value.into_owned()).unwrap();
                }
                QName(b"media-type") => {
                    media_type = String::from_utf8(attribute.value.into_owned()).unwrap();
                }
                _ => {}
            }
        }

        let manifest_item = ManifestItem {
            id,
            href,
            media_type,
        };
        manifest_items.push(Rc::new(manifest_item))
    }

    /// Search for an item in the manifest by part of its id or href.
    /// Returns the first item that matches the search.
    pub fn search_for_item(&self, query: &str) -> Option<Rc<ManifestItem>> {
        for item in &self.items {
            if item.id.contains(query) || item.href.contains(query) {
                return Some(item.clone());
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct ManifestItem {
    pub id: String,
    pub href: String,
    pub media_type: String,
}

impl PartialEq for ManifestItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.href == other.href && self.media_type == other.media_type
    }
}

#[cfg(test)]
mod manifest_tests {
    use crate::epub::EBook;

    use super::*;

    const MOBY_DICK_PATH: &str = "./test_data/epub/moby-dick.epub";

    #[test]
    fn search_for_item_should_return_matching_item_when_queried() {
        let book = EBook::read_epub(MOBY_DICK_PATH.to_string()).unwrap();

        let manifest = book.manifest;

        let toc_from_manifest = manifest
            .search_for_item("toc")
            .unwrap_or_else(|| panic!("toc was not found during search in manifest"));

        assert_eq!(toc_from_manifest.id, "toc");
        assert_eq!(toc_from_manifest.href, "toc.xhtml");
        assert_eq!(toc_from_manifest.media_type, "application/xhtml+xml");
    }
}
