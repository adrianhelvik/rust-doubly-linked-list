use std::{cell::RefCell, fmt::Formatter, rc::Rc};

#[macro_export]
macro_rules! doubly_linked_list {
    ($($element:expr), +) => {{
        let mut root = None;
        let mut ptr = None;

        $(
            let current = Rc::new(
                Node {
                    prev: RefCell::new(match &ptr {
                        Some(node) => Some(Rc::clone(node)),
                        None => None
                    }),
                    value: Rc::new($element),
                    next: RefCell::new(None),
                }
            );
            if let Some(prev) = ptr {
                *prev.next.borrow_mut() = Some(Rc::clone(&current));
                if root.is_none() {
                    root = Some(Rc::clone(&prev));
                }
            }
            ptr = Some(Rc::clone(&current));
        )*

        drop(ptr);

        DoublyLinkedList {
            root: RefCell::new(root)
        }
    }}
}

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

struct Node<'a, T> {
    prev: RefCell<Option<Rc<Node<'a, T>>>>,
    value: Rc<T>,
    next: RefCell<Option<Rc<Node<'a, T>>>>,
}

impl<'a, T> std::fmt::Debug for Node<'a, T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        fmt.write_str(format!("{:?}", self.value).as_ref())?;
        match self.next.take() {
            Some(next) => {
                fmt.write_str(",\n    ")?;
                next.fmt(fmt)?;
                *self.next.borrow_mut() = Some(next);
            }
            None => {}
        }
        Ok(())
    }
}

impl<'a, T> std::fmt::Debug for DoublyLinkedList<'a, T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        fmt.write_str("DoublyLinkedList {\n")?;
        let root = self.root.take();
        if let Some(root) = root {
            fmt.write_str("    ")?;
            *self.root.borrow_mut() = Some(Rc::clone(&root));
            root.fmt(fmt)?;
            fmt.write_str("\n")?;
        }
        fmt.write_str("}")?;
        Ok(())
    }
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
                        *node.$key.borrow_mut() = Some(Rc::clone(&value));
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

    #[test]
    fn it_can_debug_a_node() {
        let node = Node {
            prev: RefCell::new(None),
            value: Rc::new(1337),
            next: RefCell::new(None),
        };

        assert_eq!(format!("{:?}", node), "1337");
    }

    #[test]
    fn it_can_debug_a_doubly_linked_node() {
        let first = Rc::new(Node {
            prev: RefCell::new(None),
            value: Rc::new("first"),
            next: RefCell::new(None),
        });
        let second = Rc::new(Node {
            prev: RefCell::new(Some(Rc::clone(&first))),
            value: Rc::new("second"),
            next: RefCell::new(None),
        });
        *first.next.borrow_mut() = Some(Rc::clone(&second));

        assert_eq!(format!("{:?}", first), "\"first\",\n    \"second\"");
    }

    #[test]
    fn it_can_debug_a_linked_list() {
        let list = doubly_linked_list!("first", "second", "third", "fourth");

        assert_eq!(format!("{:?}", list), "DoublyLinkedList {\n    \"first\",\n    \"second\",\n    \"third\",\n    \"fourth\"\n}");
    }

    #[test]
    fn it_can_iterate_over_the_items() {
        let list = doubly_linked_list!(1, 2);

        let mut out = Vec::<i32>::new();

        for i in list.iter() {
            out.push(*i.clone());
        }

        assert_eq!(out, vec![1, 2]);
    }

    #[test]
    fn it_can_iterate_over_the_items_twice() {
        let list = doubly_linked_list!(1, 2);

        let mut first = Vec::<i32>::new();
        let mut second = Vec::<i32>::new();

        for i in list.iter() {
            first.push(*i.clone());
        }

        for i in list.iter() {
            second.push(*i.clone());
        }

        assert_eq!(vec![first, second], vec![vec![1, 2], vec![1, 2]]);
    }

    #[test]
    fn it_can_get_the_last_node() {
        let list = doubly_linked_list!(1, 2);
        let root = list.root.take().unwrap();
        let last = Node::last(root);

        assert_eq!(last.value, Rc::new(2));
    }

    #[test]
    fn it_can_reverse_iterate_over_the_items() {
        let list = doubly_linked_list!(1, 2);

        let mut out = Vec::<i32>::new();

        for i in list.rev_iter() {
            out.push(*i.clone());
        }

        assert_eq!(out, vec![2, 1]);
    }

    #[test]
    fn it_can_reverse_iterate_over_the_items_multiple_times() {
        let list = doubly_linked_list!(1, 2);

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
