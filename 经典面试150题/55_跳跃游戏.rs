/*
给你一个非负整数数组 nums ，你最初位于数组的 第一个下标 。数组中的每个元素代表你在该位置可以跳跃的最大长度。

判断你是否能够到达最后一个下标，如果可以，返回 true ；否则，返回 false 。

 

示例 1：

输入：nums = [2,3,1,1,4]
输出：true
解释：可以先跳 1 步，从下标 0 到达下标 1, 然后再从下标 1 跳 3 步到达最后一个下标。
示例 2：

输入：nums = [3,2,1,0,4]
输出：false
解释：无论怎样，总会到达下标为 3 的位置。但该下标的最大跳跃长度是 0 ， 所以永远不可能到达最后一个下标。
 

提示：

1 <= nums.length <= 104
0 <= nums[i] <= 105
*/

struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        // 解题思路：直接循环，每次记录当前能到达的最大位置（贪心算法？）
        let mut max_position = 0;
        for i in 0..nums.len() {
            if max_position < i {
                return false;
            }
            max_position = max_position.max(i + nums[i] as usize);
        }

        return true;
    }
}

#[test]
fn test_example_1() {
    // 输入：nums = [2,3,1,1,4]
    // 输出：true
    let nums = vec![2,3,1,1,4];
    
    let k = Solution::can_jump(nums);

    assert_eq!(k, true);
}

#[test]
fn test_example_2() {
    // 输入：nums = [3,2,1,0,4]
    // 输出：false
    let nums = vec![3,2,1,0,4];
    
    let k = Solution::can_jump(nums);

    assert_eq!(k, false);
}

#[test]
fn test_fail_1() {
    // 输入：nums = [0]
    // true
    let nums = vec![0];
    
    let k = Solution::can_jump(nums);

    assert_eq!(k, true);
}

#[test]
fn test_fail_2() {
    // 输入：nums = [0,1]
    // false
    let nums = vec![0,1];
    
    let k = Solution::can_jump(nums);

    assert_eq!(k, false);
}