#![allow(clippy::print_stdout)]
mod wrap;

use wrap::wrap;

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