//!
//! 数値文字参照の変換
//!

/// 数値文字参照
///
/// # Arguments
/// * `str` - 文字列
/// 
/// # Returns
/// * `String` - 変換後の文字列
fn generate_str2num(str: &str) -> String {
    let mut response= "".to_string();
    for c in str.chars() {
        let num = c as i32;
        response.push_str("&#");
        response.push_str(&num.to_string());
        response.push_str(";");
    }
    return response;
}

/// アプリケーションのエントリーポイントです。
fn main() {
    let args: std::vec::Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("Usage: {} \"characters\"", args[0]);
        // std::process::exit(1);
        return;
    }
    println!("{}", generate_str2num(args[1].as_str()));
}
