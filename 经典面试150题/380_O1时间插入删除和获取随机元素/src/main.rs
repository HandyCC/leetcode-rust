/*
实现RandomizedSet 类：

RandomizedSet() 初始化 RandomizedSet 对象
bool insert(int val) 当元素 val 不存在时，向集合中插入该项，并返回 true ；否则，返回 false 。
bool remove(int val) 当元素 val 存在时，从集合中移除该项，并返回 true ；否则，返回 false 。
int getRandom() 随机返回现有集合中的一项（测试用例保证调用此方法时集合中至少存在一个元素）。每个元素应该有 相同的概率 被返回。
你必须实现类的所有函数，并满足每个函数的 平均 时间复杂度为 O(1) 。

 

示例：

输入
["RandomizedSet", "insert", "remove", "insert", "getRandom", "remove", "insert", "getRandom"]
[[], [1], [2], [2], [], [1], [2], []]
输出
[null, true, false, true, 2, true, false, 2]

解释
RandomizedSet randomizedSet = new RandomizedSet();
randomizedSet.insert(1); // 向集合中插入 1 。返回 true 表示 1 被成功地插入。
randomizedSet.remove(2); // 返回 false ，表示集合中不存在 2 。
randomizedSet.insert(2); // 向集合中插入 2 。返回 true 。集合现在包含 [1,2] 。
randomizedSet.getRandom(); // getRandom 应随机返回 1 或 2 。
randomizedSet.remove(1); // 从集合中移除 1 ，返回 true 。集合现在包含 [2] 。
randomizedSet.insert(2); // 2 已在集合中，所以返回 false 。
randomizedSet.getRandom(); // 由于 2 是集合中唯一的数字，getRandom 总是返回 2 。
 

提示：

-231 <= val <= 231 - 1
最多调用 insert、remove 和 getRandom 函数 2 * 105 次
在调用 getRandom 方法时，数据结构中 至少存在一个 元素。
*/

use std::collections::{HashMap};

use rand::Rng;

struct RandomizedSet {
    values: Vec<i32>,
    map: HashMap<i32, usize>,
}


impl RandomizedSet {

    fn new() -> Self {
        Self {
            values: vec![],
            map: HashMap::new(),
        }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        if let Some(_) = self.map.get(&val) {
            return false;
        } else {
            self.values.push(val);
            let index = self.values.len() - 1;
            self.map.insert(val, index);
            return true;
        }
    }
    
    fn remove(&mut self, val: i32) -> bool {
        if let Some(&index) = self.map.get(&val) {
            self.map.remove(&val);
            // 把末尾元素移动到这里
            let last_index = self.values.len()-1;
            if last_index >= 1 && last_index != index {
                self.values[index] = self.values[last_index];
                self.map.insert(self.values[index], index);
            }
            self.values.pop();

            return true;
        } else {
            return false;
        }
    }
    
    fn get_random(&mut self) -> i32 {
        let mut rnd = rand::thread_rng();
        let index = rnd.gen_range(0..self.values.len());
        self.values[index]
    }
}



 #[test]
 fn test() {
    // 输入
    // ["RandomizedSet", "insert", "remove", "insert", "getRandom", "remove", "insert", "getRandom"]
    // [[], [1], [2], [2], [], [1], [2], []]
    // 输出
    // [null, true, false, true, 2, true, false, 2]
    let mut obj = RandomizedSet::new();
    assert_eq!(obj.insert(1), true);
    assert_eq!(obj.remove(2), false);
    assert_eq!(obj.insert(2), true);
    let ret_3: i32 = obj.get_random();
    assert!(ret_3 == 1 || ret_3 == 2);
    assert_eq!(obj.remove(1), true);
    assert_eq!(obj.insert(2), false);
    assert_eq!(obj.get_random(), 2);
}

fn main() {

}