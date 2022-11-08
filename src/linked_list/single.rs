pub mod simple {
    use std::mem;

    struct Node {
        data: i32,
        next: Link,
    }
    enum Link {
        Empty,
        More(Box<Node>),
    }
    pub struct List {
        head: Link,
    }
    impl List {
        pub fn new() -> Self {
            List { head: Link::Empty }
        }
        pub fn push(&mut self, data: i32) {
            let n = Node {
                data: data,
                //this is pretty extreme for a simple list.
                //we can't move the value of self.head directly bc
                //it will leave self in undefined, unintialize state
                //temporarily set it to empty to empty, while obtaining the value
                next: mem::replace(&mut self.head, Link::Empty),
            };
            self.head = Link::More(Box::new(n));
        }
        pub fn pop(&mut self) -> Option<i32> {
            match mem::replace(&mut self.head, Link::Empty) {
                Link::Empty => None,
                Link::More(head) => {
                    self.head = head.next;
                    Some(head.data)
                }
            }
        }
    }

    impl Drop for List {
        fn drop(&mut self) {
            //iterate thru all the links and set them to empty
            let mut cur_link = mem::replace(&mut self.head, Link::Empty);
            while let Link::More(mut boxed_node) = cur_link {
                cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
                //since we are in the destructor and boxed node is going out of scope,
                // it will be dropped. we set the next of boxed_node to empty so there
                // is no infinite recursion
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::List;
        #[test]
        fn test_list() {
            let mut list = List::new();
            assert_eq!(list.pop(), None);
            list.push(1);
            list.push(2);
            list.push(3);
            assert_eq!(list.pop(), Some(3));
            assert_eq!(list.pop(), Some(2));
            list.push(5);
            assert_eq!(list.pop(), Some(5));
            assert_eq!(list.pop(), Some(1));
            assert_eq!(list.pop(), None);
        }
    }
}

pub mod v2 {
    type Link<T> = Option<Box<Node<T>>>;
    pub struct List<T> {
        head: Link<T>,
    }
    struct Node<T> {
        data: T,
        next: Link<T>,
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            List { head: None }
        }
        pub fn push(&mut self, data: T) {
            let n = Node {
                data: data,
                next: self.head.take(),
            };
            self.head = Some(Box::new(n))
        }
        pub fn pop(&mut self) -> Option<T> {
            self.head.take().map(|node| {
                self.head = node.next;
                node.data
            })
        }
        pub fn peek(&self) -> Option<&T> {
            /*
            match &self.head {
                None => None,
                Some(node) => Some(&node.data),
            }
            */
            self.head.as_ref().map(|node| &node.data)
        }
        pub fn peek_mut(&mut self) -> Option<&mut T> {
            self.head.as_mut().map(|node| &mut node.data)
        }
        // into_iter returns an IntoIter
        pub fn into_iter(self) -> IntoIter<T> {
            IntoIter(self)
        }

        //iter returns a iterator
        // given that this in an implementation for a generic over
        // some lifetime, need the lifetime declaration here
        // to ensure self lives as long as the returned iterator
        pub fn iter<'a>(&'a self) -> ListIter<'a, T> {
            ListIter {
                // head is an option to a box
                // we want to obtain a reference to what is pointed to by the box
                // we cannot simply map the head, because that would move
                // the content of the option
                // instead, get a reference to the content of the option, i.e.
                // a reference to the box.
                // then, to unwind and obtain a reference to what is pointed at by the box
                // 1. deference the content in the option, i.e deref &Box -> Box
                // 2. deref the Box<T> -> <T>
                // 3. take a ref to <T>
                // ==> &( * ( *option_content)) == &**boxed_node
                next: self.head.as_ref().map(|boxed_node| &**boxed_node),
            }
            //in the latest versions of Rust, we can avoid the as_ref.map(... &**)
            //by using as_deref. This act like as_ref in such much as it
            //returns a ref to the content of the option, but it also
            //coerces the content via dereferencing on oue behalf

            //next: self.head.as_deref() //Wow that is much nicer to write...

            /* long form for my own practixe
            next: match self.head.as_ref() {
                None => None,
                Some(boxed_node) => {
                    Some(&**boxed_node)
                },
            }
            */
        }

        //iter_mut returns a iterator of mutable references
        pub fn iter_mut(&mut self) -> IterMut<'_, T> {
            IterMut {
                next: self.head.as_deref_mut(),
            }
        }
    }
    //tuple struct for simple wrapper
    pub struct IntoIter<T>(List<T>);

    impl<T> Iterator for IntoIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            self.0.pop()
        }
    }

    pub struct ListIter<'a, T> {
        next: Option<&'a Node<T>>,
    }

    impl<'a, T> Iterator for ListIter<'a, T> {
        //need the lifetime in the type declartion
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            // self.next is Option to a (lifetime'd) Node reference
            // we want to return obtain a reference to the data
            // for that Node
            // we also must update self.next such that it is an option
            // to the next node reference

            //obtaining a reference to the data is easy enough
            //use a map of next to get the node reference and return a ref to the data
            //updating next is more comples
            // with the said map, we have a ref to a node
            // we need to obtain an Option to reference that is this node's next (n')
            // we can get an Option to n' via node.next
            // since we want to reference the content of the option, use as_ref
            // this gives us the same thing as iter impl of List: a ref to a Box<Node>
            // need to deref the ref
            //      deref the box
            //      ref the resulting node
            self.next.map(|node| {
                //self.next = node.next.as_ref().map(|next_boxed_node| &**next_boxed_node);
                // in new Rust, we can do the same thing more simpy using as_deref
                self.next = node.next.as_deref();
                &node.data
            })
        }
    }

    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            let mut cur_link = self.head.take();
            while let Some(mut boxed_node) = cur_link {
                cur_link = boxed_node.next.take();
            }
        }
    }
    //iterator over mutable references to a Node
    pub struct IterMut<'a, T> {
        next: Option<&'a mut Node<T>>,
    }

    impl<'a, T> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;
        fn next(&mut self) -> Option<Self::Item> {
            self.next.take().map(|node| {
                self.next = node.next.as_deref_mut();
                &mut node.data
            })
        }
    }
    #[cfg(test)]
    mod test {
        use super::List;
        #[test]
        fn test_list_v2() {
            let mut list = List::new();
            assert_eq!(list.pop(), None);
            list.push(1);
            list.push(2);
            list.push(3);
            assert_eq!(list.pop(), Some(3));
            assert_eq!(list.pop(), Some(2));
            list.push(5);
            assert_eq!(list.pop(), Some(5));
            assert_eq!(list.pop(), Some(1));
            assert_eq!(list.pop(), None);

            list.push(7);
            assert_eq!(list.peek(), Some(&7));
            let x = list.peek_mut();
            assert_eq!(x, Some(&mut 7));

            x.map(|v| *v = 71);
            assert_eq!(list.peek(), Some(&71));
            assert_eq!(list.pop(), Some(71));
        }

        #[test]
        fn test_into_iter() {
            let mut list = List::new();
            let mut i = 0;
            while i < 4 {
                list.push(i);
                i += 1;
            }
            let mut iter = list.into_iter();
            while i > 0 {
                i -= 1;
                assert_eq!(iter.next(), Some(i));
            }
            assert_eq!(iter.next(), None)
        }

        #[test]
        fn test_iter() {
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
            let x = iter.next();
            assert_eq!(x, Some(&mut 3));

            assert_eq!(iter.next(), Some(&mut 2));
            assert_eq!(iter.next(), Some(&mut 1));
        }
    }
}
