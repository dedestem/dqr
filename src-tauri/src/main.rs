//#![cfg_attr(
//    all(not(debug_assertions), target_os = "windows"),
//    windows_subsystem = "windows"
//)]

use qrcode::QrCode;
use qrcode::render::svg;
use tauri::{Config, State};
use directories::UserDirs;
use std::{fs::File, io::Write};
use base64::engine::general_purpose;
use base64::Engine;

fn sanitize_filename(filename: &str) -> String {
    // Maak een whitelist van toegestane tekens
    let allowed_chars: &str = if cfg!(target_os = "windows") {
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789._-"
    } else {
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789._-"
    };

    // Filter het bestand door alleen toegestane tekens te behouden
    filename
        .chars()
        .filter(|c| allowed_chars.contains(*c))
        .collect()
}

#[tauri::command]
fn get_downloads_path() -> Option<String> {
    UserDirs::new()
        .and_then(|user_dirs| user_dirs.download_dir().map(|path| path.to_path_buf()))
        .map(|path_buf| path_buf.to_string_lossy().into_owned())
}


#[tauri::command]
fn generate_qr_code(data: String, _config: State<'_, Config>) -> Result<String, String> {
    // Genereer de QR-code
    let code = QrCode::new(data).map_err(|e| e.to_string())?;

    // De gerenderde QR-code als SVG
    let svg_code: String = code.render::<svg::Color>().min_dimensions(200, 200).build();

    // Encodeer de SVG-code naar een base64-string
    let base64_svg = general_purpose::STANDARD.encode(svg_code);

    // Maak een data URL
    let data_url = format!("data:image/svg+xml;base64,{}", base64_svg);
    Ok(data_url)
}


#[tauri::command]
fn generate_qr_code_export(data: String) -> Result<(), String> {
    // Genereer de QR-code
    let code = QrCode::new(data.clone()).map_err(|e| e.to_string())?;

    // De gerenderde QR-code als SVG
    let svg_code: String = code.render::<svg::Color>().min_dimensions(200, 200).build();

    // Verkrijg het pad naar de download directory
    let downloads_path = get_downloads_path().ok_or("Unable to get downloads path")?;
    
    // Clean Data
    let data: String = sanitize_filename(&data);

    // Maak de volledige bestandsnaam en pad aan
    let file_name = format!("qr_{}.svg", data);
    let file_path = format!("{}/{}", downloads_path, file_name);
    
    println!("{}", file_path);
    // Sla het SVG-bestand op
    let mut file = File::create(file_path).map_err(|e| e.to_string())?;
    file.write_all(svg_code.as_bytes()).map_err(|e| e.to_string())?;

    Ok(())
}


fn main() {
    tauri::Builder::default()
        .manage(Config::default())
        .invoke_handler(tauri::generate_handler![generate_qr_code , get_downloads_path, generate_qr_code_export])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
