#![allow(unused)]

#[derive(Clone)]
pub struct Node<T: Clone> {
    data: T,
    next: Option<Box<Node<T>>>,
}

type OptionBoxedNode<T> = Option<Box<Node<T>>>;

pub struct LinkedList<T: Clone> {
    head: OptionBoxedNode<T>,
    tail: OptionBoxedNode<T>,
}

impl<T: Clone> LinkedList<T> {
    pub fn new_empty() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

  pub fn new_with_node(node: Node<T>) -> Self {
    Self {
      head: Some(Box::new(node)),
      tail: None,
    }
  }

  pub fn append(&mut self, node: Node<T>) {
    match self.head.take() {
      Some(mut old_head) => {
        old_head.next = Some(Box::new(node.clone()));
        self.tail = Some(Box::new(node));
      }
      None => {
        self.head = Some(Box::new(node));
      }
    }
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
    assert_eq!(ll.head.unwrap().data, 16)
  }
}
