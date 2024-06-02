pub struct List {
    head: Link,
}

#[derive(Clone)]
enum Link {
    Empty,
    Elem(Box<Node>),
}

#[derive(Clone)]
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem,
            next: std::mem::replace(&mut self.head, Link::Empty),
        };
        self.head = Link::Elem(Box::new(new_node));
    }
    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::Elem(e) => {
                self.head = e.next;
                Some(e.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur = std::mem::replace(&mut self.head, Link::Empty);
        while let Link::Elem(mut e) = cur {
            cur = std::mem::replace(&mut e.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn long_list() {
        let mut list = List::new();
        for i in 0..100000 {
            list.push(i);
        }
        drop(list);
    }
}