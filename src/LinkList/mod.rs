use std::rc::Rc;

pub struct LinkList<T: PartialEq> {
    head: Option<Rc<LinkList<T>>>,
    tail: Option<Rc<LinkList<T>>>,
    value: T,
}

impl<'a, T: PartialEq> LinkList<T> {
    pub fn new(value: T) -> LinkList<T> {
        return LinkList {
            head: None,
            tail: None,
            value,
        };
    }

    pub fn add(mut self, value: T) {
        let node = Rc::new(LinkList::new(value));
        self.tail = Some(Rc::clone(&node));
        node.head = Some(Rc::clone(&Rc::new(self)));
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
