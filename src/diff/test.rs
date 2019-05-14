
use super::*;

#[derive(Debug, PartialEq, Eq)]
enum TestOperation {
    SetText(String),
    ReplaceWithText(String),
    ReplaceWithElement(Element),
    SetAttribute(String, String),
    RemoveAttribute(String),
    RemoveAllChildren,
    AddAllChildren(Vec<NodePtr>),
    PrependChild(NodePtr),
    InsertChild(NodePtr),
    DeleteChild,
    SkipChild,
}
use TestOperation::*;

struct TestDiffWriter {
    v: Vec<TestOperation>,
}

impl DiffWriter for TestDiffWriter {
    fn set_text(&mut self, text: &str) {
        self.v.push(SetText(text.into()));
    }

    fn replace_with_text(&mut self, text: &str) {
        self.v.push(ReplaceWithText(text.into()));
    }
    fn replace_with_element(&mut self, element: &Element) {
        self.v.push(ReplaceWithElement(element.clone()));
    }
    fn set_attribute(&mut self, key: &str, value: &str) {
        self.v.push(SetAttribute(key.into(), value.into()));
    }
    fn remove_attribute(&mut self, key: &str) {
        self.v.push(RemoveAttribute(key.into()));
    }
    fn remove_all_children(&mut self) {
        self.v.push(RemoveAllChildren);
    }
    fn add_all_children(&mut self, children: &[NodePtr]) {
        self.v
            .push(AddAllChildren(children.iter().cloned().collect()));
    }
    fn prepend_child(&mut self, child: &NodePtr) {
        self.v.push(PrependChild(child.clone()));
    }
    fn insert_child(&mut self, child: &NodePtr) {
        self.v.push(InsertChild(child.clone()));
    }
    fn delete_child(&mut self) {
        self.v.push(DeleteChild);
    }
    fn skip_child(&mut self) {
        self.v.push(SkipChild);
    }
}

fn empty_element(tag: &str) -> Node {
    Node::Element(Element {
        tag: tag.into(),
        children: vec![],
        attrs: im::OrdMap::new(),
    })
}

fn empty_div() -> Node {
    empty_element("div")
}

fn empty_div_element() -> Element {
    match empty_div() {
        Node::Element(e) => e,
        _ => panic!(),
    }
}

fn div_with_children(children: Vec<NodePtr>) -> Node {
    Node::Element(Element {
        tag: "div".into(),
        children,
        attrs: im::OrdMap::new(),
    })
}

fn empty_span() -> Node {
    empty_element("span")
}

fn empty_div_with_attr(attr: &str, attr_value: &str) -> Node {
    let node = empty_div();
    let mut element = if let Node::Element(element) = node {
        element
    } else {
        panic!()
    };
    let mut attrs = im::OrdMap::new();
    attrs.insert(attr.into(), attr_value.into());
    element.attrs = attrs;
    Node::Element(element)
}

fn text_containing(s: &str) -> Node {
    Node::Text(s.into())
}

fn ptr(n: Node) -> NodePtr {
    use std::sync::Arc;
    NodePtr(Arc::new(n))
}

fn diff(a: &Node, b: &Node) -> Vec<TestOperation> {
    let mut w = TestDiffWriter { v: vec![] };
    super::diff(a, b, &mut w);
    w.v
}

#[test]
fn diff_text_content() {
    let d = diff(&text_containing("hi"), &text_containing("bye"));
    assert_eq!(d, vec![SetText("bye".into())]);
}

#[test]
fn diff_element_into_text() {
    let d = diff(&empty_div(), &text_containing("bye"));
    assert_eq!(d, vec![ReplaceWithText("bye".into())]);
}

#[test]
fn diff_text_into_element() {
    let d = diff(&text_containing("bye"), &empty_div());
    assert_eq!(d, vec![ReplaceWithElement(empty_div_element())]);
}

#[test]
fn diff_element_span_into_div() {
    let d = diff(&empty_span(), &empty_div());
    assert_eq!(d, vec![ReplaceWithElement(empty_div_element())]);
}

#[test]
fn diff_element_add_attribute() {
    let old = empty_div();
    let new = empty_div_with_attr("key", "value");
    let d = diff(&old, &new);
    assert_eq!(d, vec![SetAttribute("key".into(), "value".into())]);
}

#[test]
fn diff_element_remove_attribute() {
    let old = empty_div_with_attr("key", "value");
    let new = empty_div();
    let d = diff(&old, &new);
    assert_eq!(d, vec![RemoveAttribute("key".into())]);
}

#[test]
fn diff_element_update() {
    let old = empty_div_with_attr("key", "old-value");
    let new = empty_div_with_attr("key", "new-value");
    let d = diff(&old, &new);
    assert_eq!(d, vec![SetAttribute("key".into(), "new-value".into())]);
}

#[test]
fn one_element_insertion() {
    let ele = ptr(empty_div());
    let old = div_with_children(vec![]);
    let new = div_with_children(vec![ele.clone()]);
    let d = diff(&old, &new);
    assert_eq!(d, vec![AddAllChildren(vec![ele])]);
}

#[test]
fn one_element_removal() {
    let ele = ptr(empty_div());
    let old = div_with_children(vec![ele.clone()]);
    let new = div_with_children(vec![]);
    let d = diff(&old, &new);
    assert_eq!(d, vec![RemoveAllChildren]);
}

#[test]
fn no_diff_ptr() {
    let ele = ptr(empty_div());
    let old = div_with_children(vec![ele.clone()]);
    let new = div_with_children(vec![ele.clone()]);
    let d = diff(&old, &new);
    assert_eq!(d, vec![SkipChild]);
}

#[test]
fn prepend_test() {
    let original_ptr = ptr(empty_div());
    let leading_ptr = ptr(empty_span());
    let old = div_with_children(vec![original_ptr.clone()]);
    let new = div_with_children(vec![leading_ptr.clone(), original_ptr.clone()]);
    let d = diff(&old, &new);
    assert_eq!(d, vec![PrependChild(leading_ptr.clone()), SkipChild]);
}
#[test]
fn append_test() {
    let original_ptr = ptr(empty_div());
    let trailing_ptr = ptr(empty_span());
    let old = div_with_children(vec![original_ptr.clone()]);
    let new = div_with_children(vec![original_ptr.clone(), trailing_ptr.clone()]);
    let d = diff(&old, &new);
    assert_eq!(d, vec![SkipChild, InsertChild(trailing_ptr.clone())]);
}

#[test]
fn insert_into_middle_test() {
    let outer_ptrs = ptr(empty_div());
    let inner_ptr = ptr(empty_span());
    let old = div_with_children(vec![outer_ptrs.clone(), outer_ptrs.clone()]);
    let new = div_with_children(vec![
        outer_ptrs.clone(),
        inner_ptr.clone(),
        outer_ptrs.clone(),
    ]);
    let d = diff(&old, &new);
    assert_eq!(
        d,
        vec![SkipChild, InsertChild(inner_ptr.clone()), SkipChild,]
    );
}
