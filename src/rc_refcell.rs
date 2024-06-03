use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

pub struct List<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    prev: Link<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            front: None,
            back: None,
            len: 0,
        }
    }
    pub fn push_front(&mut self, elem: T) {
        let new = Node::new(elem);

        match self.front.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new.clone());
                new.borrow_mut().next = Some(old_head);
            }
            None => {
                self.back = Some(new.clone());
            }
        };
        self.front = Some(new);
        self.len += 1;
    }
    pub fn push_back(&mut self, elem: T) {
        let new = Node::new(elem);

        match self.back.take() {
            Some(old_back) => {
                old_back.borrow_mut().next = Some(new.clone());
                new.borrow_mut().prev = Some(old_back);
            }
            None => {
                self.front = Some(new.clone());
            }
        };
        self.back = Some(new);
        self.len += 1;
    }
    pub fn pop_front(&mut self) -> Option<T> {
        self.front.take().map(|old_front| {
            match old_front.borrow_mut().next.take() {
                Some(new_front) => {
                    new_front.borrow_mut().prev.take();
                    self.front = Some(new_front);
                }
                None => {
                    self.back.take();
                }
            }
            self.len -= 1;
            Rc::try_unwrap(old_front).ok().unwrap().into_inner().elem
        })
    }
    pub fn pop_back(&mut self) -> Option<T> {
        self.back.take().map(|old_back| {
            match old_back.borrow_mut().prev.take() {
                Some(new_back) => {
                    new_back.borrow_mut().next.take();
                    self.back = Some(new_back);
                }
                None => {
                    self.front.take();
                }
            }
            self.len -= 1;
            Rc::try_unwrap(old_back).ok().unwrap().into_inner().elem
        })
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn front(&self) -> Option<Ref<T>> {
        self.front
            .as_ref()
            .map(|e| Ref::map(e.borrow(), |e| &e.elem))
    }
    pub fn front_mut(&mut self) -> Option<RefMut<T>> {
        self.front
            .as_ref()
            .map(|e| RefMut::map(e.borrow_mut(), |e: &mut Node<T>| &mut e.elem))
    }
    pub fn back(&self) -> Option<Ref<T>> {
        self.back
            .as_ref()
            .map(|e| Ref::map(e.borrow(), |e| &e.elem))
    }
    pub fn back_mut(&mut self) -> Option<RefMut<T>> {
        self.back
            .as_ref()
            .map(|e| RefMut::map(e.borrow_mut(), |e: &mut Node<T>| &mut e.elem))
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn clear(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> Extend<T> for List<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item);
        }
    }
}
impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut new_list = Self::new();
        new_list.extend(iter);
        new_list
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

pub struct IntoIter<T> {
    list: List<T>,
}

impl<T> IntoIterator for List<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { list: self }
    }
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len, Some(self.list.len))
    }
}
impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}
impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.list.len
    }
}

unsafe impl<T: Send> Send for List<T> {}
unsafe impl<T: Sync> Sync for List<T> {}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_basic_front() {
        let mut list = List::new();

        // Try to break an empty list
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Try to break a one item list
        list.push_front(10);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Mess around
        list.push_front(10);
        assert_eq!(list.len(), 1);
        list.push_front(20);
        assert_eq!(list.len(), 2);
        list.push_front(30);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(30));
        assert_eq!(list.len(), 2);
        list.push_front(40);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(40));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_basic() {
        let mut m = List::new();
        assert_eq!(m.pop_front(), None);
        assert_eq!(m.pop_back(), None);
        assert_eq!(m.pop_front(), None);
        m.push_front(1);
        assert_eq!(m.pop_front(), Some(1));
        m.push_back(2);
        m.push_back(3);
        assert_eq!(m.len(), 2);
        assert_eq!(m.pop_front(), Some(2));
        assert_eq!(m.pop_front(), Some(3));
        assert_eq!(m.len(), 0);
        assert_eq!(m.pop_front(), None);
        m.push_back(1);
        m.push_back(3);
        m.push_back(5);
        m.push_back(7);
        assert_eq!(m.pop_front(), Some(1));

        let mut n = List::new();
        n.push_front(2);
        n.push_front(3);
        {
            assert_eq!(*n.front().unwrap(), 3);
            let mut x = n.front_mut().unwrap();
            assert_eq!(*x, 3);
            *x = 0;
        }
        {
            assert_eq!(*n.back().unwrap(), 2);
            let mut y = n.back_mut().unwrap();
            assert_eq!(*y, 2);
            *y = 1;
        }
        assert_eq!(n.pop_front(), Some(0));
        assert_eq!(n.pop_front(), Some(1));
    }
}

#[allow(dead_code)]
fn assert_properties() {
    fn is_send<T: Send>() {}
    fn is_sync<T: Sync>() {}

    is_send::<List<i32>>();
    is_sync::<List<i32>>();

    is_send::<IntoIter<i32>>();
    is_sync::<IntoIter<i32>>();
}
