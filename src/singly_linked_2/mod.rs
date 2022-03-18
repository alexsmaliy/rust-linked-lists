use std::cell::RefCell;
use std::fmt::Debug;
use std::mem;
use std::rc::Rc;

use crate::traits::List as _;

mod tests;

#[derive(Debug)]
pub struct List<T> {
    maybe_head: Option<Rc<RefCell<Node<T>>>>,
    maybe_tail: Option<Rc<RefCell<Node<T>>>>,
    length: usize,
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    maybe_next: Option<Rc<RefCell<Node<T>>>>,
}

enum ListUtils {}

impl ListUtils {
    fn unpack<T>(wrapped: Option<Rc<RefCell<Node<T>>>>) -> Node<T> {
        Rc::try_unwrap(wrapped.unwrap()).ok().unwrap().into_inner()
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List::new()
    }
}

impl<T> crate::traits::List<T> for List<T> {
    fn new() -> Self {
        List {
            maybe_head: None,
            maybe_tail: None,
            length: 0,
        }
    }

    fn length(&self) -> usize {
        self.length
    }

    fn insert_at(&mut self, index: usize, t: T) -> Result<(), T> {
        if index == 0 {
            self.put_first(t);
            return Ok(());
        } else if index == self.length {
            self.put_last(t);
            return Ok(());
        } else if index > self.length {
            return Err(t);
        }

        let mut opt: Option<Rc<RefCell<Node<T>>>>;
        let mut rc: Rc<RefCell<Node<T>>>;
        let mut countdown = index - 1;

        opt = Some(Rc::clone(self.maybe_head.as_ref().unwrap()));

        loop {
            rc = Rc::clone(opt.as_ref().unwrap());
            let mut borrowed_node = rc.borrow_mut();
            let maybe_next = &mut borrowed_node.maybe_next;

            if countdown == 0 {
                let next_rc = Some(Rc::clone(maybe_next.as_ref().unwrap()));
                let new_node = Rc::new(RefCell::new(Node {
                    value: t,
                    maybe_next: next_rc,
                }));
                borrowed_node.maybe_next = Some(new_node);
                self.length += 1;
                return Ok(());
            } else {
                opt.replace(Rc::clone(maybe_next.as_ref().unwrap()));
                countdown -= 1;
            }
        }
    }

    fn put_first(&mut self, t: T) -> &mut Self {
        let new_node_rc = Rc::new(RefCell::new(Node {
            value: t,
            maybe_next: self.maybe_head.take(),
        }));
        if self.length == 0 {
            let new_node_rc_clone = new_node_rc.clone();
            self.maybe_head.replace(new_node_rc);
            self.maybe_tail.replace(new_node_rc_clone);
        } else {
            self.maybe_head.replace(new_node_rc);
        }
        self.length += 1;
        self
    }

    fn put_last(&mut self, t: T) -> &mut Self {
        if self.length == 0 {
            return self.put_first(t);
        }

        let new_node = Rc::new(RefCell::new(Node {
            value: t,
            maybe_next: None,
        }));
        let new_node_clone = new_node.clone();

        if let Some(tail_ref) = &mut self.maybe_tail {
            RefCell::borrow_mut(tail_ref).maybe_next.replace(new_node);
        }
        self.maybe_tail.replace(new_node_clone);
        self.length += 1;
        self
    }

    fn remove_first(&mut self) -> Option<T> {
        let maybe_head = self.maybe_head.take();
        if maybe_head.is_none() {
            return None;
        } else {
            #[allow(unused_must_use)]
            if self.length == 1 {
                self.maybe_tail.take();
            }
            let node_rc = maybe_head.unwrap();
            let grab_head = Rc::try_unwrap(node_rc).ok().unwrap().into_inner();
            self.maybe_head = grab_head.maybe_next;
            self.length -= 1;
            return Some(grab_head.value);
        }
    }

    fn remove_last(&mut self) -> Option<T> {
        #[allow(unused_must_use)]
        if self.length == 0 {
            return None;
        } else if self.length == 1 {
            let head = self.maybe_head.take();
            self.maybe_tail.take();
            self.length -= 1;
            return Some(ListUtils::unpack(head).value);
        }

        let mut opt: Option<Rc<RefCell<Node<T>>>> =
            Some(Rc::clone(self.maybe_head.as_ref().unwrap()));
        let mut rc: Rc<RefCell<Node<T>>>;

        loop {
            rc = Rc::clone(opt.as_ref().unwrap());
            let mut borrowed_node = RefCell::borrow_mut(&rc);
            let maybe_next = &mut borrowed_node.maybe_next;
            let next_rc = maybe_next.as_ref().unwrap();

            if std::ptr::eq(
                maybe_next.as_ref().unwrap().as_ptr(),
                self.maybe_tail.as_ref().unwrap().as_ptr(),
            ) {
                borrowed_node.maybe_next = None;
                let old_tail = self.maybe_tail.replace(Rc::clone(&rc));
                self.length -= 1;
                return Some(ListUtils::unpack(old_tail).value);
            } else {
                opt = Some(Rc::clone(next_rc));
            }
        }
    }

