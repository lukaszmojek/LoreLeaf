use std::{borrow::Borrow, fs::File, path::Path};

use quick_xml::{events::Event, name::QName, Reader};
use zip::ZipArchive;

use crate::{epub::EBook, manifest::BookManifest};

use super::table_of_contents_item::TableOfContentsItem;

#[derive(Debug, Clone)]
pub struct TableOfContents {
    pub items: Vec<TableOfContentsItem>,
}

impl TableOfContents {
    const NCX_EXTENSION: &'static str = ".ncx";

    pub fn read_table_of_contents_from_manifest(
        zip: &mut ZipArchive<File>,
        manifest: &BookManifest,
        content_dir: &Path,
    ) -> (String, String) {
        // todo!("Href here is not absolute, it is relative to opf file. This needs to be addressed.");
        let table_of_contents_from_manifest = manifest.search_for_item("toc").unwrap();

        let toc_path = content_dir.join(table_of_contents_from_manifest.href.clone());
        let toc_href = toc_path.to_str().unwrap();

        let toc_content = EBook::get_archive_file_content(zip, toc_href).unwrap_or_else(|err| {
            eprintln!("{:?}", err);
            "NONE".to_string()
        });

        (toc_href.to_string(), toc_content)
    }

    pub fn from_content(href: String, content: String, content_dir: String) -> Self {
        let is_toc_in_ncx_format = href.contains(Self::NCX_EXTENSION);

        if is_toc_in_ncx_format {
            return TableOfContents::from_toc_content_for_epub_2(content, content_dir);
        }

        TableOfContents::from_toc_content_for_epub_3(content, content_dir)
    }

