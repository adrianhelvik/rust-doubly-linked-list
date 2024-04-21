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

impl<'a, T> Iterator for NodeIterator<'a, T>
where
    T: std::fmt::Debug,
{
    type Item = Rc<T>;

    fn next(&mut self) -> Option<Rc<T>> {
        match self.reverse {
            false => {
                let node = self.node.take();
                match node {
                    Some(node) => {
                        let item = Rc::clone(&node.value);
                        let next = node.next.take();
                        self.node = match next {
                            Some(next) => {
                                *node.next.borrow_mut() = Some(Rc::clone(&next));
                                Some(Rc::clone(&next))
                            }
                            None => None,
                        };
                        Some(item)
                    }
                    None => None,
                }
            }
            true => {
                let node = self.node.take();
                match node {
                    Some(node) => {
                        let item = Rc::clone(&node.value);
                        let prev = node.prev.take();
                        self.node = match prev {
                            Some(prev) => {
                                *node.prev.borrow_mut() = Some(Rc::clone(&prev));
                                Some(Rc::clone(&prev))
                            }
                            None => None,
                        };
                        Some(item)
                    }
                    None => None,
                }
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
