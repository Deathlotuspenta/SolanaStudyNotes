// 导入随机数包
use rand::Rng;
// 导入rand包中的Rng特性（trait）
// 什么是trait？
// Rust中没有类这种概念，它是结构体和特性组成，Java中的类和接口
// 特性相当于Java中的接口，实现特性就是相当于实现接口

// 从标准库（std）里引入 io 模块，用于输入输出操作。
use std::io;


// fn main() {
//     println!("Start Guess Game！");
//     let secret_number = rand::thread_rng().gen_range(1..=100);
//     println!("secret number is {}",secret_number);

//     println!("Please,Input a number");

//     let mut user_guess_number = String::new();
//     // String::new() 创建一个新的空字符串对象  内存：user_guess_number  ─▶  "" （空字符串）

//     io::stdin().read_line(&mut user_guess_number).expect("读取失败");
//     // io::stdin()  获取一个标准的输入对象，键盘输入流相当于：Scanner sc = new Scanner(System.in)
//     // read_line(&mut 变量) 这个是一个方法， 从键盘读取一行输入，并写入变量中，&：传引用 mut 可变引用  “我把变量 user_guess_number 的地址借给你，你可以往里面写数据。”

//     // .expect("信息") 错误处理

//     println!("Your guess number is {}",user_guess_number);
// }

fn main(){
    
}