    fn remove_nth(&mut self, index: usize) -> Option<T> {
        if index == 0 {
            return self.remove_first();
        } else if index == self.length - 1 {
            return self.remove_last();
        } else if index >= self.length {
            return None;
        }

        let mut opt: Option<Rc<RefCell<Node<T>>>> =
            Some(Rc::clone(self.maybe_head.as_ref().unwrap()));
        let mut rc: Rc<RefCell<Node<T>>>;
        let mut countdown_to_preceding = index as i32 - 1;

        #[allow(unused_must_use)]
        loop {
            rc = Rc::clone(opt.as_ref().unwrap());
            let borrowed_node = &mut rc.borrow_mut().maybe_next;
            if countdown_to_preceding == 0 {
                let extracted = borrowed_node.take();
                let extracted_node = ListUtils::unpack(extracted);
                mem::replace(borrowed_node, extracted_node.maybe_next);
                self.length -= 1;
                return Some(extracted_node.value);
            } else {
                opt.replace(Rc::clone(borrowed_node.as_ref().unwrap()));
                countdown_to_preceding -= 1;
            }
        }
    }

    fn remove_first_matching<F>(&mut self, f: F) -> Option<T>
    where
        F: Fn(&T) -> bool,
    {
        let mut opt: Option<Rc<RefCell<Node<T>>>>;
        let mut rc: Rc<RefCell<Node<T>>>;

        match &self.maybe_head {
            None => {
                return None;
            }
            Some(rc) => {
                if f(&rc.borrow().value) {
                    return self.remove_first();
                } else {
                    opt = Some(Rc::clone(rc));
                }
            }
        }

        loop {
            rc = Rc::clone(opt.as_ref().unwrap());
            let mut borrowed_node = RefCell::borrow_mut(&rc);
            let maybe_next = &mut borrowed_node.maybe_next;

            if maybe_next.is_none() {
                return None;
            } else {
                let next_rc = maybe_next.as_ref().unwrap();
                if f(&next_rc.borrow().value) {
                    #[allow(unused_must_use)]
                    if std::ptr::eq(
                        maybe_next.as_ref().unwrap().as_ptr(),
                        self.maybe_tail.as_ref().unwrap().as_ptr(),
                    ) {
                        borrowed_node.maybe_next = None;
                        let old_tail = self.maybe_tail.replace(Rc::clone(&rc));
                        self.length -= 1;
                        return Some(ListUtils::unpack(old_tail).value);
                    } else {
                        let extracted_opt = maybe_next.take();
                        let extracted_node = ListUtils::unpack(extracted_opt);
                        mem::replace(maybe_next, extracted_node.maybe_next);
                        self.length -= 1;
                        return Some(extracted_node.value);
                    }
                } else {
                    opt.replace(Rc::clone(next_rc));
                }
            }
        }
    }

    fn replace_nth(&mut self, index: usize, t: T) -> Result<T, T> {
        if self.length == 0 || index >= self.length {
            return Err(t);
        }

        if self.length == 1 {
            let new_node = Rc::new(RefCell::new(Node {
                value: t,
                maybe_next: None,
            }));
            let ret = self.maybe_tail.replace(Rc::clone(&new_node));
            self.maybe_head = Some(new_node);
            return Ok(ListUtils::unpack(ret).value);
        }

        if index == 0 {
            let old_head = ListUtils::unpack(self.maybe_head.take());
            let new_head = Rc::new(RefCell::new(Node {
                value: t,
                maybe_next: old_head.maybe_next,
            }));
            self.maybe_head.replace(new_head);
            return Ok(old_head.value);
        }

        let mut opt = Some(Rc::clone(self.maybe_head.as_ref().unwrap()));
        let mut rc: Rc<RefCell<Node<T>>>;
        let mut countdown = index - 1;

        loop {
            rc = Rc::clone(opt.as_ref().unwrap());
            let mut borrowed_node = rc.borrow_mut();
            let maybe_next = &mut borrowed_node.maybe_next;

            if countdown == 0 {
                let wrapped_rc = maybe_next.as_ref().unwrap();
                #[allow(unused_must_use)]
                if std::ptr::eq(
                    wrapped_rc.as_ptr(),
                    self.maybe_tail.as_ref().unwrap().as_ptr(),
                ) {
                    maybe_next.take();
                    let old_node = ListUtils::unpack(self.maybe_tail.take());
                    let new_node = Rc::new(RefCell::new(Node {
                        value: t,
                        maybe_next: old_node.maybe_next,
                    }));
                    let new_node_clone = Rc::clone(&new_node);
                    self.maybe_tail = Some(new_node);
                    maybe_next.replace(new_node_clone);
                    return Ok(old_node.value);
                } else {
                    let old_node = ListUtils::unpack(maybe_next.take());
                    let new_node = Rc::new(RefCell::new(Node {
                        value: t,
                        maybe_next: old_node.maybe_next,
                    }));
                    maybe_next.replace(new_node);
                    return Ok(old_node.value);
                }
            } else {
                opt.replace(Rc::clone(maybe_next.as_ref().unwrap()));
                countdown -= 1;
            }
        }
    }
}
