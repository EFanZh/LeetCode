use super::super::data_structures::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Codec;

impl Codec {
    fn new() -> Self {
        Self
    }

    #[allow(clippy::unused_self)]
    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        use std::fmt::Write;

        let mut result = String::from("[");

        if let Some(mut node) = root {
            let mut queue = VecDeque::new();
            let mut non_null_nodes = 0;

            'outer: loop {
                let node_ref = node.borrow();

                write!(&mut result, "{}", node_ref.val).unwrap();

                queue.push_back(node_ref.left.clone());
                queue.push_back(node_ref.right.clone());
                non_null_nodes += usize::from(node_ref.left.is_some()) + usize::from(node_ref.right.is_some());

                drop(node_ref);

                while non_null_nodes != 0 {
                    loop {
                        if let Some(next) = queue.pop_front().unwrap() {
                            node = next;
                            non_null_nodes -= 1;

                            result.push(',');

                            continue 'outer;
                        } else {
                            result.push_str(",null");
                        }
                    }
                }

                break;
            }
        }

        result.push(']');

        result
    }

    #[allow(clippy::unused_self)]
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut iter = data[1..data.len() - 1].split_terminator(',');

        iter.next().map(|val| {
            let root = Rc::new(RefCell::new(TreeNode::new(val.parse().unwrap())));
            let mut queue = VecDeque::from(vec![root.clone()]);

            while let Some(item) = iter.next() {
                let target = queue.pop_front().unwrap();
                let mut target_ref = target.borrow_mut();

                if let Ok(val) = item.parse() {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));

                    target_ref.left = Some(left.clone());

                    queue.push_back(left);
                }

                if let Some(item) = iter.next() {
                    if let Ok(val) = item.parse() {
                        let right = Rc::new(RefCell::new(TreeNode::new(val)));

                        target_ref.right = Some(right.clone());

                        queue.push_back(right);
                    }
                } else {
                    break;
                }
            }

            root
        })
    }
}

impl super::Codec for Codec {
    fn new() -> Self {
        Self::new()
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        self.serialize(root)
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        self.deserialize(data)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Codec>();
    }
}
