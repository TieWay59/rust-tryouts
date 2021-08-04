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
