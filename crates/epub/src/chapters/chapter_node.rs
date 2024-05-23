use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug, Clone)]
pub struct ChapterNode {
    pub tag: String,
    pub content: RefCell<String>,
    pub(crate) parent: RefCell<Weak<ChapterNode>>,
    pub(crate) children: RefCell<Vec<Rc<ChapterNode>>>,
}

impl ChapterNode {
    pub(crate) fn new(tag: String, content: String) -> ChapterNode {
        ChapterNode {
            tag: tag,
            content: RefCell::new(content),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        }
    }

    pub(crate) fn add_child(parent: &Rc<ChapterNode>, child: &Rc<ChapterNode>) {
        *child.parent.borrow_mut() = Rc::downgrade(parent);
        parent.children.borrow_mut().push(Rc::clone(child));
    }

    pub(crate) fn append_to_content(node: &Rc<ChapterNode>, content: &str) {
        node.content.borrow_mut().push_str(content);
    }

    pub fn get_children(&self) -> Vec<Rc<ChapterNode>> {
        self.children.borrow().clone()
    }
}

#[cfg(test)]
mod chapter_node_tests {
    use std::rc::Rc;

    use crate::chapters::chapter_node::ChapterNode;

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
}
