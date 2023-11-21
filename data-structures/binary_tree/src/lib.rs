#[derive(Debug, PartialEq, Clone)]
pub struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

use std::cmp::{self, PartialOrd};
use std::collections::VecDeque;

impl<T: PartialOrd + Copy> Node<T> {
    pub fn new(data: T) -> Self {
        Node {
            data,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, data: T) {
        
        if self.find(data) != None {
            return;
        }
        let mut list = VecDeque::new();
        list.push_front(self);
        loop {
            let Node {
                ref mut left,
                ref mut right,
                ..
            } = list.pop_back().unwrap();
            
            match left {
                Some(node) => {
                    list.push_front(node);
                }
                None => {
                    *left = Some(Box::new(Node::new(data)));
                    return;
                }
            }

            match right {
                Some(node) => {
                    list.push_front(node);
                }
                None => {
                    *right = Some(Box::new(Node::new(data)));
                    return;
                }
            }
        }
    }
    pub fn find(&self, elem: T) -> Option<T> {

        use cmp::Ordering as O;
        return match elem.partial_cmp(&self.data) {
            Some(O::Equal) => Some(elem),
            Some(O::Less) => self.left.as_ref()?.find(elem),
            Some(O::Greater) => self.right.as_ref()?.find(elem),
            _ => None,
        };
    }

    fn _aux<F>(x: &Option<Box<Node<T>>>, res: &mut Vec<T>, mut f: F)
        where F: FnMut(&Self, &mut Vec<T>) {
            
            if let Some(ref branch) = x {
                f(branch, res);
            }
    } 
    pub fn inorder(&self, res: &mut Vec<T>) {
        use Node as N;
        N::_aux(&self.left, res, N::inorder);
        res.push(self.data);
        N::_aux(&self.right, res, N::inorder);
    }

    pub fn preorder(&self, res: &mut Vec<T>) {
        use Node as N;
        res.push(self.data);
        N::_aux(&self.left, res, N::preorder);
        N::_aux(&self.right, res, N::preorder);
    }

    pub fn postorder(&self, res: &mut Vec<T>) {
        use Node as N;
        N::_aux(&self.left, res, N::postorder);
        N::_aux(&self.right, res, N::postorder);
        res.push(self.data);
    }

    pub fn levelorder(&self, res: &mut Vec<T>) {
        
        let mut list = VecDeque::new();
        list.push_back(self);

        while !list.is_empty() {
            let node = *list.front().unwrap();
            res.push(node.data);
            list.pop_front();

            if node.left != None {
                list.push_back(node.left.as_ref().unwrap());
            }
            if node.right != None {
                list.push_back(node.right.as_ref().unwrap())
            }
        }
    }
    pub fn depth(&self) -> i32 {
        match self {
            Node {
                left: None,
                right: None, ..
            } => 1,
            Node {
                left: Some(left),
                right: None, ..
            } |
            Node {
                left: None,
                right: Some(left), ..
            } => 1 + left.depth(),
            Node {
                left: Some(l),
                right: Some(r), ..
            } => 1 + l.depth().max(r.depth()),
        }
    }
    pub fn delete(&mut self, data: T) -> bool {
        
        if self.data == data {
            return true;
        }

        if let Some(ref mut split) = self.left {
            if Self::delete(split, data) {
                self.left = None;
            }
        }

        if let Some(ref mut split) = self.right {
            if Self::delete(split, data) {
                self.right = None;
            }
        }
        false
    }
}
#[macro_export]
macro_rules! node {
    ( $( $x:expr ),* ) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )*
            let (first, last) = v.split_first().unwrap();
            let mut node = Node::new(*first);

            for elem in last {
                node.insert(*elem);
            }
            node
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Node;

    #[test]
    fn basics() {
        let mut tree = node!(1, 2, 3, 4, 5);

        println!("here: {:?}", tree);
        assert_eq!(Some(3), tree.find(3));
        assert_eq!(None, tree.find(42));

        let ch_tree = node!('a', 'g', 'b', 'e');

        assert_eq!(Some('b'), ch_tree.find('b'));

        tree.delete(3);
        assert_eq!(None, tree.find(3));
        // other possible types.
        let _tree = node!(1.1, 2.2);
        let _tree = node!(true, false);
        let _tree = node!("a", "b");
    }
    #[test]
    fn transversal() {
        let tree = node!(1, 2, 3, 4, 5, 6, 7);
        let mut result = Vec::new();

        tree.inorder(&mut result);
        assert_eq!(vec![4, 2, 5, 1, 6, 3, 7], result);

        result.clear();
        tree.preorder(&mut result);
        assert_eq!(vec![1, 2, 4, 5, 3, 6, 7], result);

        result.clear();
        tree.postorder(&mut result);
        assert_eq!(vec![4, 5, 2, 6, 7, 3, 1], result);

        result.clear();
        tree.levelorder(&mut result);
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7], result);

        assert_eq!(3, tree.depth());
    }
    #[test]
    fn macro_test() {
        let mut x = Node::new(1);

        x.insert(2);
        x.insert(3);

        let macro_x = node!(1, 2, 3);
        assert_eq!(x, macro_x);

        assert_eq!(2, x.depth());
    }
}
