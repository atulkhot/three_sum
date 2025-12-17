struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut sorted = nums;
        sorted.sort();
        let mut result: Vec<Vec<i32>> = vec![];
        let num_elems = sorted.len();
        let mut l = 0;
        let mut m = l + 1;
        let mut r = m;

        loop {
            if l + 2 >= num_elems {
                break;
            }
            if sorted[l] > 0 {
                // we cannot have any more elements summing to 0
                break;
            }
            if m == l + 1 && m >= num_elems {
                // we have only two elements left to check
                break;
            }
            r += 1;

            assert!(l != m && m != r);

            if r >= num_elems {
                if m >= num_elems {
                    l += 1;
                    m = l + 1;
                    r = m;
                } else {
                    m += 1;
                    r = m;
                }
                continue;
            }
            let sum = sorted[l] + sorted[m] + sorted[r];
            if sum > 0 {
                l += 1;
                m = l + 1;
                r = m;
                continue;
            } else if sum < 0 {
                continue;
            } else {
                /* here sum == 0 */
                result.push(vec![sorted[l], sorted[m], sorted[r]]);
                let elem = sorted[r];
                while r + 1 < num_elems && sorted[r + 1] == elem {
                    /* skip identical elements */
                    r += 1;
                }
                if r == num_elems - 1 && sorted[r] == elem {
                    break;
                }
                continue;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_first() {
        let result = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
        let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_no_sum() {
        let result = Solution::three_sum(vec![0, 1, 1]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_input_all_zeros() {
        let result = Solution::three_sum(vec![0, 0, 0, 0, 0, 0, 0]);
        let expected = vec![vec![0, 0, 0]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_one_sum_all_zeroes_at_the_end() {
        let result = Solution::three_sum(vec![-10, -8, 5, -3, 0, 0, 0]);
        let expected = vec![vec![0, 0, 0]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_one_sum_at_the_start() {
        let result = Solution::three_sum(vec![-10, 10, 0, 9, 10, 110, 1110]);
        let expected = vec![vec![-10, 0, 10]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_one_sum_disconnected() {
        let result = Solution::three_sum(vec![-10, -8, 0, 1, 9, 10, 110, 1110]);
        let expected = vec![vec![-10, 1, 9]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_one_sum_disconnected_small_vector() {
        let result = Solution::three_sum(vec![-10, -8, 0, 1, 9]);
        let expected = vec![vec![-10, 1, 9]];
        assert_eq!(result, expected);
    }
}

fn main() {
    println!("Hello, world!");
}
