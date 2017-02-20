type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;
            node.elem // map() returns an option already
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        // `while let` means "do this thing until this pattern doesn't match".
        // `boxed_node` is a pattern variable here.
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
            // boxed_node goes out of scope and gets dropped here,
            // but it's Node's `next` field has been set to Link::Empty
            // and cur_link is assigned the value of what `next` had previously
            // been, so no unbounded recursion occurs.
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // check empty list behaves right
        assert_eq!(list.pop(), None);

        // push it real good
        list.push(1);
        list.push(2);
        list.push(3);

        // check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // push it real gooder
        list.push(4);
        list.push(5);

        // normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));

        // exhausted removal
        assert_eq!(list.pop(), None);
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peekaboo() {
        let mut list = List::new();

        list.push(1);
        list.push(2);

        assert_eq!(list.peek(), Some(&2));

        {
            let head: &mut u32 = list.peek_mut().unwrap();
            *head = 3;
        }

        assert_eq!(list.peek(), Some(&3));

    }
}
