pub trait Dom {
    type Element;
    type Text;
    type Node;

    fn is_element(node: &Node) -> bool;
    fn is_element(node: &Node) -> bool;
}
