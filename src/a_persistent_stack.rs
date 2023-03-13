use std::{rc::Rc, ops::Deref};

type Link<T> = Option<Rc<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T, next: Link<T>) -> Self {
        Node { elem, next }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> Self {
        Self {
            head: Some(Rc::new(Node::new(elem, self.head.clone()))),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn tail(&self) -> Self {
        Self {
            head: self.head.as_ref().and_then(|node| node.next.clone())
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
}
