// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn sum_heji(shuzi1: &str, shuzi2: &str) -> String {
    let shuzi1 = shuzi1.parse::<u32>().unwrap();
    let shuzi2 = shuzi2.parse::<u32>().unwrap();
    let heji = shuzi1 + shuzi2;
    format!("计算结果合计为：{}", heji)
}

#[tauri::command]
fn my_custom_command() {
    println!("I was invoked from JavaScript!");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, sum_heji, my_custom_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
