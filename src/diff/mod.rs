use crate::types::*;

#[cfg(test)]
mod test;

pub trait DiffWriter {
    fn set_text(&mut self, text: &str);
    fn replace_with_text(&mut self, text: &str);
    fn replace_with_element(&mut self, element: &Element);
    fn set_attribute(&mut self, key: &str, value: &str);
    fn remove_attribute(&mut self, key: &str);
    fn remove_all_children(&mut self);
    fn add_all_children(&mut self, children: &[NodePtr]);
    fn prepend_child(&mut self, child: &NodePtr);
    fn insert_child(&mut self, child: &NodePtr);
    fn delete_child(&mut self);
    fn skip_child(&mut self);
}

pub fn diff<W: DiffWriter>(prev: &Node, new: &Node, w: &mut W) {
    match (prev, new) {
        (Node::Text(_), Node::Text(new)) => w.set_text(new),
        (Node::Element(_), Node::Text(new)) => w.replace_with_text(new),
        (Node::Text(_), Node::Element(new)) => w.replace_with_element(new),
        (Node::Element(old), Node::Element(new)) => diff_element(old, new, w),
    }
}

fn diff_element<W: DiffWriter>(prev: &Element, new: &Element, w: &mut W) {
    if prev.tag != new.tag {
        w.replace_with_element(new);
        return;
    }

    for diff in im::OrdMap::diff(&prev.attrs, &new.attrs) {
        use im::ordmap::DiffItem::*;
        match diff {
            Update {
                old: _,
                new: (k, v),
            } => w.set_attribute(k, v),
            Add((k, v)) => w.set_attribute(k, v),
            Remove((k, _)) => w.remove_attribute(k),
            _ => unimplemented!(),
        }
    }

    match (prev.children.is_empty(), new.children.is_empty()) {
        (true, true) => (),
        (false, true) => w.remove_all_children(),
        (true, false) => w.add_all_children(&new.children),
        (false, false) => {
            use lcs::DiffComponent::*;
            let mut is_first = true;
            for diff in lcs::LcsTable::new(&prev.children, &new.children).diff() {
                match diff {
                    Insertion(node) if is_first => {
                        w.prepend_child(&node);
                    }
                    Insertion(node) => {
                        w.insert_child(&node);
                    }
                    Deletion(_) => w.delete_child(),
                    Unchanged(_, _) => w.skip_child(),
                }
                is_first = false;
            }
        }
    }
}
