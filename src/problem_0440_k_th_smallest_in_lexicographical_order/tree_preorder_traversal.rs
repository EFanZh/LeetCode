pub struct Solution;

impl Solution {
    fn num_digits(mut x: u32) -> u32 {
        let mut result = 0;

        while x != 0 {
            x /= 10;
            result += 1;
        }

        result
    }

    fn find_kth_full(mut n: u32, mut k: u32, mut prefix: u32) -> u32 {
        while k != 0 {
            n /= 10;
            k -= 1;

            let branch = k / n;

            prefix = prefix * 10 + branch;
            k -= branch * n;
        }

        prefix
    }

    fn find_kth_partial(mut n: u32, mut k: u32, mut base: u32, mut subtree_size: u32, mut prefix: u32) -> u32 {
        while k != 0 {
            let last_column = n - subtree_size - 1;
            let middle_branch = last_column / base;
            let left_subtrees_size = subtree_size * middle_branch;

            k -= 1;

            if k < left_subtrees_size {
                let branch = k / subtree_size;

                return Self::find_kth_full(subtree_size, k - subtree_size * branch, prefix * 10 + branch);
            }

            k -= left_subtrees_size;
            subtree_size /= 10;

            let middle_subtree_size = subtree_size + last_column % base + 1;

            if k < middle_subtree_size {
                n = middle_subtree_size;
                base /= 10;
                prefix = prefix * 10 + middle_branch;
            } else {
                k -= middle_subtree_size;

                let branch_offset = k / subtree_size;

                return Self::find_kth_full(
                    subtree_size,
                    k - subtree_size * branch_offset,
                    prefix * 10 + middle_branch + 1 + branch_offset,
                );
            }
        }

        prefix
    }

    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        if k == 1 {
            1
        } else {
            let n = n as u32;
            let mut k = k as u32 - 1;
            let base = u32::pow(10, Self::num_digits(n) - 1);
            let mut subtree_size = (base * 10 - 1) / 9;
            let middle_branch = n / base;
            let left_subtrees_size = subtree_size * (middle_branch - 1);

            let result = if k < left_subtrees_size {
                let branch = k / subtree_size;

                Self::find_kth_full(subtree_size, k - subtree_size * branch, branch + 1)
            } else {
                k -= left_subtrees_size;
                subtree_size /= 10;

                let middle_subtree_size = subtree_size + (n - base) % base + 1;

                if k < middle_subtree_size {
                    Self::find_kth_partial(middle_subtree_size, k, base / 10, subtree_size, middle_branch)
                } else {
                    k -= middle_subtree_size;

                    let branch_offset = k / subtree_size;

                    Self::find_kth_full(
                        subtree_size,
                        k - subtree_size * branch_offset,
                        middle_branch + 1 + branch_offset,
                    )
                }
            };

            result as _
        }
    }
}

impl super::Solution for Solution {
    fn find_kth_number(n: i32, k: i32) -> i32 {
        Self::find_kth_number(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
