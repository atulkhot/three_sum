struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut sorted = nums;
        sorted.sort();
        let mut result: Vec<Vec<i32>> = vec![];
        let num_elems = sorted.len();
        let mut left = 0;
        let mut i = left + 1;
        loop {
            i = i + 1; // invariant - i >= left+2
            if i >= num_elems {
                left = left + 1;
                i = left + 2;
            }
            if left + 2 >= num_elems {
                break;
            }
            // assert!(i >= left + 2);
            let sum = sorted[left] + sorted[left + 1] + sorted[i];
            if sum > 0 {
                left = left + 1;
                if left + 2 >= num_elems {
                    break;
                }
                if sorted[left] > 0 && sorted[left + 1] > 0 && sorted[left + 2] > 0 {
                    break;
                }
                i = left + 1;
                continue;
            } else if sum < 0 {
                continue;
            } else {
                result.push(vec![sorted[left], sorted[left + 1], sorted[i]]);
                let elem = sorted[i];
                if i < num_elems && i+1 < num_elems && sorted[i+1] == elem {
                    while i < num_elems && i+1 < num_elems && sorted[i+1] == elem {
                        i += 1;
                    }
                    if i == num_elems-1 && sorted[i] == elem {
                        break;
                    }
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
