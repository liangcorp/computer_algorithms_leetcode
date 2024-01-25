mod print;

use std::cmp::Ordering;

#[derive(Debug)]
pub struct Node {
    data: i32,
    left: Pointer,
    right: Pointer,
}

type Pointer = Option<Box<Node>>;

impl Node {
    pub fn new(data: i32) -> Node {
        Node {
            data,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, data: i32) {
        match data.cmp(&self.data) {
            Ordering::Less => match self.left.as_mut() {
                Some(left) => left.insert(data),
                None => {
                    self.left = Some(Box::new(Node::new(data)));
                }
            },
            Ordering::Greater => match self.right.as_mut() {
                Some(left) => left.insert(data),
                None => {
                    self.right = Some(Box::new(Node::new(data)));
                }
            },
            Ordering::Equal => (),
        }
    }

    pub fn tree_height(&self) -> i32 {
        let l_depth = match self.left.as_ref() {
            Some(left) => left.tree_height(),
            None => 0,
        };

        let r_depth = match self.right.as_ref() {
            Some(right) => right.tree_height(),
            None => 0,
        };

        if l_depth > r_depth {
            l_depth + 1
        } else {
            r_depth + 1
        }
    }

    pub fn node_count(&self) -> i32 {
        let l_node_count = match self.left.as_ref() {
            Some(left) => left.node_count(),
            None => 0,
        };

        let r_node_count = match self.right.as_ref() {
            Some(right) => right.node_count(),
            None => 0,
        };

        l_node_count + r_node_count + 1
    }

    pub fn min_value(&self) -> i32 {
        match self.left.as_ref() {
            Some(left) => left.min_value(),
            None => self.data,
        }
    }

    // DONE: Node to be deleted is the leaf node:
    //  Its simple you can just null it out.
    //
    // @TODO: Node to be deleted has one child:
    //  You can just replace the node with the child node.
    //
    // @TODO: Node to be deleted has two child:
    //  Need to figure out what will be the
    //  replacement of the node to be deleted.
    //  Want minimal disruption to the existing tree structure
    //  Can table the replacement node from the deleted
    //      nodes left or right subtree.
    //  If taking if from the left subtree,
    //      we have to take the largest value in the left subtree.
    //  If taking if from the right subtree,
    //      we have to take the smallest value in the right subtree.
    //  Choose one approach and stick to it.
    pub fn delete(&mut self, data: i32) {
        // Only delete leaf node at the moment.
        // Taking a break from borrow checker hell
        match data.cmp(&self.data) {
            Ordering::Less => {
                if let Some(left) = self.left.as_mut() {
                    if left.as_mut().data == data
                        && left.as_mut().left.is_none()
                        && left.as_mut().right.is_none()
                    {
                        self.left = None;
                    } else {
                        left.delete(data);
                    }
                }

                if let Some(right) = self.right.as_mut() {
                    if right.as_mut().data == data
                        && right.as_mut().left.is_none()
                        && right.as_mut().right.is_none()
                    {
                        self.right = None;
                    } else {
                        right.delete(data);
                    }
                }
            }
            Ordering::Greater => {
                if let Some(left) = self.left.as_mut() {
                    if left.as_mut().data == data
                        && left.as_mut().left.is_none()
                        && left.as_mut().right.is_none()
                    {
                        println!("{}", left.as_mut().data);
                        self.left = None;
                    } else {
                        left.delete(data);
                    }
                }

                if let Some(right) = self.right.as_mut() {
                    if right.as_mut().data == data
                        && right.as_mut().left.is_none()
                        && right.as_mut().right.is_none()
                    {
                        self.right = None;
                    } else {
                        right.delete(data);
                    }
                }
            }

            Ordering::Equal => {
                if self.left.is_some() && self.right.is_none() {
                    self.left = None;
                } else if self.left.is_none() && self.right.is_some() {
                    self.right = None;
                }
            }
        }
    }
}
