use std::sync::Arc;

pub type AttrValue = String;
pub type Attrs = im::OrdMap<String, AttrValue>;

#[derive(Debug, Clone)]
pub struct NodePtr(pub Arc<Node>);

impl PartialEq for NodePtr {
    fn eq(&self, other: &NodePtr) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for NodePtr {}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Element {
    pub tag: String,
    pub children: Vec<NodePtr>,
    pub attrs: Attrs,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Node {
    Text(String),
    Element(Element),
}
