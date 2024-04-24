use std::{borrow::Borrow, fs::File, path::Path};

use quick_xml::{events::Event, name::QName, Reader};
use zip::ZipArchive;

use crate::{epub::EBook, manifest::BookManifest};

#[derive(Clone)]
pub struct TableOfContents {
    pub items: Vec<TableOfContentsItem>,
}

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
    pub fn get_href_attribute(e: &quick_xml::events::BytesStart<'_>) -> String {
        let mut href: String = "".to_string();

        for attribute in e.attributes() {
            let attr = attribute.unwrap();

            if attr.key == QName(b"href") {
                href = String::from_utf8(attr.value.to_vec()).unwrap();
            }
        }

        "OPS/".to_string() + href.as_str()
    }
}

impl TableOfContents {
    pub fn create_table_of_contents(
        zip: &mut ZipArchive<File>,
        manifest: &BookManifest,
        content_dir: &Path,
    ) -> Self {
        // todo!("Href here is not absolute, it is relative to opf file. This needs to be addressed.");
        let table_of_contents_from_manifest = manifest.search_for_item("toc").unwrap();

        let toc_path = content_dir.join(table_of_contents_from_manifest.href.clone());

        let toc_content = EBook::get_archive_file_content(zip, toc_path.to_str().unwrap())
            .unwrap_or_else(|err| {
                eprintln!("{:?}", err);
                "NONE".to_string()
            });

        TableOfContents::from_toc_content(toc_content)
    }

    pub fn from_toc_content(toc_content: String) -> TableOfContents {
        let mut reader = Reader::from_str(toc_content.borrow());
        reader.trim_text(true);

        let navigation_selector: String = "toc".to_string();

        let mut buf = Vec::new();
        let mut toc_items: Vec<TableOfContentsItem> = vec![];

        let mut toc_item_href: String = "".to_string();
        let mut toc_item_label: String = "".to_string();
        let mut toc_item_reading_started: bool = false;
        let mut is_inside_toc_nav: bool = false;

        while let Ok(event) = reader.read_event_into(&mut buf) {
            match event {
                Event::Start(ref e) | Event::Empty(ref e) => {
                    //TODO: Refactor that attribute check
                    _ = e.attributes().any(|x| -> bool {
                        if let Ok(attr) = x {
                            if attr.key == QName(b"epub:type") {
                                let ns = String::from_utf8(attr.value.to_vec()).unwrap();
                                is_inside_toc_nav = ns == navigation_selector;
                            }
                        }

                        false
                    });

                    if let b"a" = e.name().as_ref() {
                        toc_item_href = TableOfContentsItem::get_href_attribute(e);
                        toc_item_reading_started = true;
                    }
                }
                Event::Text(e) => {
                    toc_item_label = e.unescape().unwrap().to_string();
                }
                Event::End(e) => {
                    if !toc_item_reading_started {
                        continue;
                    }

                    if !is_inside_toc_nav {
                        break;
                    }

                    let toc_item = TableOfContentsItem {
                        path: toc_item_href,
                        label: toc_item_label,
                        content: None,
                    };

                    toc_items.push(toc_item);
                    toc_item_href = "".to_string();
                    toc_item_label = "".to_string();
                    toc_item_reading_started = false;
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }

        TableOfContents { items: toc_items }
    }

    pub fn search_for_item(&self, href: &str) -> Option<&TableOfContentsItem> {
        self.items.iter().find(|item| item.path == href)
    }
}

#[cfg(test)]
mod table_of_contents_tests {
    use crate::epub::EBook;

    use super::*;
    const MOBY_DICK_PATH: &str = "./data/moby-dick.epub";

    #[test]
    fn should_contain_properly_read_items_of_the_book() {
        let book = EBook::read_epub(MOBY_DICK_PATH.to_string()).unwrap();

        let table_of_contents = book.table_of_contents;

        let toc_length = table_of_contents.items.len();

        assert_eq!(toc_length, 141);

        assert_eq!(table_of_contents.items[0].path, "OPS/titlepage.xhtml");
        assert_eq!(table_of_contents.items[0].label, "Moby-Dick");

        assert_eq!(
            table_of_contents.items[toc_length - 3].path,
            "OPS/chapter_135.xhtml"
        );
        assert_eq!(
            table_of_contents.items[toc_length - 3].label,
            "Chapter 135. The Chase.—Third Day."
        );
    }

    #[test]
    fn reader_should_get_the_content_based_on_toc_item() {
        let mut book = EBook::read_epub(MOBY_DICK_PATH.to_string()).unwrap();

        let table_of_contents = book.table_of_contents.clone();

        let toc_length = table_of_contents.items.len();
        let selected_toc_item = table_of_contents.items[toc_length - 3].clone();

        let toc_item_content = book.get_content_by_toc_item(&selected_toc_item).unwrap();

        assert_eq!(selected_toc_item.path, "OPS/chapter_135.xhtml");
        assert_eq!(
            selected_toc_item.label,
            "Chapter 135. The Chase.—Third Day."
        );
        //Adding all characters count and the new line characters which are not displayed
        assert_eq!(toc_item_content.len(), 26305 + 73);
    }

    #[test]
    fn search_should_return_some_with_existing_toc_item_when_it_matches_search_criteria() {
        //arrange
        let book = EBook::read_epub(MOBY_DICK_PATH.to_string()).unwrap();
        let table_of_contents = book.table_of_contents.to_owned();
        let expected_toc_item = TableOfContentsItem {
            label: "Chapter 135. The Chase.—Third Day.".to_string(),
            path: "OPS/chapter_135.xhtml".to_string(),
            content: None,
        };

        //act
        let found_toc_item = table_of_contents
            .search_for_item(&expected_toc_item.path)
            .unwrap();

        //assert
        assert_eq!(expected_toc_item, *found_toc_item);
    }

    #[test]
    fn search_should_return_none_when_no_toc_item_matches_search_criteria() {
        //arrange
        let book = EBook::read_epub(MOBY_DICK_PATH.to_string()).unwrap();
        let table_of_contents = book.table_of_contents.to_owned();

        //act
        let found_toc_item = table_of_contents.search_for_item("OPS/chapter_666.xhtml");

        //assert
        assert!(found_toc_item.is_none());
    }
}
