use std::{io::Write, process::Command};
fn main() {
    let mut activate_command: String = String::from("source venv/bin/activate");
    if std::env::consts::OS == "windows" {
        activate_command = String::from("venv\\Scripts\\activate");
    }

    // Creates virual environment
    println!("Creating virtual environment");
    Command::new("python")
        .arg("-m")
        .arg("venv")
        .arg("venv")
        .output()
        .expect("Failed to create virtual environment");

    println!("Successfully created the virual environment. You need to run the command {} to activate the venv", activate_command);

    let git_init = Command::new("git")
        .arg("init")
        .output()
        .expect("Failed to initialize git");

    println!("{}", String::from_utf8_lossy(&git_init.stdout));

    let mut gitignore_file =
        std::fs::File::create(".gitignore").expect("Failed to create gitignore file");

    let gitignore_content = String::from("venv/\n.env");
    gitignore_file
        .write_all(gitignore_content.as_bytes())
        .expect("Failed to write to gitignore file");

    std::fs::File::create("requirements.txt").expect("Failed to create requirements file");

    std::fs::create_dir("src").expect("Failed to create src directory");

    let mut main_file = std::fs::File::create("src/main.py").expect("Failed to create main file");
    let main_file_content = r#"
# Imports here


# Code here
def main():
    print("Hello World")

if __name__ == "__main__":
    main()
    "#;
    main_file
        .write_all(main_file_content.as_bytes())
        .expect("Failed to write to main file");
}
