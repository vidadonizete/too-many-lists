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
        use std::mem::replace;
        let prev = replace(&mut self.head, None);
        let new = Node::new(elem, prev);
        self.head = Some(Box::new(new))
    }

    pub fn pop(&mut self) -> Option<T> {
        use std::mem::replace;
        match replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
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
        use std::mem::replace;
        let mut maybe = replace(&mut self.head, None);
        // using while will not overflow stack,
        // since there is only once a local variable
        while let Some(mut node) = maybe {
            maybe = replace(&mut node.next, None);
        }
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
}
