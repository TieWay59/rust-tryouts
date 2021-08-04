fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = &item;
        }
    }
    largest
}
//(dɪˈvɛləp)
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_ref(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_ref(&char_list);
    println!("The largest char is {}", result);

    println!("{}", 0b_0101_1010.to_string());

    let s1 = 2;
    let s2 = 2;
    let t1 = &s1;
    let t2 = &s2;
    assert_eq!(t1, t2); // deref? &i32 cmp &i32, 会自动解&
    assert_eq!(t1 == t2, true);

    let s1 = String::from("hhh");
    let s2 = String::from("hhh");
    let t1 = &s1;
    let t2 = &s2;
    assert_eq!(t1, t2);
    assert_eq!(t1 == t2, true);
}
