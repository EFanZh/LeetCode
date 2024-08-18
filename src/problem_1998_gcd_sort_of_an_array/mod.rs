pub mod union_find;

pub trait Solution {
    fn gcd_sort(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    const EXTRA_TEST_CASE: (&[i32], bool) = (
        &[
            431, 4310, 3448, 1293, 431, 3017, 862, 1724, 2586, 4741, 2155, 3879, 4310, 1724, 1724, 3448, 1724, 4741,
            4310, 1293, 431, 3879, 3879, 1293, 1293, 862, 862, 1724, 2586, 3448, 862, 2155, 4310, 4310, 4741, 4741,
            431, 3017, 862, 3879, 4310, 2586, 431, 2586, 431, 2586, 431, 2155, 3017, 3448, 862, 3448, 3879, 2586, 3017,
            2586, 431, 1293, 431, 862, 2155, 2155, 3879, 2586, 3879, 3017, 4310, 1724, 862, 3017, 2155, 4310, 4310,
            3017, 3448, 862, 2155, 4741, 4741, 4310, 431, 862, 862, 3879, 4741, 3448, 3448, 862, 4741, 4310, 1293,
            2586, 3448, 3017, 2155, 3448, 1724, 3448, 3017, 2586, 2586, 4741, 4741, 431, 862, 3448, 2155, 4310, 3448,
            1724, 4741, 1724, 2155, 2586, 3879, 862, 4741, 2155, 1724, 431, 1724, 4741, 1724, 2155, 3879, 862, 4741,
            3448, 3879, 862, 431, 4741, 1293, 4310, 4741, 862, 4741, 4310, 3017, 4310, 1724, 4310, 3879, 3879, 431,
            2586, 1724, 862, 4310, 2155, 3017, 431, 1293, 4741, 2155, 3017, 3448, 4310, 862, 1293, 2155, 4310, 4310,
            1293, 1293, 431, 1724, 3017, 4310, 2586, 2586, 4741, 4310, 1724, 2155, 1293, 4741, 4310, 862, 431, 1293,
            862, 2155, 3017, 862, 3017, 2586, 2155, 2155, 2586, 2586, 2586, 862, 1293, 4310, 3017, 2586, 3879, 2586,
            3448, 3017, 431, 3879, 1293, 4310, 3879, 4310, 431, 3879, 1724, 4741, 431, 431, 431, 4310, 3017, 4741,
            2155, 1724, 3879, 2155, 2586, 431, 3879, 4741, 3879, 4310, 3017, 4310, 4741, 1293, 2586, 2586, 431, 862,
            4741, 4310, 1293, 1724, 3448, 3017, 1293, 3017, 4741, 3017, 431, 4310, 3448, 2155, 3017, 3879, 1724, 2586,
            2586, 1724, 4741, 4741, 2586, 431, 2586, 4741, 1293, 2586, 3879, 1724, 3017, 3017, 2155, 3017, 4741, 1724,
            862, 3448, 1724, 3017, 1293, 2586, 2155, 3448, 862, 862, 2155, 1724, 431, 3879, 3017, 431, 2586, 1293, 431,
            1293, 3448, 3017, 4310, 1724, 3017, 431, 1293, 2586, 3879, 431, 2586, 3017, 862, 2155, 862, 1724, 2155,
            4310, 3879, 2586, 862, 2586, 3879, 2586, 4310, 2586, 4310, 2586, 4310, 2586, 4310, 2586, 3017, 3879, 431,
            2586, 1724, 1724, 4310, 2586, 3017, 3017, 3017, 1724, 2155, 1724, 4310, 431, 3448, 3017, 431, 1724, 2155,
            3017, 4310, 3448, 2155, 3879, 2155, 431, 1293, 2155, 3448, 1724, 2155, 4310, 1293, 3448, 4741, 862, 862,
            3017, 4310, 862, 4310, 862, 3017, 1293, 1724, 862, 2586, 3448, 3017, 1724, 1724, 4741, 3017, 2155, 1724,
            3448, 1293, 3879, 4741, 4741, 4310, 1724, 3017, 3448, 2155, 3017, 2155, 2586, 1724, 2155, 431, 1293, 1293,
            1724, 862, 3448, 3448, 2586, 431, 3879, 431, 3017, 3448, 3448, 4741, 1293, 4310, 3879, 3448, 4310, 2155,
            3017, 3448, 4741, 1724, 862, 1724, 1724, 3879, 431, 3017, 4310, 1293, 1724, 1293, 4741, 862, 1293, 4741,
            3879, 2586, 4741, 862, 1293, 4741, 862, 2586, 431, 3879, 4310, 2586, 1293, 3879, 862, 4310, 1293, 1293,
            3879, 862, 4310, 4310, 3879, 3448, 1724, 3448, 1293, 4310, 4741, 1724, 3017, 862, 3448, 1724, 2586, 431,
            862, 3017, 3017, 1724, 3879, 1724, 3448, 4310, 431, 431, 862, 862, 3017, 1293, 4310, 1293, 431, 3879, 1293,
            1724, 3017, 3017, 4310, 431, 862, 862, 4310, 431, 4310, 1724, 1293, 862, 431, 4741, 3879, 3879, 3879, 1724,
            3448, 2155, 862, 2155, 2586, 1724, 1293, 4310, 4741, 4741, 3448, 3879, 2586, 862, 1724, 1724, 1293, 2155,
            1293, 431, 2586, 3017, 862, 3448, 3448, 862, 1293, 862, 1724, 4310, 2586, 431, 862, 2586, 2586, 3448, 3448,
            1293, 3879, 3017, 862, 3017, 3448, 1724, 1293, 862, 1293, 2586, 3879, 3017, 2155, 1293, 3448, 1724, 3017,
            4741, 3017, 3448, 3448, 1724, 2586, 3017, 4310, 3879, 862, 4310, 3879, 3879, 862, 4741, 431, 431, 862,
            4310, 4741, 3448, 4741, 2155, 4741, 4741, 2155, 4741, 2586, 3017, 862, 4741, 4741, 1293, 4741, 862, 1293,
            862, 4310, 3448, 2155, 1724, 3448, 2155, 4741, 1724, 4741, 2155, 2586, 431, 862, 2586, 4310, 862, 862,
            3017, 3448, 1724, 3879, 3448, 1724, 862, 3448, 2586, 4741, 1724, 3448, 4741, 431, 4741, 2586, 3879, 3448,
            3879, 3879, 862, 3448, 431, 2586, 1293, 862, 2155, 2586, 3448, 3879, 4310, 4310, 862, 3017, 4741, 3448,
            3017, 4310, 862, 1293, 3017, 4741, 3879, 862, 1293, 2155, 2586, 3448, 4741, 4310, 1293, 1724, 862, 3448,
            2155, 3017, 862, 1293, 2586, 3017, 862, 1724, 2155, 3748, 937, 3748, 2811, 3748, 3748, 1874, 4685, 1874,
            2811, 1874, 3748, 937, 4685, 1874, 937, 1874, 937, 4685, 3748, 937, 1874, 2811, 1874, 937, 937, 2811, 2811,
            3748, 3748, 4685, 3748, 3748, 4685, 2811, 3748, 3748, 1874, 2811, 3748, 3748, 3748, 937, 2811, 4685, 1874,
            3748, 3748, 4685, 937, 937, 937, 4685, 4685, 3748, 2811, 3748, 937, 937, 3748, 937, 3748, 4685, 1874, 937,
            1874, 3748, 937, 3748, 4685, 3748, 1874, 3748, 3748, 1874, 2811, 3748, 4685, 937, 937, 4685, 937, 1874,
            1874, 937, 1874, 3748, 1874, 3748, 1874, 1874, 2811, 937, 3748, 937, 937, 937, 3748, 3748, 3748, 3748,
            2811, 937, 2811, 1874, 3748, 4685, 1874, 3748, 1874, 4685, 937, 3748, 3748, 1874, 3748, 4685, 4685, 2811,
            2811, 1874, 1874, 937, 937, 2811, 3748, 937, 2811, 2811, 937, 937, 4685, 2811, 1874, 1874, 937, 1874, 1874,
            1874, 4685, 937, 1874, 937, 2811, 4685, 937, 4685, 1874, 937, 3748, 1874, 937, 3748, 937, 3748, 937, 4685,
            3748, 937, 2811, 2811, 4685, 1874, 3748, 3748, 4685, 2811, 3748, 1874, 1874, 2811, 937, 2811, 4685, 937,
            937, 2811, 3748, 3748, 4685, 1874, 4685, 2811, 3748, 3748, 3748, 2811, 937, 937, 4685, 3748, 4685, 1874,
            2811, 937, 4685, 3748, 4685, 937, 937, 3748, 3748, 4685, 937, 3748, 3748, 3748, 2811, 937, 1874, 4685,
            1874, 2811, 3748, 4685, 1874, 2811, 937, 937, 2811, 4685, 937, 1874, 937, 4685, 1874, 4685, 1874, 937,
            3748, 3748, 937, 1874, 937, 1874, 937, 937, 2811, 1874, 2811, 1874, 1874, 2811, 1874, 4685, 4685, 937,
            3748, 2811, 2811, 1874, 937, 1874, 3748, 1874, 2811, 2811, 1874, 2811, 2811, 1874, 1874, 2811, 4685, 937,
            1874, 2811, 3748, 937, 4685, 2811, 3748, 937, 4685, 3748, 4685, 2811, 1874, 2811, 4685, 2811, 2811, 3748,
            3748, 2811, 1874, 1874, 1874, 1874, 3748, 3748, 4685, 3748, 2811, 937, 2811, 1874, 2811, 937, 937, 3748,
            937, 4685, 937, 937, 3748, 3748, 1874, 937, 3748, 937, 2811, 1874, 1874, 2811, 1874, 3748, 3748, 937, 2811,
            937, 2811, 937, 2811, 4685, 937, 3748, 937, 1874, 1874, 937, 1874, 937, 2811, 1874, 2811, 3748, 2811, 937,
            2811, 4685, 1874, 1874, 2811, 3748, 4685, 3748, 4685, 937, 2811, 1874, 1874, 3748, 2811, 3748, 4685, 1874,
            3748, 2811, 2811, 4685, 4685, 3748, 3748, 937, 1874, 937, 937, 2811, 4685, 3748, 2811, 3748, 3748, 937,
            3748, 937, 1874, 2811, 1874, 1874, 2811, 937, 1874, 2811, 1874, 4685, 2811, 1874, 1874, 3748, 1874, 2811,
            1874, 4685, 2811, 1874, 937, 937, 3748, 3748, 937, 2811, 1874, 3748, 2811, 2811, 937, 1874, 4685, 3748,
            1874, 1874, 3748, 3748, 3748, 4685, 2811, 3748, 3748, 2811, 3748, 1874, 3748, 4685, 4685, 2811, 937, 937,
            2811, 1874, 4685, 1874, 2811, 3748, 2811, 2811, 1874, 937, 937, 937, 1874, 937, 2811, 2811, 2811, 2811,
            4685, 937, 937, 1874, 3748, 937, 1874, 2811, 3748, 937, 937, 4685, 1874, 4685, 2811, 4685, 937, 937, 937,
            3748, 1874, 3748, 3748, 2811, 4685, 1874, 2811, 4685, 3748, 1874, 1874, 4685, 4685, 937, 3748, 937, 1874,
            1874, 3748, 3748, 937, 2811, 4685, 4685, 3748, 2811, 937, 1874, 4685, 3748, 1874, 937, 2811, 2811, 937,
            1874, 3748, 3748, 1874, 3748, 2811, 3748, 3748, 937, 2811, 2811, 4685, 3748, 4685, 937, 1874, 2811, 4685,
            937, 4685, 3748, 2811, 4685, 937, 2811, 937, 4685, 3748, 3748, 1874, 1874, 937, 2811, 2811, 3748, 4685,
            1874, 4685, 3748, 3748, 1874, 3748, 1874, 2811, 2811, 2811, 4685, 4685, 3748, 4685, 2811, 2811, 3748, 2811,
            1874, 4685, 1874, 3748, 2811, 2811, 937, 937, 937, 937, 3748, 1874, 2811, 2811, 2811, 1874, 3748, 1874,
            1874, 3748, 937, 2811, 1874, 3748, 3748, 2811, 3748, 2811, 937, 1874, 1874, 4685, 4685, 2811, 1874, 3748,
            4685, 4685, 937, 2811, 3748, 937, 4685, 937, 3748, 2811, 4685, 3748, 1874, 937, 4685, 2811, 1874, 937,
            3748, 1874, 2811, 937, 3748, 2811, 2811, 937, 4685, 1874, 4685, 937, 937, 3748, 3748, 3748, 2811, 4685,
            2811, 2811, 1874, 1874, 1874, 1874, 2811, 1874, 3748, 937, 937, 1874, 937, 937, 937, 3748, 937, 3748, 4685,
            3748, 1874, 3748, 937, 3748, 4685, 2811, 4685, 2811, 937, 2811, 2811, 3748, 937, 2811, 937, 4685, 3748,
            3748, 3748, 3748, 937, 4685, 1874, 3748, 937, 937, 1874, 2811, 1874, 4685, 3748, 1874, 4685, 3748, 1874,
            937, 3748, 3748, 2811, 1874, 937, 937, 4685, 1874, 2811, 937, 937, 3748, 2811, 937, 4685, 2811, 3748, 937,
            937, 3748, 3748, 4685, 2811, 3748, 2811, 2811, 3748, 3748, 937, 937, 3748, 3748, 4685, 4685, 4685, 937,
            1874, 4685, 1874, 3748, 3748, 3748, 937, 4685, 4685, 4685, 937, 937, 2811, 2811, 937, 4685, 4685, 2811,
            3748, 937, 937, 937, 4685, 937, 3748, 937, 3748, 3748, 2811, 1874, 1874, 4685, 2811, 4685, 2811, 3748,
            4685, 1874, 4685, 937, 2811, 1874, 4685, 2811, 937, 2811, 2811, 3748, 3748, 1874, 3748, 1874, 4685, 4685,
            937, 4685, 2811, 1874, 1874, 4685, 1874, 937, 3748, 2811, 937, 3748, 3748, 2811, 1874, 1874, 2811, 4685,
            937, 1874, 4685, 4685, 1874, 937, 4685, 4685, 2811, 1874, 1874, 3748, 937, 3748, 937, 3748, 4685, 4685,
            3748, 3748, 937, 937, 4685, 937, 937, 4685, 1874, 3748, 2811, 4685, 4685, 4685, 2811, 4685, 1874, 2811,
            1874, 3748, 3748, 4685, 2811, 2811, 1874, 937, 937, 2811, 937, 4685, 1874, 4685, 937, 2811, 937, 4685,
            1874, 937, 937, 3748, 4685, 937, 1874, 937, 1874, 1874, 2811, 1874, 937, 4685, 2811, 3748, 1874, 1874, 937,
            937, 937, 1874, 937, 4685, 3748, 4685, 937, 3748, 4685, 2811, 4685, 937, 4685, 937, 4685, 2811, 2811, 4685,
            1874, 1874, 937, 2811, 4685, 3748, 937, 1874, 3748, 937, 2811, 3748, 937, 4685, 937, 2811, 937, 4685, 1874,
            1874, 4685, 937, 3748, 937, 1874, 1874, 937, 4685, 1874, 937, 1874, 2811, 4685, 937, 1874, 2811, 2811, 937,
            3748, 2811, 4685, 937, 2811, 937, 1874, 4685, 2811, 2811, 4685, 937, 3748, 2811, 3748, 4685, 1874, 1874,
            937, 2811, 937, 2811, 937, 2811, 4685, 3748, 3748, 3748, 1874, 2811, 3748, 4685, 4685, 4685, 937, 1874,
            4685, 2811, 4685, 4685, 2811, 1874, 3748, 1874, 2811, 3786, 1893, 1893, 3155, 1893, 4417, 4417, 1893, 1893,
            1893, 4417, 4417, 4417, 4417, 4417, 4417, 631, 631, 1893, 2524, 1262, 4417, 1893, 1262, 4417, 3155, 2524,
            2524, 631, 1262, 2524, 631, 3155, 2524, 1262, 1262, 2524, 1262, 1893, 1893, 1262, 4417, 4417, 1251, 4865,
            1807, 3614, 695, 4170, 1251, 3058, 2224, 2502, 3475, 556, 695, 834, 417, 1807, 1807, 2085, 556, 973, 1112,
            695, 2641, 4726, 1251, 278, 556, 3336, 4587, 3197, 1946, 695, 3197, 1390, 2919, 4170, 3058, 834, 139, 3058,
            2641, 278, 1112, 417, 417, 1390, 1251, 1112, 556, 1807, 3475, 2641, 1946, 3058, 1946, 2363, 3336, 2641,
            1529, 2641, 973, 3753, 417, 417, 1112, 417, 1112, 1112, 4865, 2641, 834, 4170, 2502, 278, 3336, 2363, 1529,
            973, 4448, 2085, 4448, 3197, 4031, 695, 3197, 1668, 1668, 2780, 4587, 417, 3058, 3058, 695, 4726, 1529,
            3892, 4031, 4309, 3197, 1390, 4865, 139, 695, 3336, 3336, 3197, 4170, 2363, 1668, 139, 2780, 1668, 1112,
            1946, 1946, 1112, 1529, 4309, 3197, 4448, 4865, 4170, 3336, 417, 3892, 556, 1529, 2363, 556, 2363, 2780,
            2919, 2919, 278, 2919, 2780, 139, 3336, 3614, 4448, 3892, 417, 3892, 278, 3336, 139, 3058, 1529, 2363,
            1946, 4031, 1112, 3614, 2363, 3475, 973, 2641, 1946, 3614, 834, 278, 2224, 4309, 3614, 556, 1668, 556,
            3336, 1529, 834, 3336, 556, 4031, 2085, 2641, 2641, 1390, 2919, 2502, 4587, 4865, 4170, 2780, 3475, 973,
            1529, 4865, 3892, 1946, 2224, 834, 4170, 3614, 4587, 3336, 3753, 2502, 2224, 1668, 4309, 3058, 1946, 1112,
            4865, 4309, 2085, 2780, 1390, 2502, 834, 139, 3058, 4309, 4031, 3058, 4448, 2363, 3058, 3475, 973, 973,
            2919, 3753, 417, 4865, 3336, 1668, 278, 1668, 3892, 139, 556, 1251, 1668, 139, 1529, 2502, 4170, 2363,
            1529, 3753, 1946, 3614, 3753, 1946, 4587, 1807, 4865, 278, 4726, 834, 3892, 2224, 4031, 1668, 3753, 2919,
            3475, 2641, 1946, 834, 834, 278, 2919, 2363, 1251, 4448, 4031, 1390, 1529, 3753, 4865, 3475, 973, 139,
            4865, 1112, 1946, 1390, 4170, 2502, 417, 3336, 3058, 3058, 278, 2919, 1251, 1668, 3892, 4865, 973, 246,
            1230, 2173, 3157, 656, 4756, 3608, 1312, 902, 410, 2829, 1435, 1189, 4305, 2162, 47, 1948, 1540,
        ],
        true,
    );

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[7, 21, 3] as &[_], true),
            (&[5, 2, 6, 2], false),
            (&[10, 5, 9, 3, 15], true),
            (&[10, 7, 3, 5, 2], false),
            EXTRA_TEST_CASE,
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::gcd_sort(nums.to_vec()), expected);
        }
    }
}
