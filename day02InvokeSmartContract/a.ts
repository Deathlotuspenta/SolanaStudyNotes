// 新建一个交易对象
//一个交易可以包含一个或者多个Instruction（指令），每一个指令代表一次智能合约调用或系统操作
// 可以往transaction中add()指令，最后整体签名发送
const transaction = new web3.Transaction();

// 创建一个指令（Instruction）
transaction.add(
  new web3.TransactionInstruction({
    keys: [],
    programId: new web3.PublicKey(pg.PROGRAM_ID),
  })
);

// TransactionInstruction 代表交易中的单条指令
// programId 是要调用的Solana程序地址（智能合约地址）
// keys 是需要访问的账户列表（如：读取余额、写入状态的账户）
// 每个key通常为:{pubkey:PublicKey,isSigner:boolean,isWritable:boolean} ,这里为空表示不访问任何额外账户，也就是只调用程序不进行账户交互

console.log("Sending transaction...");

// 发送并确认
const txHash = await web3.sendAndConfirmTransaction(
  pg.connection, // 当前链接的solana网络
  transaction, // 发送的交易对象 也就是上方创建的交易对象
  [pg.wallet.keypair] // 用于签名交易的密钥对数组（这里是钱包的力邀） 签名是必须的，因为solana需要验证发送者的身份
);

console.log("Transaction sent with hash:", txHash);
