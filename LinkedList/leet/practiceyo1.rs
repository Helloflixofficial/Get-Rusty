#[derive(Debug, PartialEq, Eq)]
struct Tree {
    root: Option<Box<Node>>,
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

impl From<Node> for Option<Box<Node>> {
    fn from(node: Node) -> Self {
        Some(Box::new(node))
    }
}

impl Tree {
    fn new() -> Self {
        Tree { root: None }
    }

    fn insert(&mut self, value: i32) {
        // match &mut self.root {
        //     None => {
        //         self.root = Node::new(value).into();
        //     }

        //     Some(node) => {
        //         Tree::insert_recursive(node, value);
        //     }
        // }
        self.insert_iterative(value);
    }

    //secount one

    fn insert_iterative(&mut self, value: i32) {
        if self.root.is_none() {
            self.root = Node::new(value).into();
            return;
        }
        let mut q: Vec<&mut Box<Node>> = Vec::new();
        let root = self.root.as_mut().unwrap();
        q.push(root);

        while let Some(node) = q.pop() {
            if value > node.value {
                let right = &mut node.right;
                match right {
                    None => {
                        *right = Node::new(value).into();
                    }
                    Some(n) => {
                        q.push(n);
                    }
                }
            } else if value < node.value {
                let left = &mut node.left;
                match left {
                    None => {
                        *left = Node::new(value).into();
                    }
                    Some(n) => {
                        q.push(n);
                    }
                }
            }
        }
    }

    /////////// first one
    fn insert_recursive(node: &mut Box<Node>, value: i32) {
        if value > node.value {
            match &mut node.right {
                None => {
                    node.right = Node::new(value).into();
                }
                Some(n) => {
                    Tree::insert_recursive(n, value);
                }
            }
        } else if value < node.value {
            match &mut node.left {
                None => {
                    node.left = Node::new(value).into();
                }
                Some(n) => {
                    Tree::insert_recursive(n, value);
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn work_builds_tree() {
        let mut tree = Tree::new();
        tree.insert(8);
        tree.insert(10);
        tree.insert(3);
        tree.insert(1);
        tree.insert(6);
        tree.insert(4);
        tree.insert(7);
        tree.insert(14);
        tree.insert(13);
        tree.insert(v);
        assert_eq!(tree.root.is_some(), true);
        println!("{:?}", tree);
    }
}
