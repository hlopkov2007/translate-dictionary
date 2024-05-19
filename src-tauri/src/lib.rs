// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod translator;

#[tauri::command] 
fn translate_from_ru_to_en(word: &str) -> String{

    if let Ok(text) = translator::translate(word, "ru", "en") {
        return format!("{text}");
    }
    return format!("{}, not translated", word);
}

#[tauri::command] 
fn translate_from_en_to_ru(word: &str) -> String{

    if let Ok(text) = translator::translate(word, "en", "ru") {
        return format!("{text}");
    }
    return format!("{}, not translated", word);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![translate_from_ru_to_en, translate_from_en_to_ru])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
