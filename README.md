# Rust 练习

## 阶乘

这样写的好处在于，fac 不需要 mut。

```rust
{
    let n = 1000000;
    let m = 1000000007;
    let fac: Vec<i64> = (1..=n)
        .scan(1, |acc, n| {
            *acc = *acc * n % m;
            Some(*acc)
        })
        .collect();

    assert_eq!(&fac[..5], [1, 2, 6, 24, 120]);
}
```

## 无效的 range 不会执行

```rust
    let a = [1, 2, 3];
    for i in 0..a.len() {
        for j in i + 3..a.len() {
            println!("{}", a[j]);
        }
    }
```

## 逆转整数

这里演示了一个返回`Option`的优雅写法，里面返回 `None` 的情况就是 checked 算术。

问号属于 `match` 的省略写法，直接把 `None` 抛出去，对于简单函数很方便。（找一下文档里叫什么）

本质上也是借用语言特性。

```rust
pub fn reverse(x: i32) -> i32 {
    fn rev(mut x: i32) -> Option<i32> {
        let mut ret = 0i32;
        while x != 0 {
            ret = ret.checked_mul(10)?.checked_add(x % 10)?;
            x = x / 10;
        }
        Some(ret)
    }
    rev(x).unwrap_or(0)
}
```

## upper_bound

正经 `upper_bound` 的对应在 rust 的 std 里是 `partition_point` 这个 v1.52.0 才让用。

如果你使用了 `binary_search` 就会遇到很无语的情况，看源码的实现就明白了。简单地说，它是不稳定的，如果有一段相同的数字，返回其中任何（\*）一个下标都有可能（\* 针对每个数据是确定的，但不能保证头尾）。

```rust
impl Solution {
    // https://leetcode-cn.com/problems/next-permutation/solution/
    pub fn next_permutation(nums: &mut Vec<i32>) {
        for i in (0..nums.len() - 1).rev() {
            if nums[i] < nums[i + 1] {
                nums[i + 1..].reverse();
                // let j = i + 1 + nums[i + 1..].partition_point(|&x| x <= nums[i]); stable 1.52.0
                // nums.swap(i, j);
                for j in i + 1..nums.len() {
                    if nums[i] < nums[j] {
                        nums.swap(i, j);
                        break;
                    }
                }
                return;
            }
        }
        nums.reverse();
    }
}
/*
Line 7, Char 47: use of unstable library feature 'partition_point': new API (solution.rs)
  |
7 |                 let j = i + 1 + nums[i + 1..].partition_point(|&v| v <= nums[i]);
  |                                               ^^^^^^^^^^^^^^^
  |
  = note: see issue #73831 <https://github.com/rust-lang/rust/issues/73831> for more information
error: aborting due to previous error
*/

```

## for 写法

有时候 n 不是 usize 类型，这样会导致所有的 i 都不是。

如果后续要把 i 作为下标，就每一处都要转型，比较烦。

可以像下面这样避开一部分。

```rust
for i in 1..n as usize {
    // --snip--
}
```
