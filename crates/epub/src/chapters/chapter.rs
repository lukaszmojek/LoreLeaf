use std::rc::Rc;

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

    pub fn from_item_with_content(item: TableOfContentsItem, content: String) -> Chapter {
        Chapter {
            path: item.path.clone(),
            label: item.label.clone(),
            recreated_structure: Chapter::recreate_structure(&content),
            _raw_content: content,
        }
    }

    //TODO: Introduce placeholders for the children inside parents
    //TODO: Create a different version of this method that would flatten the structure, so every element should have a contnet,
    // then child content and at the end there would be still a place for parent content, resulting in 3 potential elements in a place of 1 and its children.
    // Alternatively, this can be dane in other method, when translating the structure to the elements in bevy
    fn recreate_structure(chapter_content: &str) -> Rc<ChapterNode> {
        let mut root = Rc::new(ChapterNode::new("root".to_string(), vec![], String::new()));
        let mut current_node = root.clone();

        let mut reader = Reader::from_str(chapter_content);
        reader.trim_text(false);

        let mut buf = Vec::new();
        let mut tag: String = String::new();
        let mut content: String = String::new();

        while let Ok(event) = reader.read_event_into(&mut buf) {
            match event {
                Event::Start(ref e) => {
                    let mut classes: Vec<String> = vec![];

                    for attr in e.attributes() {
                        let attr = attr.unwrap();
                        let key = String::from_utf8(attr.key.0.to_vec()).unwrap();
                        let value = String::from_utf8(attr.value.to_vec()).unwrap();

                        println!("{}: {}", key, value);

                        if key == "class" {
                            classes = value.split_whitespace().map(|s| s.to_string()).collect();
                        }
                    }

                    tag = String::from_utf8(e.name().0.to_vec()).unwrap();

                    let new_node = Rc::new(ChapterNode::new(tag, classes, String::new()));

                    ChapterNode::add_child(&current_node, &new_node);

                    current_node = new_node;
                }
                Event::Text(e) => {
                    content = e.unescape().unwrap().replace("\u{ad}", "").to_string();
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
                    vec![],
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
            assert!(body.unwrap().children.borrow()[0]
                .classes
                .contains(&"Basic-text-frame".to_string()));
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

    mod recreate_structure_tests {
        use std::borrow::Borrow;

        use crate::chapters::chapter::Chapter;

        #[test]
        fn should_recreate_div_with_nested_element_and_multiple_classes() {
            //arrange
            let chapter_content: &str = r#"<div class="line bold">Hello there - <i class="character-name">said Obi Wan</i></div>"#;

            //act
            let sut = Chapter::recreate_structure(chapter_content);

            //assert
            assert_eq!(sut.tag, "root".to_string());
            assert_eq!(sut.children.borrow().len(), 1);

            let hello_there_div = &sut.children.borrow()[0];

            assert_eq!(hello_there_div.tag, "div".to_string());
            assert_eq!(hello_there_div.content, "Hello there - ".to_string().into());
            assert_eq!(hello_there_div.classes.len(), 2);
            assert_eq!(hello_there_div.classes[0], "line");
            assert_eq!(hello_there_div.classes[1], "bold");
            assert_eq!(hello_there_div.children.borrow().len(), 1);

            let obi_wan_said = &hello_there_div.children.borrow()[0];

            assert_eq!(obi_wan_said.tag, "i".to_string());
            assert_eq!(obi_wan_said.content, "said Obi Wan".to_string().into());
            assert_eq!(obi_wan_said.classes.len(), 1);
            assert_eq!(obi_wan_said.classes[0], "character-name");
            assert_eq!(obi_wan_said.children.borrow().len(), 0);
        }

        #[test]
        fn should_recreate_paragraph_with_single_quotes_in_content() {
            //arrange
            let chapter_content: &str =
                r#"<p class="line">'General Kenobi' - replied Grievous</p>"#;

            //act
            let sut = Chapter::recreate_structure(chapter_content);

            //assert
            assert_eq!(sut.tag, "root".to_string());
            assert_eq!(sut.children.borrow().len(), 1);

            let general_kenobi = &sut.children.borrow()[0];

            assert_eq!(general_kenobi.tag, "p".to_string());
            assert_eq!(
                general_kenobi.content,
                "'General Kenobi' - replied Grievous".to_string().into()
            );
            assert_eq!(general_kenobi.classes.len(), 1);
            assert_eq!(general_kenobi.classes[0], "line");
            assert_eq!(general_kenobi.children.borrow().len(), 0);
        }

        #[test]
        fn should_recreate_complex_chapter_structure() {
            //arrange
            let chapter_content: &str = r#"
            <h1 class="first-heading">Chapter 1</h1>
            <div class="line bold">Hello there - <i>said Obi Wan</i></div>
            <p class="line bold">'General Kenobi' - replied Grievous</p>
            <div class="list">
                <div>1</div>
                <div>2</div>
                <div>3
                    <div class="nested-first">3a</div>
                    <div class="nested-last">3b</div>
                </div>
            </div>
        "#;

            //act
            let sut = Chapter::recreate_structure(chapter_content);

            //assert
            assert_eq!(sut.tag, "root".to_string());
            assert_eq!(sut.children.borrow().len(), 4);

            let chapter_heading_h1 = &sut.children.borrow()[0];
            assert_eq!(chapter_heading_h1.tag, "h1".to_string());
            assert_eq!(chapter_heading_h1.content, "Chapter 1".to_string().into());
            assert_eq!(chapter_heading_h1.children.borrow().len(), 0);

            let hello_there_div = &sut.children.borrow()[1];
            assert_eq!(hello_there_div.tag, "div".to_string());
            assert_eq!(hello_there_div.content, "Hello there - ".to_string().into());
            assert_eq!(hello_there_div.classes.len(), 2);
            assert_eq!(hello_there_div.classes[0], "line");
            assert_eq!(hello_there_div.classes[1], "bold");
            assert_eq!(hello_there_div.children.borrow().len(), 1);

            let said_obi_wan_i = &hello_there_div.children.borrow()[0];
            assert_eq!(said_obi_wan_i.tag, "i".to_string());
            assert_eq!(said_obi_wan_i.content, "said Obi Wan".to_string().into());
            assert_eq!(said_obi_wan_i.classes.len(), 0);
            assert_eq!(said_obi_wan_i.children.borrow().len(), 0);

            let general_kenobi_p = &sut.children.borrow()[2];
            assert_eq!(general_kenobi_p.tag, "p".to_string());
            assert_eq!(
                general_kenobi_p.content,
                "'General Kenobi' - replied Grievous".to_string().into()
            );
            assert_eq!(general_kenobi_p.children.borrow().len(), 0);

            let list_div = &sut.children.borrow()[3];
            assert_eq!(list_div.tag, "div".to_string());
            assert_eq!(list_div.content.borrow().trim(), String::new());
            assert_eq!(list_div.children.borrow().len(), 3);

            let first_level_first_div = &list_div.children.borrow()[0];
            assert_eq!(first_level_first_div.tag, "div".to_string());
            assert_eq!(first_level_first_div.content, "1".to_string().into());
            assert_eq!(first_level_first_div.children.borrow().len(), 0);

            let first_level_second_div = &list_div.children.borrow()[1];
            assert_eq!(first_level_second_div.tag, "div".to_string());
            assert_eq!(first_level_second_div.content, "2".to_string().into());
            assert_eq!(first_level_second_div.children.borrow().len(), 0);

            let first_level_third_div = &list_div.children.borrow()[2];
            assert_eq!(first_level_third_div.tag, "div".to_string());
            assert_eq!(
                first_level_third_div.content.borrow().trim(),
                "3".to_string()
            );
            assert_eq!(first_level_third_div.children.borrow().len(), 2);

            let second_level_first_div = &first_level_third_div.children.borrow()[0];
            assert_eq!(second_level_first_div.tag, "div".to_string());
            assert_eq!(second_level_first_div.content, "3a".to_string().into());
            assert_eq!(second_level_first_div.children.borrow().len(), 0);

            let second_level_second_div = &first_level_third_div.children.borrow()[1];
            assert_eq!(second_level_second_div.tag, "div".to_string());
            assert_eq!(second_level_second_div.content, "3b".to_string().into());
            assert_eq!(second_level_second_div.children.borrow().len(), 0);
        }

        #[test]
        /// Removes U+00ad (soft hyphen) from the content
        fn should_remove_soft_hyphens_from_the_content() {
            //arrange
            let chapter_content: &str = r#"
            <body>
                <p>Znaj­do­wa­łem się na polu.</p>
            </body>
        "#;

            //act
            let sut = Chapter::recreate_structure(chapter_content);

            //assert
            assert_eq!(sut.tag, "root".to_string());
            assert_eq!(sut.children.borrow().len(), 1);

            let body = &sut.children.borrow()[0];
            assert_eq!(body.tag, "body".to_string());

            let first_paragraph = &body.children.borrow()[0];
            assert_eq!(first_paragraph.tag, "p".to_string());
            assert_eq!(
                first_paragraph.content,
                "Znajdowałem się na polu.".to_string().into()
            );
            assert_eq!(first_paragraph.children.borrow().len(), 0);
        }
    }
}
