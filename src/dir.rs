use std::fs;

pub fn change_directory(current_dir: &mut std::path::PathBuf, new_dir: &str) -> Result<(), String> {
    let new_path = current_dir.join(new_dir);
    if !new_path.exists() || !new_path.is_dir() {
        return Err(format!("Directory '{}' does not exist.", new_dir));
    }

    *current_dir = new_path.canonicalize().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn list_directory(current_dir: &std::path::PathBuf) -> Result<(), String> {
    let entries = fs::read_dir(current_dir).map_err(|e| e.to_string())?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        println!("{}", entry.file_name().to_string_lossy());
    }
    Ok(())
}