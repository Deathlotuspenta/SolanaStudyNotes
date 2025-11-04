// 导入 solana-program Solana 官方提供的 SDK 包，智能合约的所有基础功能都来自这里
// Account_info ：帐户详细信息
// entrypoint 程序入口
// msg ：在 Solana 上打印消息
// AccountInfo：封装了账户的详细信息，比如余额、所有者、公钥、数据等。

// entrypoint：宏定义，用来告诉 Solana 程序从哪里开始执行。

// ProgramResult：函数返回类型，表示智能合约执行的结果，常见是 Ok(()) 或 Err(ProgramError::InvalidInstructionData)。

// msg!：Solana 提供的打印函数（在链上日志中可查看），相当于 println!()。
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

// 定义代码的入口，开始执行智能合约
entrypoint!(process_instruction);

// 定义一个名为 process_instruction 的公共函数。参数为程序 id、帐户和指令数据字段
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // 将“Hello World ”消息打印在 Solana 区块链上
    msg!("Hello, Solana!刘昌伟恭喜你能在链上发布消息");

    // 向系统返回状态代码来退出程序
    Ok(())
}

// fn example1() -> i32 {
//     5; // 执行，但不返回
//     6; // 执行，但不返回
// }

// fn example2() -> i32 {
//     5;  // 这行执行但丢弃
//     6   // 最后一行没有分号 => 返回 6
// }
