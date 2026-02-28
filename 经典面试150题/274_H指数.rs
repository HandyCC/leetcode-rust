/*
给你一个整数数组 citations ，其中 citations[i] 表示研究者的第 i 篇论文被引用的次数。计算并返回该研究者的 h 指数。

根据维基百科上 h 指数的定义：h 代表“高引用次数” ，一名科研人员的 h 指数 是指他（她）至少发表了 h 篇论文，并且 至少 有 h 篇论文被引用次数大于等于 h 。如果 h 有多种可能的值，h 指数 是其中最大的那个。

 

示例 1：

输入：citations = [3,0,6,1,5]
输出：3 
解释：给定数组表示研究者总共有 5 篇论文，每篇论文相应的被引用了 3, 0, 6, 1, 5 次。
     由于研究者有 3 篇论文每篇 至少 被引用了 3 次，其余两篇论文每篇被引用 不多于 3 次，所以她的 h 指数是 3。
示例 2：

输入：citations = [1,3,1]
输出：1
 

提示：

n == citations.length
1 <= n <= 5000
0 <= citations[i] <= 1000
*/

struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        // 解题思路：计数排序（将数量和引用次数绑定到一起）
        let n = citations.len();
        let mut cnt = vec![0; n+1]; // 数量可能是0到n个
        for c in citations {
            cnt[n.min(c as usize)] += 1;     // 结果不会大于n，所以引入次数大于n的当做n处理
        }

        let mut s = 0;
        for (i, &c) in cnt.iter().enumerate().rev() {
            s += c;
            if s >= i {
                return i as _;
            }
        }
        unreachable!()
    }
}

#[test]
fn test_example_1() {
    // 输入：citations = [3,0,6,1,5]
    // 输出：3 
    let citations = vec![3,0,6,1,5];

    let k = Solution::h_index(citations);

    assert_eq!(k, 3);
}

#[test]
fn test_fail_1() {
    // 输入：citations = [100]
    // 输出：1
    let citations = vec![100];

    let k = Solution::h_index(citations);

    assert_eq!(k, 1);
}