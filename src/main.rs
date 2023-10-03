use std::env;
use whoami;
use whoami::hostname;
#[allow(unused_imports)]
use std::process::Command;
use std::io::{self, Write};
mod dir;
mod run;
use run::{launch_application, is_application};
use dir::{change_directory, list_directory};

const WELCOME_MESSAGE: &str = "Welcome to Ash, the acronym-less shell, written in Rust!\n";

fn main() {
    let mut current_dir = env::current_dir().expect("Failed to get current directory");

    print!("{}", WELCOME_MESSAGE);
    loop {
        let current_dir_display = display_current_dir(&current_dir).to_string();
        let hostname = get_hostname().unwrap();
        print!(" {} {:?} $ ", hostname, current_dir_display);
        io::stdout().flush().expect("Failed to flush stdout");

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        let user_input = user_input.trim();

        if user_input.eq_ignore_ascii_case("exit") || user_input.eq_ignore_ascii_case("quit") {
            break;
        }

        if user_input.starts_with("echo ") {
            let text_to_echo = &user_input[5..];
            println!("{}", text_to_echo);
        } else if user_input.starts_with("cd ") {
            let new_dir = &user_input[3..];
            if let Err(err) = change_directory(&mut current_dir, new_dir) {
                eprintln!("Failed to change directory: {}", err);
            }
        } else if user_input.eq_ignore_ascii_case("ls") {
            if let Err(err) = list_directory(&current_dir) {
                eprintln!("Failed to list directory: {}", err);
            }

        } else if user_input.starts_with("clear") {
            clear_screen();
        } else if user_input.starts_with("pwd") {
            println!("{}", current_dir.display());
        }
        else if user_input.starts_with("") {
            let app_to_run = &user_input[4..].trim();
        
            if is_application(app_to_run) {
                if let Err(err) = launch_application(app_to_run) {
                    eprintln!("Failed to launch application: {}", err);
                }
            } else {
                eprintln!("Application not found: {}", app_to_run);
            }
        }        
        else {
            println!("Command not supported: {}", user_input);
        }
    }
}

#[allow(dead_code)]
fn print_current_dir(current_dir: &std::path::PathBuf, user: &str) {
    if let Ok(hostname) = get_hostname() {
        let mut current_dir_display = current_dir.display().to_string();
        if current_dir_display.starts_with(&format!("/home/{}/", user)) {
            current_dir_display = current_dir_display.replacen(&format!("/home/{}/", user), "~", 1);
        }
        println!("{}@{}:{}", user, hostname, current_dir_display);
    } else {
        eprintln!("Failed to get hostname");
    }
}

fn display_current_dir(current_dir: &std::path::PathBuf) -> String {
    if let Some(path_str) = current_dir.to_str() {
        path_str.to_string()
    } else {
        // Handle the case where the path cannot be converted to a string
        "Invalid Path".to_string()
    }
}

fn clear_screen() {
    std::process::Command::new("clear").status().unwrap();
}

fn get_hostname() -> std::result::Result<String, String> {
    let username = whoami::username();
    let hostname_str = hostname();
    let final_str = format!("{}@{}", username, hostname_str);
    if final_str.len() == 0 {
        return Err("Failed to get hostname".to_string());
    }
    Ok(final_str)
}