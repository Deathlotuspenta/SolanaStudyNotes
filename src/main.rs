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
    println!("Start guess number!");
    
    let secret_number =rand::thread_rng().gen_range(1..=100);

    println!("生成的随机数：{}",secret_number);
        

    // loop 无线循环，除非主动跳出break;
    loop{
        println!("Input your number:");

        // 生成神秘数字
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取失败");

        
        
        // 对 parse() 的结果进行匹配处理。
        // 成功就取出数字；失败就打印提示并重新输入。
        // match 很像 switch
        // parse是对数字取值，返回i32 其中guess i32 就是定义返回类型

        let guess: i32 = match guess.trim().parse(){
            Ok(num) => num,
            // 这里的num是一个变量绑定，表示吧【ok里的值绑定到变量num上】，然后在返回左边的num
            Err(_) =>{
                // Err(错误信息) 这里的_表示忽略错误信息，相当于java的catch(Exception e) {... 但是我没用使用e,打印也没有}
                // 如果要使用 者是Err(e) 然后打印就行
                println!("请输入有效的数字");
                continue;
            }
        };

        // 下方使用if + 符号的形式也行 但是推荐使用这种风格，更安全
        match guess.cmp(&secret_number){
            //  a.cmp(&b) 会返回一个Ordering枚举值：
            // 如果 a < b → Ordering::Less
            // 如果 a > b → Ordering::Greater
            // 如果 a == b → Ordering::Equal

            std::cmp::Ordering::Less => println!("太小了"),
            std::cmp::Ordering::Greater => println!("太大了"),
            std::cmp::Ordering::Equal => {
                println!("正确");
                break;
            }
        }

    }

}