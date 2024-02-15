# feng-yan
疯言疯语，仿『与佛论道』，但是wasm
## 原理
先将要加密的内容**base64**编码  
对**base64**编码的内容按映射表进行字符串替换  
**rust代码在**在rust-wasm目录下
当前映射：
```rust
  let letters = "abcdefghijklmnopqrstuvwxyzQWERTYUIOPASDFGHJKLMNBVCXZ1234567890=+/";
  let key = "洞呆噤图发噔嘶現袭哮森果达嘿氏人嗥很告出非啽嗡咬类杂取吖诉既盗诱住性更拙意么萌宝嘍雜嗷温咯现達偶象你鱼嚄笨注眠常捕嚁嗒动蜂肉孽迦筻";
```