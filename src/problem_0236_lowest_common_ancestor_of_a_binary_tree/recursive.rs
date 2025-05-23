use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::ptr;
use std::rc::Rc;

impl Solution {
    fn has_node(
        root: Option<&RefCell<TreeNode>>,
        node: &RefCell<TreeNode>,
        yes: &dyn Fn() -> Rc<RefCell<TreeNode>>,
        no: &dyn Fn() -> Rc<RefCell<TreeNode>>,
    ) -> Rc<RefCell<TreeNode>> {
        root.map_or_else(no, |root| {
            if ptr::eq(root, node) {
                yes()
            } else {
                let root_ref = root.borrow();

                Self::has_node(root_ref.left.as_deref(), node, yes, &|| {
                    Self::has_node(root_ref.right.as_deref(), node, yes, no)
                })
            }
        })
    }

    #[expect(clippy::ref_option, reason = "by-design")]
    fn find_nodes(
        root: &Option<Rc<RefCell<TreeNode>>>,
        p: &RefCell<TreeNode>,
        q: &RefCell<TreeNode>,
        nothing: &dyn Fn() -> Rc<RefCell<TreeNode>>,
        only_p: &dyn Fn() -> Rc<RefCell<TreeNode>>,
        only_q: &dyn Fn() -> Rc<RefCell<TreeNode>>,
    ) -> Rc<RefCell<TreeNode>> {
        root.as_ref().map_or_else(nothing, |root| {
            let root_ref = root.borrow();

            if ptr::eq(root.as_ref(), p) {
                Self::has_node(root_ref.left.as_deref(), q, &|| Rc::clone(root), &|| {
                    Self::has_node(root_ref.right.as_deref(), q, &|| Rc::clone(root), only_p)
                })
            } else if ptr::eq(root.as_ref(), q) {
                Self::has_node(root_ref.left.as_deref(), p, &|| Rc::clone(root), &|| {
                    Self::has_node(root_ref.right.as_deref(), p, &|| Rc::clone(root), only_q)
                })
            } else {
                Self::find_nodes(
                    &root_ref.left,
                    p,
                    q,
                    &|| Self::find_nodes(&root_ref.right, p, q, nothing, only_p, only_q),
                    &|| Self::has_node(root_ref.right.as_deref(), q, &|| Rc::clone(root), only_p),
                    &|| Self::has_node(root_ref.right.as_deref(), p, &|| Rc::clone(root), only_q),
                )
            }
        })
    }

    fn helper(root: &Rc<RefCell<TreeNode>>, p: &RefCell<TreeNode>, q: &RefCell<TreeNode>) -> Rc<RefCell<TreeNode>> {
        if ptr::eq(root.as_ref(), p) || ptr::eq(root.as_ref(), q) {
            Rc::clone(root)
        } else {
            let root_ref = root.borrow();

            Self::find_nodes(
                &root_ref.left,
                p,
                q,
                &|| Self::helper(root_ref.right.as_ref().unwrap(), p, q),
                &|| Rc::clone(root),
                &|| Rc::clone(root),
            )
        }
    }

    #[expect(clippy::single_option_map, reason = "required")]
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        root.map(|root| Self::helper(&root, p.unwrap().as_ref(), q.unwrap().as_ref()))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::lowest_common_ancestor(root, p, q)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
