// Module building the end-result Rust code
use std::str;
use std::path::Path;
use crate::arg_parser::{Order, Execution};
use std::fs::{self, OpenOptions};
use std::io::prelude::*;

fn search_and_replace(path_to_main: &Path, search: &str, replace: &str) -> Result<(), Box<dyn std::error::Error>> {
    // thanks to https://users.rust-lang.org/t/replacing-content-in-file/52690/5
    let file_content = fs::read_to_string(path_to_main)?;
    let new_content = file_content.replace(search, &replace);

    let mut file = OpenOptions::new().write(true).truncate(true).open(path_to_main)?;
    file.write(new_content.as_bytes())?;

    Ok(())
}

fn vec_to_string(shellcode: Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
    //println!("{:?}", shellcode.clone());
    let result: String = String::from_utf8(shellcode)?;
    Ok(result)
}

pub fn meta_puzzle(order: Order, shellcode: Vec<u8>) {
    let path_to_main;
    match order.execution {
        Execution::CreateThread => path_to_main = Path::new("templates/createThread/src/main.rs"),
        _ => panic!("Don't even know how this error exists."),
    }
    let search = "{{shellcode}}";
    let replace: String;
    match vec_to_string(shellcode) {
        Ok(content) => replace = content,
        Err(err) => panic!("{:?}", err),
    }
    let _ = search_and_replace(path_to_main, search, &replace);
}