extern crate bip39;

use ::bip39::{Mnemonic, MnemonicType, Language};

fn main() {
    let mnemonic_type = MnemonicType::for_word_count(12).unwrap();
    let mnemonic= match Mnemonic::new(mnemonic_type, Language::English, "") {
        Ok(b) => b,
        Err(_) => { assert!(false); return }
    };
    let phrase = str::replace(&mnemonic.get_string(), " ", "\", \"");
    println!("cardano mnemonics are [\"{}\"]", phrase);
}
