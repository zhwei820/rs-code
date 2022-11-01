// https://leetcode.cn/problems/count-numbers-with-unique-digit/
struct Solution;

impl Solution {
    fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n < 11 {
            Self::f(n)
        } else {
            Self::f(10)
        }
    }

    fn f(n: i32) -> i32 {
        match n {
            1 => 10,
            2 => 9 * 9 + 10,
            3 => 9 * 9 * 8 + 9 * 9 + 10,
            4 => 9 * 9 * 8 * 7 + 9 * 9 * 8 + 9 * 9 + 10,
            5 => 9 * 9 * 8 * 7 * 6 + 9 * 9 * 8 * 7 + 9 * 9 * 8 + 9 * 9 + 10,
            6 => 9 * 9 * 8 * 7 * 6 * 5 + 9 * 9 * 8 * 7 * 6 + 9 * 9 * 8 * 7 + 9 * 9 * 8 + 9 * 9 + 10,
            7 => {
                9 * 9 * 8 * 7 * 6 * 5 * 4
                    + 9 * 9 * 8 * 7 * 6 * 5
                    + 9 * 9 * 8 * 7 * 6
                    + 9 * 9 * 8 * 7
                    + 9 * 9 * 8
                    + 9 * 9
                    + 10
            }
            8 => {
                9 * 9 * 8 * 7 * 6 * 5 * 4 * 3
                    + 9 * 9 * 8 * 7 * 6 * 5 * 4
                    + 9 * 9 * 8 * 7 * 6 * 5
                    + 9 * 9 * 8 * 7 * 6
                    + 9 * 9 * 8 * 7
                    + 9 * 9 * 8
                    + 9 * 9
                    + 10
            }
            9 => {
                9 * 9 * 8 * 7 * 6 * 5 * 4 * 3 * 2
                    + 9 * 9 * 8 * 7 * 6 * 5 * 4 * 3
                    + 9 * 9 * 8 * 7 * 6 * 5 * 4
                    + 9 * 9 * 8 * 7 * 6 * 5
                    + 9 * 9 * 8 * 7 * 6
                    + 9 * 9 * 8 * 7
                    + 9 * 9 * 8
                    + 9 * 9
                    + 10
            }
            10 => {
                9 * 9 * 8 * 7 * 6 * 5 * 4 * 3 * 2
                    + 9 * 9 * 8 * 7 * 6 * 5 * 4 * 3 * 2
                    + 9 * 9 * 8 * 7 * 6 * 5 * 4 * 3
                    + 9 * 9 * 8 * 7 * 6 * 5 * 4
                    + 9 * 9 * 8 * 7 * 6 * 5
                    + 9 * 9 * 8 * 7 * 6
                    + 9 * 9 * 8 * 7
                    + 9 * 9 * 8
                    + 9 * 9
                    + 10
            }
            0 => 1,
            _ => 0,
        }
    }
}

#[test]
fn test() {
    let n = 2;
    let res = 91;
    assert_eq!(Solution::count_numbers_with_unique_digits(n), res);
}
