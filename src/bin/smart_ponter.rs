use std::ops::Deref;

use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    mutable_tree();
}

fn box_pointer() {
    let b = Box::new(5);
    println!("b = {}", b)
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn cons_list() -> List {
    Cons(
        1, Box::new(Cons(
            2, Box::new(Cons(
                3, Box::new(Nil))))))
}

#[derive(Debug)]
struct Mp3 {
    audio: Vec<u8>,
    artist: Option<String>,
    title: Option<String>,
}

impl Deref for Mp3 {
    type Target = Vec<u8>;

    fn deref(&self) -> &Vec<u8> {
        &self.audio
    }
}

fn deref_smart_pointers() {
    let my_favorite_song = Mp3 {
        audio: vec![1, 2, 3],
        artist: Some(String::from("Nirvana")),
        title: Some(String::from("Smells like Teen Spirit")),
    };

    println!("{:?}", *my_favorite_song);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer");
    }
}

fn drop_custom_smart_ptr() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    println!("Wait for it...")
}

enum Tree {
    Cons(i32, Rc<Tree>),
    Nil
}

fn cons_tree() {
    let a = Rc::new(Tree::Cons(
        5, Rc::new(Tree::Cons(
            10, Rc::new(Tree::Nil)))));
    println!("rc = {}", Rc::strong_count(&a));
    let b = Tree::Cons(3, a.clone());
    println!("rc after creating b = {}", Rc::strong_count(&a));
    {
        let c = Tree::Cons(4, a.clone());
        println!("rc after creating c = {}", Rc::strong_count(&a));
    }
    println!("rc after c goes out of scope = {}", Rc::strong_count(&a));
}

fn immutably_borrows(a: &i32) {
    println!("a is {}", a);
}

fn mutably_borrows(b: &mut i32) {
    *b += 1;
}

fn demo(r: &RefCell<i32>) {
    immutably_borrows(&r.borrow());
    mutably_borrows(&mut r.borrow_mut());
    immutably_borrows(&r.borrow());
}

fn runtime_borrow_with_refcell() {
    let data = RefCell::new(5);
    demo(&data);
}

#[derive(Debug)]
enum MTree {
    Cons(Rc<RefCell<i32>>, Rc<MTree>),
    Nil,
}

fn mutable_tree() {
    let value = Rc::new(RefCell::new(5));

    let a = MTree::Cons(value.clone(), Rc::new(MTree::Nil));
    let shared_list = Rc::new(a);

    let b = MTree::Cons(
        Rc::new(RefCell::new(6)), shared_list.clone());
    let c = MTree::Cons(
        Rc::new(RefCell::new(10)), shared_list.clone());

    *value.borrow_mut() += 10;

    println!("shared list after = {:?}", shared_list);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}