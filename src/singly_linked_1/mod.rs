use std::mem;

use super::traits::List as _;

mod tests;

#[derive(Debug)]
pub struct List<T> {
    maybe_contents: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    maybe_next: Option<Box<Node<T>>>,
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List::new()
    }
}

impl<T> super::traits::List<T> for List<T> {
    /// Make a new empty list.
    fn new() -> Self {
        List {
            maybe_contents: None,
        }
    }

    // Walks the list to count nodes.
    fn length(&self) -> usize {
        match &self.maybe_contents {
            None => 0,
            Some(boxed_node) => {
                let mut length = 1;
                let mut boxed_node = boxed_node;
                while let Some(next_boxed_node) = &boxed_node.maybe_next {
                    boxed_node = next_boxed_node;
                    length += 1;
                }
                length
            }
        }
    }

    /// Append a value at list position 0.
    fn put_first(&mut self, t: T) -> &mut Self {
        let new_node = Box::new(Node {
            value: t,
            maybe_next: self.maybe_contents.take(),
        });
        self.maybe_contents.replace(new_node);
        self
    }

    /// Remove and return the value at list position 0 (or None if the list is empty).
    fn remove_first(&mut self) -> Option<T> {
        match self.maybe_contents.take() {
            None => None,
            Some(boxed_node) => {
                self.maybe_contents = boxed_node.maybe_next;
                Some(boxed_node.value)
            }
        }
    }

    /// Remove and return the value at list position N (and None if the list isn't long enough).
    fn remove_nth(&mut self, index: usize) -> Option<T> {
        let mut maybe_boxed_node = &mut self.maybe_contents;
        let mut current_index: usize = 0;
        #[allow(unused_must_use)]
        loop {
            match maybe_boxed_node {
                None => {
                    return None;
                }
                Some(_) if current_index == index => {
                    let grab_node = maybe_boxed_node.take().unwrap();
                    mem::replace(maybe_boxed_node, grab_node.maybe_next);
                    return Some(grab_node.value);
                }
                Some(boxed_node) => {
                    maybe_boxed_node = &mut boxed_node.maybe_next;
                    current_index += 1;
                }
            }
        }
    }

    fn insert_at(&mut self, index: usize, t: T) -> Result<(), T> {
        let mut maybe_boxed_node = &mut self.maybe_contents;
        let mut current_index: usize = 0;
        #[allow(unused_must_use)]
        loop {
            match maybe_boxed_node {
                _ if current_index == index => {
                    let grab_node = maybe_boxed_node.take();
                    let new_node: Option<Box<Node<T>>> = Some(Box::new(Node {
                        value: t,
                        maybe_next: grab_node,
                    }));
                    mem::replace(maybe_boxed_node, new_node);
                    return Ok(());
                }
                Some(boxed_node) => {
                    maybe_boxed_node = &mut boxed_node.maybe_next;
                    current_index += 1;
                }
                None => {
                    return Err(t);
                }
            }
        }
    }

    /// Replace and return the value at list position N.
    /// If N is not a valid index, the supplied value is wrapped in an error and returned to the caller.
    fn replace_nth(&mut self, index: usize, t: T) -> Result<T, T> {
        let mut maybe_boxed_node = &mut self.maybe_contents;
        let mut current_index: usize = 0;
        #[allow(unused_must_use)]
        loop {
            match maybe_boxed_node {
                None => {
                    return Result::Err(t);
                }
                Some(_) if current_index == index => {
                    let grab_node = maybe_boxed_node.take().unwrap();
                    let replacement_node = Node {
                        value: t,
                        maybe_next: grab_node.maybe_next,
                    };
                    maybe_boxed_node.replace(Box::new(replacement_node));
                    return Ok(grab_node.value);
                }
                Some(boxed_node) => {
                    maybe_boxed_node = &mut boxed_node.maybe_next;
                    current_index += 1;
                }
            }
        }
    }

    /// Remove and return the first value for which the given predicate returns true.
    fn remove_first_matching<F>(&mut self, pred: F) -> Option<T>
    where
        F: Fn(&T) -> bool,
    {
        let mut maybe_boxed_node = &mut self.maybe_contents;
        #[allow(unused_must_use)]
        loop {
            match maybe_boxed_node {
                None => {
                    return None;
                }
                Some(boxed_value) if pred(&boxed_value.value) => {
                    let grab_node = maybe_boxed_node.take().unwrap();
                    mem::replace(maybe_boxed_node, grab_node.maybe_next);
                    return Some(grab_node.value);
                }
                Some(boxed_node) => {
                    maybe_boxed_node = &mut boxed_node.maybe_next;
                }
            }
        }
    }

    /// Append a node at the smallest invalid index.
    fn put_last(&mut self, t: T) -> &mut Self {
        let new_node = Some(Box::new(Node {
            value: t,
            maybe_next: None,
        }));
        match &mut self.maybe_contents {
            None => self.maybe_contents = new_node,
            Some(boxed_node) => {
                let mut boxed_node = boxed_node;
                loop {
                    match &mut boxed_node.maybe_next {
                        tail @ None => {
                            *tail = new_node;
                            break;
                        }
                        Some(next_boxed_node) => {
                            boxed_node = next_boxed_node;
                        }
                    }
                }
            }
        }
        self
    }

    /// Remove and return the node at the largest index.
    fn remove_last(&mut self) -> Option<T> {
        let mut maybe_boxed_node = &mut self.maybe_contents;
        loop {
            match maybe_boxed_node {
                None => {
                    return None;
                }
                Some(boxed_node) if boxed_node.maybe_next.is_none() => {
                    let grab_last_node = maybe_boxed_node.take();
                    return Some(grab_last_node.unwrap().value);
                }
                Some(boxed_node) => {
                    maybe_boxed_node = &mut boxed_node.maybe_next;
                }
            };
        }
    }
}
