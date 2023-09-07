use std::fmt;
use std::fmt::Debug;

pub trait Container<T> {
    fn insert(&mut self, data: T);
    fn get(&self, index: u64) -> Option<T>;
    fn remove(&mut self, index: u64) -> bool;
    fn size(&self) -> usize;
    fn display(&self)
    where
        Self: Debug,
    {
        println!("{:#?}", self);
    }
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    size: usize,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data: data,
            size: 1 as usize,
            left: None,
            right: None,
        }
    }
}

impl<T: Debug + Clone> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref l) = self.left {
            _ = write!(f, "{}", l);
        }
        let left = if self.left.is_some() { "/" } else { "_" };
        let right = if self.right.is_some() { "\\" } else { "_" };
        let res = write!(
            f,
            "|{{{left}{right}}} size:{:>3} data:{:X?}|\r\n",
            self.size, self.data
        );

        if let Some(ref r) = self.right {
            _ = write!(f, "{}", r);
        }

        res
    }
}

#[derive(Debug)]
pub struct OrderStatisticTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T> OrderStatisticTree<T>
where
    T: PartialOrd + Debug + Clone,
{
    pub fn new() -> Self {
        OrderStatisticTree { root: None }
    }

    fn push_node(a_node_cur: &mut Box<Node<T>>, a_node_new: Box<Node<T>>) {
        let val_cur = &a_node_cur.data;
        let val_new = &a_node_new.data;

        a_node_cur.size += 1;

        if val_cur > val_new {
            if let Some(left) = a_node_cur.left.as_mut() {
                Self::push_node(left, a_node_new);
            } else {
                a_node_cur.left = Some(a_node_new);
            }
        } else {
            if let Some(right) = a_node_cur.right.as_mut() {
                Self::push_node(right, a_node_new);
            } else {
                a_node_cur.right = Some(a_node_new);
            }
        }
    }

    fn find(root: &Box<Node<T>>, index: usize) -> Option<T> {
        let p = match root.left.as_ref() {
            Some(n) => n.size + 1,
            _ => 1 as usize,
        };

        if p == index {
            return Some(root.data.clone());
        } else if root.left.is_some() && index < p {
            return Self::find(&root.left.as_ref().unwrap(), index);
        } else if root.right.is_some() && index > p {
            return Self::find(&root.right.as_ref().unwrap(), index - p);
        }

        None
    }

    fn remove(aroot: &mut Option<Box<Node<T>>>, index: usize) -> Option<T> {
        if let Some(root) = aroot {

            let p = match root.left.as_ref() {
                Some(n) => n.size + 1,
                _ => 1 as usize,
            };

            if root.left.is_some() && index < p {
                root.size -= 1;
                return Self::remove(&mut root.left, index);
            } else if root.right.is_some() && index > p {
                root.size -= 1;
                return Self::remove(&mut root.right, index - p);
            } else if p == index {
                if root.left.is_some() && root.right.is_some() {
                    // remove current and find first in the right
                    // store data from right
                    root.size -= 1;
                    let a = Self::remove(&mut root.right, 1);
                    root.data = a.clone().unwrap();
                    return a;
                } else {
                    let data = root.data.clone();

                    if let Some(l) = root.left.take() {
                        *aroot = Some(l);
                    } else if let Some(r) = root.right.take() {
                        *aroot = Some(r);
                    } else {
                        // drop root somehow
                        *aroot = None;
                    }
                    return Some(data);
                }
            }
        }

        None
    }
}

