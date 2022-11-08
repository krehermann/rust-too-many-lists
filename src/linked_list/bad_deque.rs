//use std::borrow::BorrowMut;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
///Disclamer: this is an illustation of a poorly considered idea

pub struct DLList<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            next: None,
            prev: None,
        }))
    }
}

///DLList is a doubly linked list
// the implementation is more complex than singly linked list
// a good guide post is to keep the invariant that each and every
// node in the list is pointed to twice: interior nodes are pointed
// to by prev and next, while boundary nodes are pointed to from one
// elem in the list and the list itself
impl<T> DLList<T> {
    pub fn new() -> Self {
        DLList {
            head: None,
            tail: None,
        }
    }
    pub fn push_front1(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }
    
    pub fn push_front(&mut self, elem: T) {
        //new node needs 2 new links, all else should not change number of links
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone()); // +1 pointer to new_head
                new_head.borrow_mut().next = Some(old_head); // +1 old_head
                self.head = Some(new_head); // +1 new_head, -1 old_head
            }
            None => {
                //empty list
                self.tail = Some(new_head.clone()); // +1 new_head
                self.head = Some(new_head); // +1 new_head
               
            }
        }
    }
/*
    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            Some(old_head) => {
                self.head = old_head.borrow_mut().next.as_ref();
                Some(old_head.borrow().elem)
            }
            None => None
            
        }
    }
    */
}
