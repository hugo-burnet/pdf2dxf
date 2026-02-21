mod pdf_converter;

use std::path::Path;
use lopdf::Document;

#[tauri::command]
fn convert_pdf(input_path: String, scale_factor: f64, unit: String) -> Result<String, String> {
    let input_pdf_path = Path::new(&input_path);
    if !input_pdf_path.exists() {
        return Err(format!("Le fichier '{}' est introuvable.", input_path));
    }

    let output_dxf_path = input_pdf_path.with_extension("dxf");
    let output_path_str = output_dxf_path.to_str().ok_or("Invalid output path")?.to_string();

    // Chargement du PDF via lopdf
    let doc = Document::load(input_pdf_path)
        .map_err(|e| format!("Erreur lors du décodage du PDF : {:?}", e))?;

    // Extraction des images
    pdf_converter::extract_images(&doc, &output_dxf_path);

    // Extraction des vecteurs
    let lines = pdf_converter::extract_pdf_paths(&doc);

    if lines.is_empty() {
        return Err("Aucun vecteur graphique n'a été trouvé dans le PDF.".to_string());
    }

    // Génération du DXF avec facteur d'échelle et unité
    pdf_converter::generate_dxf(&lines, &output_path_str, scale_factor, &unit)
        .map_err(|e| format!("Erreur lors de la génération du DXF : {:?}", e))?;

    Ok(output_path_str)
}

#[tauri::command]
fn open_dxf(app: tauri::AppHandle, path: String) -> Result<(), String> {
    use tauri_plugin_opener::OpenerExt;
    app.opener().open_path(path.clone(), None::<&str>).map_err(|e| format!("Impossible d'ouvrir le fichier : {}", e))?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![convert_pdf, open_dxf])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
