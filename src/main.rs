use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    left: Rc<RefCell<Option<Node>>>,
    right: Rc<RefCell<Option<Node>>>,
}

impl Node {
    fn new(value: i32) -> Node {
        Node { value, left: Rc::new(RefCell::new(None)), right: Rc::new(RefCell::new(None)) }
    }
}

struct BST {
    root: Rc<RefCell<Option<Node>>>,
}

impl BST {
    fn new(value: i32) -> BST {
        BST { root: Rc::new(RefCell::new(Some(Node::new(value)))) }
    }

    fn insert(&mut self, value: i32) -> &str {
        let insert_result = self.insert_helper(value, Rc::clone(&self.root));

        let r = match insert_result {
            Ok(()) => "insert sucessfull",
            Err(err) => err,
        };

        r
    }

    fn insert_helper(&self, value: i32, node: Rc<RefCell<Option<Node>>>) -> Result<(), &str> {
        if node.borrow().is_none() {
            *node.borrow_mut() = Some(Node::new(value));
            return Ok(());
        }
        
        let next_node = {
            let curr_value = node.borrow().as_ref().unwrap().value;
            if value > curr_value {
                Some(Rc::clone(&node.borrow().as_ref().unwrap().right))
            } else if value < curr_value {
                Some(Rc::clone(&node.borrow().as_ref().unwrap().left))
            } else {
                None
            }
        };

        if next_node.is_some() {
            self.insert_helper(value, next_node.unwrap())
        } else {
            Err("duplicate key")
        }
    }

    fn inorder_string(&self) -> String {
        let mut s = String::from("");
        self.inorder_string_helper(&mut s, Rc::clone(&self.root));
        s
    }

    fn inorder_string_helper(&self, s: &mut String, node: Rc<RefCell<Option<Node>>>) {
        if node.borrow().is_none() {
            return;
        }
        self.inorder_string_helper(s, Rc::clone(&node.borrow().as_ref().unwrap().left));
        s.push_str(format!("{} ", node.borrow().as_ref().unwrap().value).as_str());
        self.inorder_string_helper(s, Rc::clone(&node.borrow().as_ref().unwrap().right));
    }
}

fn main() {
    let mut bst = BST::new(5);
    bst.insert(3);
    bst.insert(7);
    bst.insert(1);
    bst.insert(4);
    bst.insert(6);
    bst.insert(8);
    assert_eq!(bst.inorder_string(), "1 3 4 5 6 7 8 ");
}
