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
        match &mut self.root {
            None => {
                self.root = Node::new(value).into();
            }
            Some(node) => {
                Tree::insert_recursive(node, value);
            }
        }
    }

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
mod test1 {
    use super::*;

    #[test]
    fn it_works_as_tree() {
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

        assert_eq!(tree.root.is_some(), true);
        println!("{:?}", tree);
    }
}



// fn inorder(&self) -> Vec!(i32) {
//     if self.root.is_none() {
//         return Vec::new();
//     }
//     let mut result: Vec<i32> = Vec::new();
//     let root = seflf.root.as_ref().unwrap(); Tree::traverse_inorder_recursive(values, root);


// }




// #test[cfg(derive)]
// fn works(){
//     let mut tree = Tree::new();
//     tree.insert(8)
//     tree.insert(10)
//     tree.insert(3)
//     tree.insert(1)
//     tree.insert(6)
//     tree.insert(4)
//     tree.insert(7)
//     tree.insert(14)
//     tree.insert(13)
//     assert_eq(tree.inorder(),vec![1,3,4,6,7,8,10,13,14]);
// }