    pub fn from_toc_content_for_epub_2(
        toc_content: String,
        content_dir: String,
    ) -> TableOfContents {
        let mut reader = Reader::from_str(toc_content.borrow());
        reader.trim_text(true);

        let navigation_selector: &[u8] = b"navMap";

        let mut buf = Vec::new();
        let mut toc_items: Vec<TableOfContentsItem> = vec![];

        let mut toc_item_href: String = String::new();
        let mut toc_item_label: String = String::new();
        let mut toc_item_reading_started: bool = false;
        let mut is_inside_toc_nav: bool = false;

        while let Ok(event) = reader.read_event_into(&mut buf) {
            match event {
                Event::Start(ref e) | Event::Empty(ref e) => {
                    if !is_inside_toc_nav {
                        is_inside_toc_nav = e.name().as_ref() == navigation_selector;
                    }

                    if let b"content" = e.name().as_ref() {
                        toc_item_href = TableOfContentsItem::get_src_attribute_epub2(
                            e.attributes(),
                            &content_dir,
                        );
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

                    let toc_item = TableOfContentsItem::new(toc_item_href, toc_item_label, None);

                    toc_items.push(toc_item);
                    toc_item_href = String::new();
                    toc_item_label = String::new();
                    toc_item_reading_started = false;
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }

        TableOfContents { items: toc_items }
    }

    pub fn from_toc_content_for_epub_3(
        toc_content: String,
        content_dir: String,
    ) -> TableOfContents {
        let mut reader = Reader::from_str(toc_content.borrow());
        reader.trim_text(true);

        let navigation_selector: String = "toc".to_string();

        let mut buf = Vec::new();
        let mut toc_items: Vec<TableOfContentsItem> = vec![];

        let mut toc_item_href: String = String::new();
        let mut toc_item_label: String = String::new();
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
                        toc_item_href = TableOfContentsItem::get_href_attribute_epub3(
                            e.attributes(),
                            &content_dir,
                        );
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

                    let toc_item = TableOfContentsItem::new(toc_item_href, toc_item_label, None);

                    toc_items.push(toc_item);
                    toc_item_href = String::new();
                    toc_item_label = String::new();
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

    pub fn previous_relative(&self, href: &str) -> Option<&TableOfContentsItem> {
        let current_toc_item = self.search_for_item(href).unwrap();

        if self.items.starts_with(&[current_toc_item.clone()]) {
            return None;
        }

        for (index, item) in self.items.iter().enumerate() {
            if item == current_toc_item {
                return Some(&self.items[index - 1]);
            }
        }

        None
    }

    pub fn next_relative(&self, href: &str) -> Option<&TableOfContentsItem> {
        let current_toc_item = self.search_for_item(href).unwrap();

        if self.items.ends_with(&[current_toc_item.clone()]) {
            return None;
        }

        for (index, item) in self.items.iter().enumerate() {
            if item == current_toc_item {
                return Some(&self.items[index + 1]);
            }
        }

        None
    }
}

#[cfg(test)]
mod epub2 {
    use crate::table_of_contents::table_of_contents::TableOfContents;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn should_parse_nav_point() {
        const RAW_ITEM: &str = r#"
          <navMap>
          <navPoint id="navpoint1" playOrder="1">
            <navLabel>
              <text>Spis treści</text>
            </navLabel>
            <content src="Text/DRAGONEZA-1.xhtml#toc_marker-1"/>
          </navPoint>
        </navMap>
        "#;
        let content_dir = "OEBPS".to_string();
        let table_of_contents =
            TableOfContents::from_toc_content_for_epub_2(RAW_ITEM.to_string(), content_dir);

        assert_eq!(table_of_contents.items.len(), 1);
        assert_eq!(table_of_contents.items[0].label, "Spis treści");
        assert_eq!(
            table_of_contents.items[0].path,
            "OEBPS/Text/DRAGONEZA-1.xhtml"
        );
        assert_eq!(
            table_of_contents.items[0].anchor,
            Some("toc_marker-1".to_string())
        )
    }

    #[test]
    fn should_contain_properly_read_items_of_the_book() {
        const TOC_NCX_SAMPLE_PATH: &str = "./test_data/toc/toc.ncx";
        let content_dir = "OEBPS".to_string();
        let mut toc_file = File::open(TOC_NCX_SAMPLE_PATH).unwrap();
        let mut toc_content = String::new();
        toc_file.read_to_string(&mut toc_content).unwrap();

        let table_of_contents =
            TableOfContents::from_content("toc.ncx".to_string(), toc_content, content_dir);

        let toc_length = table_of_contents.items.len();

        assert_eq!(toc_length, 41);

        assert_eq!(
            table_of_contents.items[0].path,
            "OEBPS/Text/DRAGONEZA-1.xhtml"
        );
        assert_eq!(
            table_of_contents.items[0].anchor,
            Some("toc_marker-1".to_string())
        );
        assert_eq!(table_of_contents.items[0].label, "Spis treści");

        assert_eq!(
            table_of_contents.items[toc_length - 2].path,
            "OEBPS/Text/DRAGONEZA-21.xhtml"
        );
        assert_eq!(
            table_of_contents.items[toc_length - 2].anchor,
            Some("toc_marker-40".to_string())
        );
        assert_eq!(
            table_of_contents.items[toc_length - 2].label,
            "Donnerwetter"
        );

        assert_eq!(
            table_of_contents.items[toc_length - 1].path,
            "OEBPS/Text/DRAGONEZA-21.xhtml"
        );
        assert_eq!(
            table_of_contents.items[toc_length - 1].anchor,
            Some("toc_marker-41".to_string())
        );
        assert_eq!(
            table_of_contents.items[toc_length - 1].label,
            "Krzysztof Adamski"
        );
    }
}

#[cfg(test)]
mod epub3 {
    use crate::table_of_contents::table_of_contents::TableOfContents;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn should_parse_nav_point() {
        const RAW_ITEM: &str = r#"
        <nav xmlns:epub="http://www.idpf.org/2007/ops" epub:type="toc" id="toc">
           <ol>
              <li class="toc-BookTitlePage-rw" id="toc-titlepage">
                 <a href="titlepage.xhtml">Moby-Dick</a>
              </li>
           </ol>
        </nav>
        "#;

        let content_dir = "OPS".to_string();
        let table_of_contents =
            TableOfContents::from_toc_content_for_epub_3(RAW_ITEM.to_string(), content_dir);

        assert_eq!(table_of_contents.items.len(), 1);
        assert_eq!(table_of_contents.items[0].label, "Moby-Dick");
        assert_eq!(table_of_contents.items[0].path, "OPS/titlepage.xhtml");
    }

    #[test]
    fn should_contain_properly_read_items_of_the_book() {
        const TOC_XHTML_SAMPLE_PATH: &str = "./test_data/toc/toc.xhtml";
        let mut toc_file = File::open(TOC_XHTML_SAMPLE_PATH).unwrap();
        let mut toc_content = String::new();
        toc_file.read_to_string(&mut toc_content).unwrap();

        let content_dir = "OPS".to_string();
        let table_of_contents =
            TableOfContents::from_content("toc.xhtml".to_string(), toc_content, content_dir);

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
}

#[cfg(test)]
mod navigation {
    use crate::epub::EBook;

    use super::*;
    const MOBY_DICK_PATH: &str = "./test_data/epub/moby-dick.epub";

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
        let expected_toc_item = TableOfContentsItem::new(
            "OPS/chapter_135.xhtml".to_string(),
            "Chapter 135. The Chase.—Third Day.".to_string(),
            None,
        );

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

    #[test]
    fn get_next_relative_should_return_some_with_existing_toc_item_when_there_is_next_toc_item() {
        //arrange
        let book = EBook::read_epub(MOBY_DICK_PATH.to_string()).unwrap();
        let table_of_contents = book.table_of_contents.to_owned();
        let current_toc_item = TableOfContentsItem::new(
            "OPS/chapter_135.xhtml".to_string(),
            "Chapter 135. The Chase.—Third Day.".to_string(),
            None,
        );
        let expected_toc_item = TableOfContentsItem::new(
            "OPS/chapter_136.xhtml".to_string(),
            "Epilogue".to_string(),
            None,
        );

        //act
        let next_toc_item = table_of_contents
            .next_relative(&current_toc_item.path)
            .unwrap();

        //assert
        assert_eq!(expected_toc_item, *next_toc_item);
    }

    #[test]
    fn get_next_relative_should_return_none_when_there_is_no_next_toc_item() {
        //arrange
        let book = EBook::read_epub(MOBY_DICK_PATH.to_string()).unwrap();
        let table_of_contents = book.table_of_contents.to_owned();
        let current_toc_item = TableOfContentsItem::new(
            "OPS/copyright.xhtml".to_string(),
            "Copyright Page".to_string(),
            None,
        );

        //act
        let next_toc_item = table_of_contents.next_relative(&current_toc_item.path);

        //assert
        assert!(next_toc_item.is_none());
    }

    #[test]
    fn get_previous_relative_should_return_some_with_existing_toc_item_when_there_is_previous_toc_item(
    ) {
        //arrange
        let book = EBook::read_epub(MOBY_DICK_PATH.to_string()).unwrap();
        let table_of_contents = book.table_of_contents.to_owned();
        let current_toc_item = TableOfContentsItem::new(
            "OPS/chapter_136.xhtml".to_string(),
            "Epilogue".to_string(),
            None,
        );
        let expected_toc_item = TableOfContentsItem::new(
            "OPS/chapter_135.xhtml".to_string(),
            "Chapter 135. The Chase.—Third Day.".to_string(),
            None,
        );

        //act
        let next_toc_item = table_of_contents
            .previous_relative(&current_toc_item.path)
            .unwrap();

        //assert
        assert_eq!(expected_toc_item, *next_toc_item);
    }

    #[test]
    fn get_previous_relative_should_return_none_when_there_is_no_previous_toc_item() {
        //arrange
        let book = EBook::read_epub(MOBY_DICK_PATH.to_string()).unwrap();
        let table_of_contents = book.table_of_contents.to_owned();
        let current_toc_item = TableOfContentsItem::new(
            "OPS/titlepage.xhtml".to_string(),
            "Moby-Dick".to_string(),
            None,
        );

        //act
        let next_toc_item = table_of_contents.previous_relative(&current_toc_item.path);

        //assert
        assert!(next_toc_item.is_none());
    }
}
