#![allow(unused_variables, dead_code)]

/* 
ゴール：
1. cargo run -- Cargo.toml と実行して、このプログラムの動作を確認してください
2. 存在しないファイル名を指定して実行すると、panicがおきます。修正してください
3. コンビネータを使って、read_file2を実装してください
*/

use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut args = env::args();
    if let Some(file) = args.nth(1) {
       match read_file(&file) {
           Ok(file) => println!("{}", file),
           Err(err) => println!("no such file or directory")
       }
       match read_file2(&file) {
           Ok(file) => println!("{}", file),
           Err(err) => println!("{}", err)
       }
    }
}

fn read_file(filename: &String) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn read_file2(filename: &String) -> Result<String, String> {
    File::open(filename)
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|err| err.to_string())
                .map(|_| contents)
        })
}

