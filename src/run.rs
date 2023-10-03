use std::process::Command;
use std::env;

pub fn launch_application(app_name: &str) -> Result<(), String> {
    let result = Command::new(app_name).spawn();
    match result {
        Ok(mut child) => {
            // Wait for the application to finish executing
            let status = child.wait();
            match status {
                Ok(_) => Ok(()),
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

pub fn is_application(command: &str) -> bool {
    if let Ok(path_var) = env::var("PATH") {
        let paths: Vec<_> = env::split_paths(&path_var).collect();

        for path in paths {
            let path_entry = path.join(command);
            if path_entry.is_file() {
                return true;
            }
        }
    }
    
    false
}