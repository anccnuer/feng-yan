// 编译到wasm命令 wasm-pack build --target web


use base64::decode;
use base64::encode;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn encode_str(s: String) -> String {
    let letters = "abcdefghijklmnopqrstuvwxyzQWERTYUIOPASDFGHJKLMNBVCXZ1234567890=+/";
    let key = "洞呆噤图发噔嘶現袭哮森果达嘿氏人嗥很告出非啽嗡咬类杂取吖诉既盗诱住性更拙意么萌宝嘍雜嗷温咯现達偶象你鱼嚄笨注眠常捕嚁嗒动蜂肉孽迦筻";
    let str = gen(encode(s), letters, key);
    return str;
}
#[wasm_bindgen]
pub fn decode_str(s: String) -> String {
    let letters = "abcdefghijklmnopqrstuvwxyzQWERTYUIOPASDFGHJKLMNBVCXZ1234567890=+/";
    let key = "洞呆噤图发噔嘶現袭哮森果达嘿氏人嗥很告出非啽嗡咬类杂取吖诉既盗诱住性更拙意么萌宝嘍雜嗷温咯现達偶象你鱼嚄笨注眠常捕嚁嗒动蜂肉孽迦筻";
    let temp = &gen(s,key,letters);
    let str= String::from_utf8(decode(&temp).unwrap()).unwrap();
    return str;
}
#[wasm_bindgen]
pub fn gen(s: String, from: &str, to: &str) -> String {
    let mut result = String::new();
    let mut map = HashMap::new();
    for (i, letter) in from.chars().enumerate() {
        let char = to.chars().nth(i).unwrap();
        map.insert(letter, char);
    }

    for c in s.chars() {
        match map.get(&c) {
            Some(char) => result.push(*char),
            None => result.push(c),
        }
    }

    return result;
}
