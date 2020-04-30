#[warn(unused_imports)]
use std::ops::{Deref, DerefMut};
// use std::collections::linked_list;
use std::rc::Rc;

struct Node<T> {
    pre: Option<Rc<Node<T>>>,
    next: Option<Rc<Node<T>>>,
    value: T,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        return Node {
            pre: None,
            next: None,
            value,
        };
    }
}

pub struct LinkList<T> {
    head: Option<Rc<Node<T>>>,
    tail: Option<Rc<Node<T>>>,
    len: u32,
}

impl<T: PartialEq + Clone> LinkList<T> {
    pub fn new() -> Self {
        return LinkList {
            head: None,
            tail: None,
            len: 0,
        };
    }

    pub fn push_back(&mut self, value: T) {
        let mut node = Node::new(value);
        let r = Rc::new(node);
        if self.len == 0 {
            self.head = Some(Rc::clone(&r));
        } else {
            let mut tail_rc = self.tail.take().unwrap();
            let mut tail_node = *Rc::get_mut(&mut tail_rc).unwrap();
            tail_node.next = Some(Rc::clone(&r));
            node.pre = Some(Rc::clone(&tail_rc));
        }
        self.tail = Some(r);
        self.len += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match self.tail {
            None => None,
            Some(_) => {
                let node = self.tail.take().unwrap();
                self.tail = node.pre;
                return Some(node.value);
            }
        }
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

    // pub fn get_tail(self) -> Option<T> {
    //     let tail = self.tail;
    //     match tail {
    //         Some(mut r) => Some((*Rc::get_mut(&mut r).unwrap()).value.clone()),
    //         None => None,
    //     }
    // }
}

impl<T: PartialEq> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T: PartialEq> DerefMut for Node<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}
