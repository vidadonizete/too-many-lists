use std::ops::AddAssign;

type Maybe<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Maybe<T>,
}

struct Node<T> {
    elem: T,
    next: Maybe<T>,
}

impl<T> Node<T> {
    fn new(elem: T, next: Maybe<T>) -> Self {
        Node { elem, next }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let node = Node::new(elem, self.head.take());
        self.head = Some(Box::new(node))
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> Iterator for List<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl<T> AddAssign<T> for List<T> {
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        self.push(rhs)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut maybe = self.head.take();
        // using while will not overflow stack,
        // since there is only once a local variable
        while let Some(mut node) = maybe {
            maybe = node.next.take();
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn should_push_properly() {
        let mut list = List::<u8>::new();

        assert_eq!(list.pop(), None);

        list += 2;
        list += 4;
        list += 6;

        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(4));

        list += 12;
        list += 13;
        list += 14;

        assert_eq!(list.pop(), Some(14));
        assert_eq!(list.pop(), Some(13));
        assert_eq!(list.pop(), Some(12));
        assert_eq!(list.pop(), Some(2));

        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();

        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list += 1;
        list += 2;
        list += 3;

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.pop();

        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.peek_mut(), Some(&mut 2));

        list.peek_mut().map(|value| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.peek_mut(), Some(&mut 42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn should_iterate_over_list() {
        let mut list = List::<u8>::new();

        assert_eq!(list.pop(), None);

        list += 2;
        list += 4;
        list += 6;

        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
