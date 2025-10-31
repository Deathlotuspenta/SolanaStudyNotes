fn main() {
    // let mut a = 1;
    // a = 2;
    // println!("Hello, world!");
    // println!("a的值:{}",a)



    // let a = -1;
    // println!("{}",a)

    // let resutl = 
    
    // i:有符号整型 也就是有负数
    // u:无符号整型，无负数

    // println!("{}",function_study(1))

    ownership_condition_transfered_use();

}


// fn add(x: i32, y: i32) -> i32	接收两个 i32，返回一个 i32
// fn say_hello() -> String	不接收参数，返回一个 String
// fn print_hi() -> ()	返回空值（等价于 void）
// 箭头代表返回值
// 以及传入的参数必须声明类型!!!!!必须写！！！！

fn function_study(x:i32)-> i32{



    return x+1;
}

// 权限转移 /  所有权

// 所有权可以提高 Rust 性能，避免大部分内存泄漏问题

// Rust中的每一个值都有一个对应的变量作为它的所有者

// 在同一时间内，值有且仅有一个所有者

// 当所有者离开自己的作用域时，它持有的值就会被释放掉  

// String::from : 用于创建有一块所有权的内存


fn ownership_condition1(){
    let a = String::from("a");
    println!("{}",a)
} // 这里a就会消失  没有人所有


// fn ownership_condition2(){
//     let a = String::from("a");
//     let b = a;
//     println!("{}",a) // 报错，无法使用a 因为a的值1 已经属于了b
// }

// 如何使用已经转移所有权的 变量？

fn ownership_condition_transfered_use() -> (){
    let a = String::from("ab使用转移所有权的变量");
    let b = a.clone(); // 使用深拷贝
    println!("{}",b)
}