impl<T> Container<T> for OrderStatisticTree<T>
where
    T: PartialOrd + Debug + Clone,
{
    fn insert(&mut self, data: T) {
        let node = Box::new(Node::new(data));

        if let Some(root) = self.root.as_mut() {
            Self::push_node(root, node);
        } else {
            self.root = Some(node);
        }
    }

    fn get(&self, index: u64) -> Option<T> {
        return match self.root.as_ref() {
            Some(n) => Self::find(n, index as usize + 1),
            _ => None,
        };
    }

    fn remove(&mut self, index: u64) -> bool {
        if index as usize >= self.size() {
            return false;
        }
        Self::remove(&mut self.root, index as usize + 1).is_some()
    }

    fn size(&self) -> usize {
        if let Some(root) = self.root.as_ref() {
            root.size
        } else {
            0
        }
    }

    fn display(&self)
    where
        Self: Debug,
    {
        match self.root {
            Some(ref n) => println!("{}", n),
            _ => println!("Empty"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_size() {
        let c = OrderStatisticTree::<u8>::new();
        assert_eq!(c.size(), 0usize);
    }

    #[test]
    fn one_size_add_one() {
        let mut c = OrderStatisticTree::new();
        c.insert(0u8);
        assert_eq!(c.size(), 1usize);
    }

    #[test]
    fn zero_size_add_remove_one() {
        let mut c = OrderStatisticTree::new();
        c.insert(0u8);
        c.remove(0);
        assert_eq!(
            c.size(),
            0usize,
            "test inserting and removing from container"
        );
    }

    #[test]
    fn add_multiple_data_items() {
        let mut c = OrderStatisticTree::new();
        for _ in 0..10 {
            c.insert(0u8);
        }
        assert_eq!(
            c.size(),
            10usize,
            "test multiple inserting to the container"
        );
    }

    #[test]
    fn add_multiple_data_items_remove_all_from_back() {
        let mut c = OrderStatisticTree::new();
        for _ in 0..10 {
            c.insert(0u8);
        }
        for i in (0..10).into_iter().rev() {
            c.remove(i);
        }
        assert_eq!(
            c.size(),
            0usize,
            "test multiple removing from the container from back"
        );
    }

    #[test]
    fn add_multiple_data_items_remove_all_from_front() {
        let mut c = OrderStatisticTree::new();
        for _ in 0..10 {
            c.insert(0u8);
        }
        for _ in 0..10 {
            c.remove(0);
        }
        assert_eq!(
            c.size(),
            0usize,
            "test multiple removing from the container from front"
        );
    }

    #[test]
    fn add_multiple_data_items_in_direct_get_test_indices() {
        let mut c = OrderStatisticTree::new();
        for i in 0..10 {
            c.insert(i);
        }

        for i in 0..10 {
            assert_eq!(c.get(i), Some(i));
        }
    }

    #[test]
    fn add_multiple_data_items_in_reverse_get_test_indices() {
        let mut c = OrderStatisticTree::new();
        for i in (0..10).into_iter().rev() {
            c.insert(i);
        }

        for i in 0..10 {
            assert_eq!(c.get(i), Some(i));
        }
    }

    #[test]
    fn add_multiple_data_items_in_zig_zag_get_test_indices() {
        let mut c = OrderStatisticTree::new();
        for i in (0..5).chain((5..10).into_iter().rev()) {
            c.insert(i);
        }

        for i in 0..10 {
            assert_eq!(c.get(i), Some(i));
        }
    }

    #[test]
    fn add_multiple_data_items_in_btree_mode_get_test_indices() {
        let mut c = OrderStatisticTree::new();
        for i in [5, 3, 7, 1, 4, 6, 8, 0, 2, 9] {
            c.insert(i);
        }

        for i in 0..10 {
            assert_eq!(c.get(i), Some(i));
        }
    }

    #[test]
    fn add_multiple_chars_in_btree_mode_get_test_indices() {
        let mut c = OrderStatisticTree::new();
        for i in ['e', 'c', 'i', 'b', 'd', 'f', 'j', 'a', 'g'] {
            c.insert(i);
        }

        for (ch, i) in ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'i', 'j']
            .into_iter()
            .zip((0..10).into_iter())
        {
            assert_eq!(c.get(i), Some(ch));
        }
    }

    #[test]
    fn add_multiple_string_in_btree_mode_get_test_indices() {
        let mut c = OrderStatisticTree::new();
        for i in ["e", "c", "i", "b", "d", "f", "j", "a", "g"] {
            c.insert(i);
        }

        for (ch, i) in ["a", "b", "c", "d", "e", "f", "g", "i", "j"]
            .into_iter()
            .zip((0..10).into_iter())
        {
            assert_eq!(c.get(i), Some(ch));
        }
    }

    #[test]
    fn add_multiple_cyrillic_string_in_btree_mode_get_test_indices() {
        let mut c = OrderStatisticTree::new();
        for i in ["д", "в", "и", "б", "г", "е", "к", "а", "ж"] {
            c.insert(i);
        }

        for (ch, i) in ["а", "б", "в", "г", "д", "е", "ж", "и", "к"]
            .into_iter()
            .zip((0..10).into_iter())
        {
            assert_eq!(c.get(i), Some(ch));
        }
    }
}
