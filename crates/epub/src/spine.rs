use std::rc::Rc;

use quick_xml::{events::Event, name::QName, Reader};

use crate::ManifestItem;

#[derive(Debug)]
pub struct BookSpine {
    pub items: Vec<BookSpineItem>,
}

#[derive(Debug)]
pub struct BookSpineItem {
    pub id: String,
    //TODO: Connect spine item with manifest item
    // value: Rc<ManifestItem>,
}

impl BookSpine {
    pub fn from_opf(opf_content: &String) -> BookSpine {
        let mut reader = Reader::from_str(opf_content);
        reader.trim_text(true);

        let mut buf = Vec::new();
        let mut spine: Vec<BookSpineItem> = Vec::new();

        while let Ok(event) = reader.read_event_into(&mut buf) {
            match event {
                Event::Start(ref e) | Event::Empty(ref e) => {
                    // current_tag = String::from_utf8(e.name().as_ref().to_vec()).unwrap();
                    if let b"itemref" = e.name().as_ref() {
                        for attribute in e.attributes() {
                            let attr = attribute.unwrap();
                            if attr.key == QName(b"idref") {
                                let spine_item = BookSpineItem {
                                    id: String::from_utf8(attr.value.into_owned()).unwrap(),
                                };
                                spine.push(spine_item);
                                break;
                            }
                        }
                    }
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }

        BookSpine { items: spine }
    }
}
