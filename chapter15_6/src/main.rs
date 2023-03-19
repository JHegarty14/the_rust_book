use crate::List::{Cons, Nil};
use std::borrow::{BorrowMut, Borrow};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// second element is wrapped in a RefCell so we have the ability to modify it
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    /// access the send item in a cons list if it exists
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

/// each instance of a node owns its children, and that ownership can be shared
/// to allow direct access to each Node in the tree
#[derive(Debug)]
struct UnidirectionalNode {
    value: i32,
    children: RefCell<Vec<Rc<UnidirectionalNode>>>,
}

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b created = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // println!("a next item = {:?}", a.tail());
    // not flagged at compile time bc RefCell implements `unsafe`, but will overflow at runtime
    // circular reference between a.tail and b.tail
    //          +---+
    //          | b | ----------------+           
    //          +---+                 |
    //  +---+                         |
    //  | a | ---------+              |
    //  +---+          |              |
    //                 V              V
    //             +------+      +-------+
    //         +-> | 5 |  |----->| 10 |  | ----+
    //         |   +------+      +-------+     |
    //         +-------------------------------+

    // Weak references can prevent references cycles
    // Rc::clone creates a strong reference and increments the strong_count of an Rc<T> instance
    // calling Rc::downgrade can create a weak reference (Weak<T>) from an Rc<T> instance and increments the weak_count by 1
    // a Weak<T> instance may reference a dropped value, you must confirm the value exists before you do anything with it

    let uni_leaf = Rc::new(UnidirectionalNode {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let uni_branch = Rc::new(UnidirectionalNode {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&uni_leaf)]),
    });

    println!("uni_leaf: {:?}", uni_leaf);
    println!("uni_branch: {:?}", uni_branch);
    // leaf has two owners: leaf and branch. branch can reference leaf with a strong reference because leaf doesn't reference branch
    // but what if the relationship goes both ways, like if leaf references branch as a parent Node?

    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });
    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });

    println!("leaf parent before = {:?}", leaf.parent.borrow().upgrade());

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent after = {:?}", leaf.parent.borrow().upgrade());

    let leaf2 = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!("leaf2 strong = {}, weak = {}", Rc::strong_count(&leaf2), Rc::weak_count(&leaf2));
    // leaf2 strong = 1, weak = 0

    {
        let branch2 = Rc::new(Node {
            value: 5,
            children: RefCell::new(vec![Rc::clone(&leaf2)]),
            parent: RefCell::new(Weak::new()),
        });
        *leaf2.parent.borrow_mut() = Rc::downgrade(&branch2);

        println!("branch2 strong = {}, weak = {}", Rc::strong_count(&branch2), Rc::weak_count(&branch2));
        // branch2 strong = 1, weak = 1

        println!("in closure leaf2 strong = {}, weak = {}", Rc::strong_count(&leaf2), Rc::weak_count(&leaf2));
        // in closure leaf2 strong = 2, weak = 0
    } // <--- when inner scope ends, branch2 goes out of scope and the strong count of the Rc<Node> goes to 0 and is dropped
    // leaf2.weak_count has no bearing on whether the Node can be dropped, so there are no memory leaks

    println!("leaf2 parent = {:?}", leaf2.parent.borrow().upgrade());
    // leaf2 parent = None

    println!("ending leaf2 strong = {}, weak = {}", Rc::strong_count(&leaf2), Rc::weak_count(&leaf2));
    // ending leaf2 strong = 1, weak = 0
}
