pub struct Node<'a, T: PartialEq> {
    value: T,
    pre: Option<&'a Node<'a, T>>,
    next: Option<&'a Node<'a, T>>,
}

impl<'a, T: PartialEq> Node<'a, T> {
    pub fn new(value: T, pre: Option<&'a Node<'a, T>>, next: Option<&'a Node<'a, T>>) -> Node<'a, T>{
        return Node {
            value,
            pre,
            next,
        }
    }

    pub fn get_value(&self) -> &T {
        return &self.value;
    }
}
