use std::process::Command;
use std::env;

pub fn execute_command(command: &str, args: Vec<&str>) -> Result<(), String> {
    let result = Command::new(command)
        .args(args)
        .spawn();

    match result {
        Ok(mut child) => {
            let status = child.wait();
            match status {
                Ok(exit_status) => {
                    if exit_status.success() {
                        Ok(())
                    } else {
                        Err(format!(
                            "Command '{}' failed with exit code {}",
                            command,
                            exit_status.code().unwrap_or(1)
                        ))
                    }
                }
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

pub fn sudo() -> Result<(), String> {
    // BUILT-IN SUDO FUNCTIONALITY WITHOUT USER NEEDING SUDO ON THEIR SYSTEM
    todo!(); // cancelled COMMAND, WILL DELETE FROM CODEBASE SOON
}