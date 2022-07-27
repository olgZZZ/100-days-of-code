use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    // Delete this field
    // dummy is needed to avoid unused parameter error during compilation
    // dummy: ::std::marker::PhantomData<T>,
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut node = &self.head;
        while node.is_some() {
            node = node.as_ref().map(|boxed| &boxed.next).unwrap();
            len += 1;
        }
        len
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node { data: element, next: self.head.take() }));
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut head = self.head.take();
        self.head = match head {
            Some(ref mut node) => node.next.take(),
            None => None,
        };
        head.map(|boxed| boxed.data)
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|boxed| &boxed.data)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();
        let mut node = &self.head;
        while let &Some(ref boxed) = node {
            list.push(boxed.data.clone());
            node = &boxed.next;
        }
        list
    }
}

impl<T: Clone> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for value in iter {
            list.push(value);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T: Clone> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = vec![];
        while let Some(value) = linked_list.pop() {
            vec.insert(0, value);
        }
        vec
    }
}
