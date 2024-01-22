pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

struct Context {
    s: Vec<u8>,
    counts: Box<[[(u8, u8); 26]]>,
    result: (u8, u8),
}

impl Solution {
    fn helper(context: &mut Context, start: u8, end: u8) {
        let mut count = context.counts[usize::from(end) - 1];

        if let Some(left_count) = context.counts.get(usize::from(start).wrapping_sub(1)) {
            for (target, left_value) in count.iter_mut().zip(left_count) {
                target.0 -= left_value.0;
                target.1 -= left_value.1;
            }
        }

        if count
            .iter()
            .any(|&(upper_count, lower_count)| (upper_count == 0) != (lower_count == 0))
        {
            let mut prev = start;

            for i in start..end {
                let c = context.s[usize::from(i)];

                if if c.is_ascii_uppercase() {
                    count[usize::from(c) - usize::from(b'A')].1
                } else {
                    count[usize::from(c) - usize::from(b'a')].0
                } == 0
                {
                    if i - prev > context.result.1 - context.result.0 {
                        Self::helper(context, prev, i);
                    }

                    prev = i + 1;
                }
            }

            if end - prev > context.result.1 - context.result.0 {
                Self::helper(context, prev, end);
            }
        } else {
            context.result = (start, end);
        }
    }

    pub fn longest_nice_substring(s: String) -> String {
        let n = s.len();

        let mut context = Context {
            s: s.into_bytes(),
            counts: vec![[(0_u8, 0_u8); 26]; n].into_boxed_slice(),
            result: (0, 0),
        };

        let mut count = [(0_u8, 0_u8); 26];

        for (target, &c) in context.counts.iter_mut().zip(&context.s) {
            if c.is_ascii_uppercase() {
                count[usize::from(c) - usize::from(b'A')].0 += 1;
            } else {
                count[usize::from(c) - usize::from(b'a')].1 += 1;
            };

            *target = count;
        }

        Self::helper(&mut context, 0, n as _);

        context
            .s
            .copy_within(usize::from(context.result.0)..usize::from(context.result.1), 0);

        context.s.truncate(usize::from(context.result.1 - context.result.0));

        String::from_utf8(context.s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_nice_substring(s: String) -> String {
        Self::longest_nice_substring(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
