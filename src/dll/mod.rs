use crate::node::Node;

pub struct DLL<'a, T: PartialEq> {
    head: Option<&'a Node<'a, T>>,
    tail: Option<&'a Node<'a, T>>,
    len: usize,
}

impl<'a, T: PartialEq> DLL<'a, T> {
    pub fn new() -> DLL<'a, T> {
        return DLL {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn add(&mut self, value: T) {
        let pre = if self.len == 0 { None } else { self.tail };
        let next = None;
        let node: Node<'a, T> = Node::new(value, pre, next);
        if self.len == 0 {
            self.head = Some(&node);
        }
        self.tail = Some(&node);
        self.len += 1;
    }

    /*pub fn remove(&mut self, p: &Node<T>) -> bool {
        if self.len == 0 { return false; }
        let mut pointer = self.head;
        while pointer.is_some() {
            let node = pointer.take().unwrap();
            if true {
                return true;
            }
        }
        return false;
    }*/
}
