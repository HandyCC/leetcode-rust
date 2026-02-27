/*
给定一个大小为 n 的数组 nums ，返回其中的多数元素。多数元素是指在数组中出现次数 大于 ⌊ n/2 ⌋ 的元素。

你可以假设数组是非空的，并且给定的数组总是存在多数元素。

 

示例 1：

输入：nums = [3,2,3]
输出：3
示例 2：

输入：nums = [2,2,1,1,1,2,2]
输出：2
 

提示：
n == nums.length
1 <= n <= 5 * 104
-109 <= nums[i] <= 109
输入保证数组中一定有一个多数元素。
 

进阶：尝试设计时间复杂度为 O(n)、空间复杂度为 O(1) 的算法解决此问题。
*/

struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // 摩尔投票法：高效、空间占用低，在算法竞赛、数据流处理和大数据分析中被广泛使用。
        let mut votes = 0;
        let mut ans = 0;

        for num in nums {
            if votes == 0 {
                ans = num;
            }
            if ans == num {
                votes += 1;
            } else {
                votes -= 1;
            }
        }

        return ans;
    }
}



#[test]
fn test_example_1() {
    // 输入：nums = [3,2,3]
    // 输出：3
    let nums = vec![3,2,3];
    
    let k = Solution::majority_element(nums);

    assert_eq!(k, 3);
}

#[test]
fn test_fail() {
    // 输入：nums = [6,5,5]
    // 输出：5
    let nums = vec![6,5,5];
    
    let k = Solution::majority_element(nums);

    assert_eq!(k, 5);
}