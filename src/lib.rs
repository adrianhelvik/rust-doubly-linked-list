use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct DoublyLinkedList<'a, T> {
    root: RefCell<Option<Rc<Node<'a, T>>>>,
}

impl<'a, T> DoublyLinkedList<'a, T> {
    pub fn iter(&self) -> NodeIterator<'a, T> {
        let root = self.root.take();
        match root {
            Some(node) => {
                *self.root.borrow_mut() = Some(Rc::clone(&node));
                NodeIterator {
                    node: Some(Rc::clone(&node)),
                    reverse: false,
                }
            }
            None => NodeIterator {
                node: None,
                reverse: false,
            },
        }
    }

    pub fn rev_iter(&self) -> NodeIterator<'a, T> {
        let root = self.root.take();
        match root {
            Some(node) => {
                let last = Node::last(Rc::clone(&node));
                *self.root.borrow_mut() = Some(Rc::clone(&node));
                NodeIterator {
                    node: Some(Rc::clone(&last)),
                    reverse: true,
                }
            }
            None => NodeIterator {
                node: None,
                reverse: true,
            },
        }
    }
}

#[derive(Debug)]
struct Node<'a, T> {
    prev: RefCell<Option<Rc<Node<'a, T>>>>,
    value: Rc<T>,
    next: RefCell<Option<Rc<Node<'a, T>>>>,
}

impl<'a, T> Node<'a, T> {
    pub fn last(root: Rc<Node<T>>) -> Rc<Node<T>> {
        let mut node = root;
        while let Some(next) = node.next.take() {
            *node.next.borrow_mut() = Some(Rc::clone(&next));
            node = next;
        }
        Rc::clone(&node)
    }
}

#[derive(Debug)]
pub struct NodeIterator<'a, T> {
    node: Option<Rc<Node<'a, T>>>,
    reverse: bool,
}

// Get the next item in a node iterator.
// `$key` should be either `prev` or `next`.
macro_rules! iterate_in_direction {
    ($self:ident, $key:ident) => {{
        let node = $self.node.take();
        match node {
            Some(node) => {
                let new_cell = Rc::clone(&node.value);
                let value = node.$key.take();
                $self.node = match value {
                    Some(value) => {
                        *node.next.borrow_mut() = Some(Rc::clone(&value));
                        Some(Rc::clone(&value))
                    }
                    None => None,
                };
                Some(new_cell)
            }
            None => None,
        }
    }};
}

impl<'a, T> Iterator for NodeIterator<'a, T>
where
    T: std::fmt::Debug,
{
    type Item = Rc<T>;

    fn next(&mut self) -> Option<Rc<T>> {
        match self.reverse {
            false => {
                iterate_in_direction!(self, next)
            }
            true => {
                iterate_in_direction!(self, prev)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_dummy_list<'a>() -> DoublyLinkedList<'a, i32> {
        let last = Rc::new(Node {
            prev: RefCell::new(None),
            value: Rc::new(2),
            next: RefCell::new(None),
        });
        let first = Rc::new(Node {
            prev: RefCell::new(None),
            value: Rc::new(1),
            next: RefCell::new(Some(Rc::clone(&last))),
        });
        {
            *last.prev.borrow_mut() = Some(Rc::clone(&first));
        }
        let list: DoublyLinkedList<i32> = DoublyLinkedList {
            root: RefCell::new(Some(Rc::clone(&first))),
        };
        list
    }

    #[test]
    fn it_can_iterate_over_the_items() {
        let list = create_dummy_list();

        let mut out = Vec::<i32>::new();

        for i in list.iter() {
            out.push(*i.clone());
        }

        assert_eq!(out, vec![1, 2]);
    }

    #[test]
    fn it_can_iterate_over_the_items_twice() {
        let list = create_dummy_list();

        let mut out = Vec::<i32>::new();

        for i in list.iter() {
            out.push(*i.clone());
        }

        for i in list.iter() {
            out.push(*i.clone());
        }

        assert_eq!(out, vec![1, 2, 1, 2]);
    }

    #[test]
    fn it_can_get_the_last_node() {
        let list = create_dummy_list();
        let root = list.root.take().unwrap();
        let last = Node::last(root);

        assert_eq!(last.value, Rc::new(2));
    }

    #[test]
    fn it_can_reverse_iterate_over_the_items() {
        let list = create_dummy_list();

        let mut out = Vec::<i32>::new();

        for i in list.rev_iter() {
            out.push(*i.clone());
        }

        assert_eq!(out, vec![2, 1]);
    }

    #[test]
    fn it_can_reverse_iterate_over_the_items_multiple_times() {
        let list = create_dummy_list();

        let mut out = Vec::<i32>::new();

        for i in list.rev_iter() {
            out.push(*i.clone());
        }

        for i in list.rev_iter() {
            out.push(*i.clone());
        }

        assert_eq!(out, vec![2, 1, 2, 1]);
    }
}
