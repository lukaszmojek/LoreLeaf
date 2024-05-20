use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::str::FromStr;

use quick_xml::events::Event;
use quick_xml::Reader;

#[derive(Debug)]
struct ChapterNode {
    tag: String,
    content: String,
    parent: RefCell<Weak<ChapterNode>>,
    children: RefCell<Vec<Rc<ChapterNode>>>,
}

impl ChapterNode {
    fn new(tag: String, content: String) -> Rc<ChapterNode> {
        Rc::new(ChapterNode {
            tag: tag,
            content: content,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }

    fn add_child(parent: &Rc<ChapterNode>, child: &Rc<ChapterNode>) {
        child.parent.borrow_mut().upgrade().map(|old_parent| {
            old_parent
                .children
                .borrow_mut()
                .retain(|p| !Rc::ptr_eq(p, child));
        });
        *child.parent.borrow_mut() = Rc::downgrade(parent);
        parent.children.borrow_mut().push(Rc::clone(child));
    }
}

struct Chapter {}

impl Chapter {
    //TODO: This implementation is not complete. Seems that I have mistakenly created a logic, where only 1 parent is valid at a given level during parsing.
    fn recreate_structure(chapter_content: &str) -> Rc<ChapterNode> {
        let mut root = ChapterNode::new("root".to_string(), "".to_string());
        let mut current_node = Rc::clone(&root);

        let mut reader = Reader::from_str(chapter_content);
        reader.trim_text(true);

        let mut buf = Vec::new();

        while let Ok(event) = reader.read_event_into(&mut buf) {
            match event {
                Event::Start(ref e) => {
                    let tag = e.name();
                    let content = reader.read_text(tag).unwrap();
                    let recreate_tag_name = String::from_utf8(tag.0.to_vec()).unwrap();
                    let new_node = ChapterNode::new(recreate_tag_name, content.to_string());

                    ChapterNode::add_child(&current_node, &new_node);

                    current_node = new_node;
                }
                Event::End(ref e) => {
                    let parent = current_node.parent.borrow().upgrade().unwrap();
                    current_node = parent
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }

        root
    }
}

#[cfg(test)]
mod structure {
    use crate::chapters::structure::Chapter;

    use super::ChapterNode;

    #[test]
    fn should_create_chapter_node() {
        let sut = ChapterNode::new("h1".to_string(), "Chapter 1".to_string());

        assert_eq!(sut.tag, "h1".to_string());
        assert_eq!(sut.content, "Chapter 1".to_string());
        assert_eq!(sut.children.borrow().len(), 0);
        assert!(sut.parent.borrow().upgrade().is_none());
    }

    #[test]
    fn should_add_child_to_chapter_node() {
        let sut = ChapterNode::new("h1".to_string(), "Chapter 1".to_string());
        let child = ChapterNode::new("div".to_string(), "Some line of text".to_string());

        ChapterNode::add_child(&sut, &child);

        assert_eq!(sut.children.borrow().len(), 1);
        assert_eq!(sut.children.borrow()[0].tag, "div".to_string());
        assert_eq!(
            sut.children.borrow()[0].content,
            "Some line of text".to_string()
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
        assert_eq!(sut.children.borrow()[0].content, "Chapter 1".to_string());
        assert_eq!(sut.children.borrow()[0].children.borrow().len(), 0);

        assert_eq!(sut.children.borrow()[1].tag, "div".to_string());
        // assert_eq!(sut.children.borrow()[1].content, "Hello there - <i>said Obi Wan</i>".to_string());
        assert_eq!(sut.children.borrow()[1].children.borrow().len(), 1);

        assert_eq!(
            sut.children.borrow()[1].children.borrow()[0].tag,
            "i".to_string()
        );
        assert_eq!(
            sut.children.borrow()[1].children.borrow()[0].content,
            "said Obi Wan".to_string()
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
            "'General Kenobi' - replied Grievous".to_string()
        );
        assert_eq!(sut.children.borrow()[2].children.borrow().len(), 0);

        assert_eq!(sut.children.borrow()[3].tag, "div".to_string());
        assert_eq!(sut.children.borrow()[3].content, "".to_string());
        assert_eq!(sut.children.borrow()[3].children.borrow().len(), 3);

        assert_eq!(
            sut.children.borrow()[3].children.borrow()[0].tag,
            "div".to_string()
        );
        assert_eq!(
            sut.children.borrow()[3].children.borrow()[0].content,
            "1".to_string()
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
            "2".to_string()
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
        // assert_eq!(sut.children.borrow()[3].children.borrow()[2].content, "3".to_string());
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
            "3a".to_string()
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
                .borrow()[0]
                .content,
            "3b".to_string()
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
    }
}
