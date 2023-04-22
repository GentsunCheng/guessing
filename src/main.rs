use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() 
{
    println!("猜数游戏 ");
    let num = rand::thread_rng().gen_range(1..101);
    loop
    {
        println!("请猜测一个1-100的数!");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");

        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜测的数是: {}",guess);

        match guess.cmp(&num)
        {
            Ordering::Less => println!("太小"),
            Ordering::Greater => println!("太大"),
            Ordering::Equal =>
            {
                println!("获胜");
                break;
            }
        }
    }
}
