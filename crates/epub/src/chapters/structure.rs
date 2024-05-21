use std::cell::RefCell;
use std::rc::{Rc, Weak};

use quick_xml::events::Event;
use quick_xml::Reader;

#[derive(Debug)]
struct ChapterNode {
    tag: String,
    content: RefCell<String>,
    parent: RefCell<Weak<ChapterNode>>,
    children: RefCell<Vec<Rc<ChapterNode>>>,
}

impl ChapterNode {
    fn new(tag: String, content: String) -> ChapterNode {
        ChapterNode {
            tag: tag,
            content: RefCell::new(content),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        }
    }

    fn add_child(parent: &Rc<ChapterNode>, child: &Rc<ChapterNode>) {
        *child.parent.borrow_mut() = Rc::downgrade(parent);
        parent.children.borrow_mut().push(Rc::clone(child));
    }

    fn append_to_content(node: &Rc<ChapterNode>, content: &str) {
        node.content.borrow_mut().push_str(content);
    }
}

struct Chapter {}

impl Chapter {
    //TODO: Introduce placeholders for the children inside parents
    fn recreate_structure(chapter_content: &str) -> Rc<ChapterNode> {
        let mut root = Rc::new(ChapterNode::new("root".to_string(), String::new()));
        let mut current_node = root.clone();

        let mut reader = Reader::from_str(chapter_content);
        reader.trim_text(true);

        let mut buf = Vec::new();
        let mut tag: String = String::new();
        let mut content: String = String::new();
        let mut nesting_level: u8 = 0;
        let mut currently_reading_other_item: bool = false;

        while let Ok(event) = reader.read_event_into(&mut buf) {
            match event {
                Event::Start(ref e) => {
                    // if currently_reading_other_item {
                    //     continue;
                    // }
                    currently_reading_other_item = true;
                    nesting_level += 1;

                    tag = String::from_utf8(e.name().0.to_vec()).unwrap();

                    let new_node = Rc::new(ChapterNode::new(tag, String::new()));

                    ChapterNode::add_child(&current_node, &new_node);

                    current_node = new_node;
                    println!("Start curent: {:?}", current_node);
                }
                Event::Text(e) => {
                    content = e.unescape().unwrap().to_string();
                    ChapterNode::append_to_content(&current_node, &content);

                    println!("Text curent: {:?}", current_node);
                }
                Event::End(ref e) => {
                    println!("End curent: {:?}", current_node);

                    currently_reading_other_item = false;
                    nesting_level -= 1;

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
}

#[cfg(test)]
mod structure {
    use std::rc::Rc;

    use crate::chapters::structure::Chapter;

    use super::ChapterNode;

    #[test]
    fn should_create_chapter_node() {
        let sut = ChapterNode::new("h1".to_string(), "Chapter 1".to_string());

        assert_eq!(sut.tag, "h1".to_string());
        assert_eq!(sut.content, "Chapter 1".to_string().into());
        assert_eq!(sut.children.borrow().len(), 0);
        assert!(sut.parent.borrow().upgrade().is_none());
    }

    #[test]
    fn should_add_child_to_chapter_node() {
        let sut = Rc::new(ChapterNode::new("h1".to_string(), "Chapter 1".to_string()));
        let child = Rc::new(ChapterNode::new(
            "div".to_string(),
            "Some line of text".to_string(),
        ));

        ChapterNode::add_child(&sut, &child);

        assert_eq!(sut.children.borrow().len(), 1);
        assert_eq!(sut.children.borrow()[0].tag, "div".to_string());
        assert_eq!(
            sut.children.borrow()[0].content,
            "Some line of text".to_string().into()
        );
        assert_eq!(
            child.parent.borrow().upgrade().unwrap().tag,
            "h1".to_string()
        );
    }

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
