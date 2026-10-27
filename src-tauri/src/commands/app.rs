use crate::storage::ensure_storage_directory;
use serde::Serialize;
use tauri::Manager;

#[derive(Serialize)]
pub struct AppInfo {
    name: String,
    version: String,
    storage_mode: String,
}

#[tauri::command]
pub fn get_app_info() -> AppInfo {
    AppInfo {
        name: "Stash".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        storage_mode: "local".to_string(),
    }
}

#[tauri::command]
pub fn initialise_storage(app: tauri::AppHandle) -> Result<String, String> {
    let documents = app
        .path()
        .document_dir()
        .map_err(|error| error.to_string())?;

    let stash_directory = documents.join("Stash");

    ensure_storage_directory(&stash_directory).map_err(|error| error.to_string())?;

    Ok(stash_directory.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_info_contains_expected_values() {
        let info = get_app_info();

        assert_eq!(info.name, "Stash");
        assert_eq!(info.storage_mode, "local");
        assert!(!info.version.is_empty());
    }
}
