#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[tauri::command]
fn calculate(file1_path: String, file2_path: String, operation: String) -> Result<String, String> {
    let file1_lines = read_lines(&file1_path).map_err(|e| e.to_string())?;
    let file2_lines = read_lines(&file2_path).map_err(|e| e.to_string())?;

    let set1: HashSet<_> = file1_lines.into_iter().collect();
    let set2: HashSet<_> = file2_lines.into_iter().collect();

    let result_set: HashSet<String> = match operation.as_str() {
        "intersection" => set1.intersection(&set2).cloned().collect(), // 交集
        "union" => set1.union(&set2).cloned().collect(),               // 并集
        "difference" => set1.difference(&set2).cloned().collect(),     // 差集
        _ => return Err("无效的操作".into()),
    };

    let result = result_set.into_iter().collect::<Vec<_>>().join("\n");
    Ok(result)
}

// 按行读取文件
fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);
    buf.lines().collect()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![calculate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
