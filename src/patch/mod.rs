trait DiffApplier {
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
