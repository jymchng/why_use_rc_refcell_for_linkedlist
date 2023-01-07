#![allow(unused)]
use std::rc::Rc;

#[derive(Clone)]
pub struct Node<T: Clone> {
    data: T,
    next: Option<Rc<Node<T>>>,
}

type OptionRcNode<T> = Option<Rc<Node<T>>>;

pub struct LinkedList<T: Clone> {
    head: OptionRcNode<T>,
    tail: OptionRcNode<T>,
}

impl<T: Clone> LinkedList<T> {
    pub fn new_empty() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

  pub fn new_with_node(node: Node<T>) -> Self {
    let mut new_ll = Self::new_empty();
    Self::append(&mut new_ll, node);
    new_ll
  }

  pub fn append(&mut self, node: Node<T>) {
    let appending_node = Rc::new(node);
    match self.tail.take() {
      Some(ref mut old_tail) => {
        let old_tail = Rc::make_mut(old_tail);
        old_tail.next = Some(Rc::clone(&appending_node));
      }
      None => {
        self.head = Some(Rc::clone(&appending_node));
      }
    }
    self.tail = Some(appending_node);
  }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_ll_new_empty() {
        let ll = LinkedList::<u32>::new_empty();
        assert!(ll.head.is_none() && ll.tail.is_none());
    }

  #[test]
  fn test_ll_new_with_node() {
    let new_node = Node {
      data: 16,
      next: None,
    };
    let ll = LinkedList::new_with_node(new_node);
    assert_eq!(ll.head.unwrap().data, 16);
    assert_eq!(ll.tail.unwrap().data, 16);
  }

  #[test]
  fn test_ll_append() {
    let new_node = Node {
      data: 16,
      next: None,
    };
    let mut ll = LinkedList::new_with_node(new_node);
    ll.append( Node {data: 17, next: None} );
    assert_eq!(ll.head.unwrap().next.as_ref().unwrap().data, 17);
  }
}

// error[E0382]: use of moved value: `node`
//   --> src/why_use_rc_refcell/why_Rc_not_works.rs:34:35
//    |
// 30 |   pub fn append(&mut self, node: Node<T>) {
//    |                            ---- move occurs because `node` has type `Node<T>`, which does not implement the `Copy` trait
// ...
// 33 |         old_head.next = Some(Rc::new(node));
//    |                                       ---- value moved here
// 34 |         self.tail = Some(Rc::new(node));
//    |                                   ^^^^ value used here after move