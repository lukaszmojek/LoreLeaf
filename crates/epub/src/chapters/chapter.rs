use std::cell::RefCell;
use std::rc::{Rc, Weak};

use quick_xml::events::Event;
use quick_xml::Reader;

use crate::epub::EBook;
use crate::table_of_contents::table_of_contents_item::TableOfContentsItem;

use super::chapter_node::ChapterNode;

#[derive(Debug, Clone)]
pub struct Chapter {
    pub path: String,
    pub label: String,
    pub recreated_structure: Rc<ChapterNode>,
    pub(crate) _raw_content: String,
}

impl PartialEq for Chapter {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path && self.label == other.label
    }
}

impl Chapter {
    pub fn from_item(item: TableOfContentsItem, ebook: &mut EBook) -> Chapter {
        //TODO: Consider moving chapter creation to this method invocation, since from this point on TocItem is not itself anymore
        let content = ebook.get_content_by_toc_item(&item).unwrap();

        Chapter {
            path: item.path.clone(),
            label: item.label.clone(),
            recreated_structure: Chapter::recreate_structure(&content),
            _raw_content: content,
        }
    }

    //TODO: Introduce placeholders for the children inside parents
    fn recreate_structure(chapter_content: &str) -> Rc<ChapterNode> {
        let mut root = Rc::new(ChapterNode::new("root".to_string(), String::new()));
        let mut current_node = root.clone();

        let mut reader = Reader::from_str(chapter_content);
        reader.trim_text(true);

        let mut buf = Vec::new();
        let mut tag: String = String::new();
        let mut content: String = String::new();

        while let Ok(event) = reader.read_event_into(&mut buf) {
            match event {
                Event::Start(ref e) => {
                    // if currently_reading_other_item {
                    //     continue;
                    // }
                    tag = String::from_utf8(e.name().0.to_vec()).unwrap();

                    let new_node = Rc::new(ChapterNode::new(tag, String::new()));

                    ChapterNode::add_child(&current_node, &new_node);

                    current_node = new_node;
                }
                Event::Text(e) => {
                    content = e.unescape().unwrap().to_string();
                    ChapterNode::append_to_content(&current_node, &content);
                }
                Event::End(ref e) => {
                    let parent = current_node.parent.borrow().upgrade().unwrap();
                    current_node = parent;
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }

        current_node
    }

    pub fn get_body(&self) -> Option<Rc<ChapterNode>> {
        const BODY_TAG: &str = "body";

        for child_element in self.recreated_structure.children.borrow().iter() {
            if child_element.tag == BODY_TAG {
                let body = Rc::clone(child_element);
                return Some(body);
            }

            for grandchild_element in child_element.children.borrow().iter() {
                if grandchild_element.tag == BODY_TAG {
                    let body = Rc::clone(grandchild_element);
                    return Some(body);
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod chaptger_node_tests {
    use crate::chapters::chapter::Chapter;

    #[test]
    fn should_recreate_chapter_structure() {
        //arrange
        let chapter_content: &str = r#"
            <h1>Chapter 1</h1>
            <div>Hello there - <i>said Obi Wan</i></div>
            <p>'General Kenobi' - replied Grievous</p>
            <div>
                <div>1</div>
                <div>2</div>
                <div>3
                    <div>3a</div>
                    <div>3b</div>
                </div>
            </div>
        "#;

        //act
        let sut = Chapter::recreate_structure(chapter_content);

        //assert
        assert_eq!(sut.tag, "root".to_string());
        assert_eq!(sut.children.borrow().len(), 4);

        assert_eq!(sut.children.borrow()[0].tag, "h1".to_string());
        assert_eq!(
            sut.children.borrow()[0].content,
            "Chapter 1".to_string().into()
        );
        assert_eq!(sut.children.borrow()[0].children.borrow().len(), 0);

        assert_eq!(sut.children.borrow()[1].tag, "div".to_string());
        assert_eq!(
            sut.children.borrow()[1].content,
            "Hello there -".to_string().into() // "Hello there - {{0001}}".to_string().into()
        );
        assert_eq!(sut.children.borrow()[1].children.borrow().len(), 1);

        assert_eq!(
            sut.children.borrow()[1].children.borrow()[0].tag,
            "i".to_string()
        );
        assert_eq!(
            sut.children.borrow()[1].children.borrow()[0].content,
            "said Obi Wan".to_string().into()
        );
        assert_eq!(
            sut.children.borrow()[1].children.borrow()[0]
                .children
                .borrow()
                .len(),
            0
        );

        assert_eq!(sut.children.borrow()[2].tag, "p".to_string());
        assert_eq!(
            sut.children.borrow()[2].content,
            "'General Kenobi' - replied Grievous".to_string().into()
        );
        assert_eq!(sut.children.borrow()[2].children.borrow().len(), 0);

        assert_eq!(sut.children.borrow()[3].tag, "div".to_string());
        assert_eq!(
            sut.children.borrow()[3].content,
            String::new().into() // "{{0001}}{{0002}}".to_string().into()
        );
        assert_eq!(sut.children.borrow()[3].children.borrow().len(), 3);

        assert_eq!(
            sut.children.borrow()[3].children.borrow()[0].tag,
            "div".to_string()
        );
        assert_eq!(
            sut.children.borrow()[3].children.borrow()[0].content,
            "1".to_string().into()
        );
        assert_eq!(
            sut.children.borrow()[3].children.borrow()[0]
                .children
                .borrow()
                .len(),
            0
        );

        assert_eq!(
            sut.children.borrow()[3].children.borrow()[1].tag,
            "div".to_string()
        );
        assert_eq!(
            sut.children.borrow()[3].children.borrow()[1].content,
            "2".to_string().into()
        );
        assert_eq!(
            sut.children.borrow()[3].children.borrow()[1]
                .children
                .borrow()
                .len(),
            0
        );

        assert_eq!(
            sut.children.borrow()[3].children.borrow()[2].tag,
            "div".to_string()
        );
        assert_eq!(
            sut.children.borrow()[3].children.borrow()[2].content,
            "3".to_string().into() // "3{{0001}}{{0002}}".to_string().into()
        );
        assert_eq!(
            sut.children.borrow()[3].children.borrow()[2]
                .children
                .borrow()
                .len(),
            2
        );

        assert_eq!(
            sut.children.borrow()[3].children.borrow()[2]
                .children
                .borrow()[0]
                .tag,
            "div".to_string()
        );
        assert_eq!(
            sut.children.borrow()[3].children.borrow()[2]
                .children
                .borrow()[0]
                .content,
            "3a".to_string().into()
        );
        assert_eq!(
            sut.children.borrow()[3].children.borrow()[2]
                .children
                .borrow()[0]
                .children
                .borrow()
                .len(),
            0
        );

        assert_eq!(
            sut.children.borrow()[3].children.borrow()[2]
                .children
                .borrow()[0]
                .tag,
            "div".to_string()
        );
        assert_eq!(
            sut.children.borrow()[3].children.borrow()[2]
                .children
                .borrow()[1]
                .content,
            "3b".to_string().into()
        );
        assert_eq!(
            sut.children.borrow()[3].children.borrow()[2]
                .children
                .borrow()[1]
                .children
                .borrow()
                .len(),
            0
        );
    }
}

#[cfg(test)]
mod chapter_tests {
    use super::*;

    mod partial_eq {
        use std::rc::Rc;

        use super::*;

        #[test]
        fn partial_eq_should_check_path() {
            //arrange
            let chapter1 = create_chapter("1", "1", "111");
            let chapter2 = create_chapter("2", "1", "111");

            //assert
            assert_ne!(chapter1, chapter2);
        }

        #[test]
        fn partial_eq_should_check_label() {
            //arrange
            let chapter1 = create_chapter("1", "1", "111");
            let chapter2 = create_chapter("1", "2", "111");

            //assert
            assert_ne!(chapter1, chapter2);
        }

        #[test]
        fn partial_eq_should_not_check_content() {
            //arrange
            let chapter1 = create_chapter("1", "1", "111");
            let chapter2 = create_chapter("1", "1", "222");

            //assert
            assert_eq!(chapter1, chapter2);
        }

        fn create_chapter(path: &str, label: &str, content: &str) -> Chapter {
            Chapter {
                path: path.to_string(),
                label: label.to_string(),
                _raw_content: content.to_string(),
                recreated_structure: Rc::new(ChapterNode::new(
                    "tag".to_string(),
                    "content".to_string(),
                )),
            }
        }
    }

    mod get_body {
        use crate::chapters::chapter::Chapter;

        #[test]
        fn should_return_some_when_body_node_exists_in_contnet() {
            //arrange
            let chapter_content: &str = r#"
                <?xml version='1.0' encoding='UTF-8'?>
                <html xmlns="http://www.w3.org/1999/xhtml">
                    <head>
                        <title>DRAGONEZA-1</title>
                        <link href="../Styles/Dragoneza_idstyles.css" rel="stylesheet" type="text/css"/>
                        <meta http-equiv="Content-Type" content="text/html; charset=utf-8"/>
                    </head>
                    <body id="DRAGONEZA" xml:lang="pl-PL">
                        <div class="Basic-text-frame">
                            <p id="toc_marker-1" class="TYTUL-R">Spis tre­ści</p>
                            <p class="spis-tytul"><a href="../Text/DRAGONEZA-2.xhtml#Zakotwiczenie">Ko­bie­ta, któ­ra sły­sza­ła smo­ki</a></p>
                            <p class="spis-autor">Agniesz­ka Fu­liń­ska</p>
                        </div>
                    </body>
                </html>
                "#;

            let chapter_node = Chapter::recreate_structure(chapter_content);
            let chapter = Chapter {
                path: "path".to_string(),
                label: "label".to_string(),
                recreated_structure: chapter_node,
                _raw_content: chapter_content.to_string(),
            };

            //act
            let body = chapter.get_body();

            //assert
            assert!(body.is_some());
        }

        #[test]
        fn should_return_none_when_no_body_node_exists_in_contnet() {
            //arrange
            let chapter_content: &str = r#"
                <?xml version='1.0' encoding='UTF-8'?>
                <html xmlns="http://www.w3.org/1999/xhtml">
                    <head>
                        <title>DRAGONEZA-1</title>
                        <link href="../Styles/Dragoneza_idstyles.css" rel="stylesheet" type="text/css"/>
                        <meta http-equiv="Content-Type" content="text/html; charset=utf-8"/>
                    </head>
                    <div class="Basic-text-frame">
                        <p id="toc_marker-1" class="TYTUL-R">Spis tre­ści</p>
                        <p class="spis-tytul"><a href="../Text/DRAGONEZA-2.xhtml#Zakotwiczenie">Ko­bie­ta, któ­ra sły­sza­ła smo­ki</a></p>
                        <p class="spis-autor">Agniesz­ka Fu­liń­ska</p>
                    </div>
                </html>
                "#;

            let chapter_node = Chapter::recreate_structure(chapter_content);
            let chapter = Chapter {
                path: "path".to_string(),
                label: "label".to_string(),
                recreated_structure: chapter_node,
                _raw_content: chapter_content.to_string(),
            };

            //act
            let body = chapter.get_body();

            //assert
            assert!(body.is_none());
        }
    }
}
