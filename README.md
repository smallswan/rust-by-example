# rust-by-example
本项目受[《Rust By Example》](https://doc.rust-lang.org/rust-by-example/) 启发，希望通过具体、完整的示例代码，来进一步掌握rust编程语言。本项目主要包括一些第三方rust库的示例代码，以及一些实用的算法。
## rust第三方库
- [rand](https://github.com/rust-random/rand) 随机数
- [rust-crypto](https://github.com/DaGenix/rust-crypto/)  RUST加解密算法库  
- [chrono](https://github.com/chronotope/chrono)  Date and Time for Rust  
- [serde](https://serde.rs/)  序列化和反序列化框架  
- [rustc_serialize](https://docs.rs/rustc-serialize/0.3.24/rustc_serialize/) 简单的编码、解码库

## 实用的工具：
- 身份证号验证
- 统一社会信用代码验证
- 雪花算法(snowflake)
- 敏感词检测（DFA算法）

## 运行测试
```
cargo test -- --nocapture validators::tests::money_test
cargo test -- --nocapture random::tests::first_demo
cargo test -- --nocapture tests::hof
cargo test -- --nocapture sensitive_word
```
```
cargo run --example convert .\examples\scaledown\借条.jpeg  .\examples\scaledown\借条-half-height.jpeg
```