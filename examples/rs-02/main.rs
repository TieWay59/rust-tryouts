use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("猜数字：");
    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("读取行失败");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入数字！");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("输入 < 答案"),
            Ordering::Greater => println!("输入 > 答案"),
            Ordering::Equal => {
                println!("输入 = 答案");
                break;
            }
        }
    }
}
