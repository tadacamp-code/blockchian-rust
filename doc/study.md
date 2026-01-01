# Rust 语法知识点整理（由易到难）

1. 行注释 `//`
   - 说明：单行注释，从 `//` 开始到行尾都会被编译器忽略。
   - 示例：
   ```rust
   //self.blocks.push(new_block);
   ```

2. 模块声明 `mod`
   - 说明：在 crate 根或模块内声明子模块，告诉编译器去加载对应文件。
   - 示例：
   ```rust
   mod block;
   mod blockchain;
   ```

3. 导入语句 `use`
   - 说明：把路径引入当前作用域，简化后续调用；可使用嵌套路径或通配符 `*`。
   - 示例：
   ```rust
   use crate::block::*;
   use crypto::{digest::Digest, sha2::Sha256};
   ```

4. 函数定义与返回类型
   - 说明：`fn` 定义函数，参数类型必写；可用 `->` 指定返回类型。
   - 示例：
   ```rust
   fn main() {
       println!("Hello, world!");
   }

   pub fn new_block(data: String, prev_block_hash: String, height: usize) -> Result<Block> {
       /* ... */
   }
   ```

5. 结构体定义与字段
   - 说明：`struct` 定义数据结构；字段名和类型成对出现。
   - 示例：
   ```rust
   pub struct Block {
       timestamp: u128,
       transactions: String,
       prev_block_hash: String,
       hash: String,
       height: usize,
       nonce: i32,
   }
   ```

6. 可见性 `pub`
   - 说明：`pub` 让类型或函数对外可见；未标注的字段/方法默认私有。
   - 示例：
   ```rust
   pub struct Blockchain {
       current_hash: String,
       db: sled::Db,
   }
   ```

7. `impl` 块与方法接收者 `self`
   - 说明：`impl` 为类型添加方法；`&self` 是不可变借用，`&mut self` 是可变借用。
   - 示例：
   ```rust
   impl Block {
       pub fn get_hash(&self) -> String {
           self.hash.clone()
       }

       fn run_proof_if_work(&mut self) -> Result<()> {
           /* ... */
       }
   }
   ```

8. 常量 `const` 与类型别名 `type`
   - 说明：`const` 定义编译期常量；`type` 为复杂类型起别名。
   - 示例：
   ```rust
   pub const TARGET_HEXS: usize = 4;
   pub type Result<T> = std::result::Result<T, failure::Error>;
   ```

9. 宏调用 `!`
   - 说明：宏用 `!` 调用，常见于打印和调试输出。
   - 示例：
   ```rust
   println!("item {:?}", item);
   dbg!(&b);
   info!("Mining the block");
   ```

10. 属性 `#[...]`（派生与条件编译）
    - 说明：`#[derive(...)]` 自动生成 trait 实现；`#[cfg(test)]` 只在测试时编译。
    - 示例：
    ```rust
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Block { /* ... */ }

    #[cfg(test)]
    mod tests { /* ... */ }
    ```

11. 所有权与借用（`&` / `&mut` / `clone`）
    - 说明：`&` 借用不转移所有权，`&mut` 可变借用；`clone()` 产生新所有权。
    - 示例：
    ```rust
    hasher.input(&data[..]);
    self.current_hash = block.get_prev_hash();
    ```

12. 泛型类型与 `turbofish`
    - 说明：泛型用 `<T>` 表示；`::<T>` 明确指定类型参数。
    - 示例：
    ```rust
    let mut vec1: Vec<u8> = vec![];
    if let Ok(block) = bincode::deserialize::<Block>(&b) { /* ... */ }
    ```

13. 控制流：`match` / `if let` / `while` / `for`
    - 说明：`match` 做全匹配；`if let` 简化单分支匹配；`while` 循环；`for` 迭代器循环。
    - 示例：
    ```rust
    match db.get("LAST")? {
        Some(hash) => { /* ... */ }
        None => { /* ... */ }
    }

    while !self.validate()? {
        self.nonce += 1;
    }

    for item in b.iter() {
        println!("item {:?}", item);
    }
    ```

14. `Result` / `Option` 与错误传播 `?`
    - 说明：`?` 自动把 `Err` 返回给调用方；`unwrap()` 直接取值，失败会 panic。
    - 示例：
    ```rust
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_millis();

    let mut b = Blockchain::new().unwrap();
    ```

15. Trait 实现与关联类型
    - 说明：为类型实现 trait；`type Item = ...` 是关联类型。
    - 示例：
    ```rust
    impl<'a> Iterator for BlockchainIter<'a> {
        type Item = Block;
        fn next(&mut self) -> Option<Self::Item> { /* ... */ }
    }
    ```

16. 生命周期参数 `'a`
    - 说明：生命周期参数标注引用关系，确保引用有效期不超过被借用对象。
    - 示例：
    ```rust
    pub struct BlockchainIter<'a> {
        current_hash: String,
        bc: &'a Blockchain,
    }
    ```
