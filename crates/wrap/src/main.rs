#![allow(clippy::print_stdout)]
mod wrap;
use std::path::PathBuf;
use wrap::{Wrap, WrapConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user_root_path = String::from("test_projects/event");
    // let user_root_path = String::from("test_projects/temp/asset");
    let user_language_path = String::from("src/languages/index.ts");
    let user_include = vec![String::from("src")];
    let user_exclude = vec![String::from("src/languages")];
    wrap(
        user_root_path,
        user_language_path,
        user_include,
        user_exclude,
    )
}

pub fn wrap(
    user_root_path: String,
    user_language_path: String,
    user_include: Vec<String>,
    user_exclude: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // 根路径
    let root = PathBuf::from(user_root_path.as_str());
    if !root.exists() {
        println!("根路径不存在");
        return Ok(());
    }
    // 国际化文件路径
    let language_path = root.join(user_language_path);
    if !language_path.exists() {
        println!("国际化文件路径不存在");
        return Ok(());
    }

    let wrapped_config = WrapConfig {
        root_path: root.clone(),
        language_path: language_path.clone(),
        include: user_include,
        exclude: user_exclude,
    };
    let mut wrap = Wrap::new(wrapped_config);
    wrap.wrap();

    Ok(())
}
