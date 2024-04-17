use std::borrow::Borrow;

use quick_xml::{events::Event, name::QName, Reader};

#[derive(Clone)]
pub struct TableOfContents {
    pub items: Vec<TableOfContentsItem>,
}

#[derive(Debug, Clone)]
pub struct TableOfContentsItem {
    // id: String,
    pub href: String,
    pub label: String,
}

impl TableOfContentsItem {
    pub fn get_href_attribute(e: &quick_xml::events::BytesStart<'_>) -> String {
        let mut href: String = "".to_string();

        for attribute in e.attributes() {
            let attr = attribute.unwrap();

            if attr.key == QName(b"href") {
                href = String::from_utf8(attr.value.to_vec()).unwrap();
            }
        }

        href
    }
}

impl TableOfContents {
    pub fn from_content(toc_content: String) -> TableOfContents {
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
                        href: toc_item_href,
                        label: toc_item_label,
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
}
