/// Shared ownership via a persistent immutable singly-linked list

pub mod stack {
    use std::rc::Rc;

    pub struct List<T> {
        head: Link<T>,
    }
    // Link is an option to a reference counted node so that
    // we can have shared access. this guarantees memory safety
    // when there are no more references, but limits us
    // to shared references to the internal Node, and precludes
    // mutation.
    type Link<T> = Option<Rc<Node<T>>>;
    struct Node<T> {
        elem: T,
        next: Link<T>,
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            List { head: None }
        }

        /// prepend takes a list and an element and returns a List whose head is the
        /// new element and whose next the input list
        // since prepend must return a list with new content, and since we only have
        // shared references (so we can't must anything), we need mechanism to
        // create new reference to the existing list and shove it into next
        // This is accomplished with the Clone trait which does "get another one like this one"
        // and is logically comparable to the copy constuctor in C++ but only explicit invoked
        // In particular, RC uses Clone to increase the reference count
        pub fn prepend(&self, elem: T) -> List<T> {
            List {
                head: Some(Rc::new(Node {
                    elem: elem,
                    next: self.head.clone(),
                })),
            }
        }

        /// tail takes a list and returns the whole list except the first element
        pub fn tail(&self) -> List<T> {
            List {
                head: self
                    .head
                    .as_ref()
                    .and_then(|ref_rc_node| ref_rc_node.next.clone()),
            }
        }

        /// returns a reference to the first element
        pub fn head(&self) -> Option<&T> {
            self.head.as_ref().map(|node| &node.elem)
        }

        pub fn iter(&mut self) -> Iter<'_, T> {
            Iter {
                next: self.head.as_deref(),
            }
        }
    }
    // like with the mutable list, there is a recursive destructor problem. perhaps
    // it's not quite a bad as that case because it is bounded if the recursion hits
    // the head of another list (since the drop'ing will stop there).
    // in the previous code we mutated the Node inside the Box. That is illegal here
    // Logically, it is ok to drop the head of the List if and only if we (somehow)
    // know that there are no other references to the node that is the current head.
    // Similarly, given such knowledge, we would understand to stop drop'ing if
    // any references exist.

    // In practice, Rc provides this logic construct via try_unwrap
    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            let mut head = self.head.take();
            while let Some(node) = head {
                if let Ok(mut node) = Rc::try_unwrap(node) {
                    head = node.next.take()
                } else {
                    break;
                }
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
    #[cfg(test)]
    mod test {
        use super::List;
        #[test]
        fn test_stack() {
            let list = List::new();
            assert_eq!(list.head(), None);

            let list = list.prepend(1).prepend(2).prepend(3);
            assert_eq!(list.head(), Some(&3));

            let list = list.tail();
            assert_eq!(list.head(), Some(&2));

            let list = list.tail();
            assert_eq!(list.head(), Some(&1));

            let list = list.tail();
            assert_eq!(list.head(), None);

            // test empty tail
            let list = list.tail();
            assert_eq!(list.head(), None);
        }
    }
}
