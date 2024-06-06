use std::sync::{Arc, RwLock, Weak};

#[derive(Debug)]
pub struct ChapterNode {
    pub tag: String,
    pub classes: Vec<String>,
    pub content: RwLock<String>,
    pub(crate) parent: RwLock<Weak<ChapterNode>>,
    pub(crate) children: RwLock<Vec<Arc<ChapterNode>>>,
}

impl ChapterNode {
    pub(crate) fn new(tag: String, classes: Vec<String>, content: String) -> ChapterNode {
        ChapterNode {
            tag: tag,
            classes,
            content: RwLock::new(content),
            parent: RwLock::new(Weak::new()),
            children: RwLock::new(vec![]),
        }
    }

    pub(crate) fn add_child(parent_node: &Arc<ChapterNode>, child_node: &Arc<ChapterNode>) {
        let child_parent_weak = Arc::downgrade(&parent_node);
        let child = Arc::clone(child_node);
        let parent = Arc::clone(parent_node);

        let mut child_parent = child.parent.write().expect("Lock poisoned");
        *child_parent = child_parent_weak;

        let mut children = parent.children.write().expect("Lock poisoned");
        children.push(Arc::clone(&child));
    }
    pub(crate) fn append_to_content(&self, content: &str) {
        self.content
            .write()
            .expect("Lock poisoned")
            .push_str(content);
    }

    pub fn get_content(&self) -> String {
        self.content.read().expect("Lock poisoned").clone()
    }

    pub fn get_children(&self) -> Vec<Arc<ChapterNode>> {
        self.children.read().expect("Lock poisoned").clone()
    }

    pub fn get_parent(&self) -> Weak<ChapterNode> {
        self.parent.read().expect("Lock poisoned").clone()
    }
}

#[cfg(test)]
mod chapter_node_tests {
    use std::sync::Arc;

    use crate::chapters::chapter_node::ChapterNode;

    #[test]
    fn should_create_chapter_node() {
        let sut = ChapterNode::new("h1".to_string(), vec![], "Chapter 1".to_string());

        assert_eq!(sut.tag, "h1".to_string());
        assert_eq!(sut.get_content(), "Chapter 1".to_string());
        assert_eq!(sut.get_children().len(), 0);
        assert!(sut.get_parent().upgrade().is_none());
    }

    #[test]
    fn should_add_child_to_chapter_node() {
        let sut = Arc::new(ChapterNode::new(
            "h1".to_string(),
            vec![],
            "Chapter 1".to_string(),
        ));
        let child = Arc::new(ChapterNode::new(
            "div".to_string(),
            vec![],
            "Some line of text".to_string(),
        ));

        ChapterNode::add_child(&sut, &child);

        assert_eq!(sut.get_children().len(), 1);
        assert_eq!(sut.get_children()[0].tag, "div".to_string());
        assert_eq!(
            sut.get_children()[0].get_content(),
            "Some line of text".to_string()
        );
        assert_eq!(child.get_parent().upgrade().unwrap().tag, "h1".to_string());
    }
}
