use std::{io::Write, process::Command};
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    println!("{:?}", args);
    if args.len() >= 2{    
         // Create a new folder and do everything inside
        let new_dir = String::from(args[1].clone());
        println!("{}", new_dir);
        // Create the folder
        std::fs::create_dir(&new_dir).unwrap(); 
        // Change the current directory to the new one
        std::env::set_current_dir(&new_dir).unwrap();
    }
    else{
        println!("Please provide a name for the folder");
        std::process::exit(1);
    }

    let mut activate_command: String = String::from("source venv/bin/activate");
    if std::env::consts::OS == "windows" {
        activate_command = String::from("venv\\Scripts\\activate");
    }

    // Creates virual environment
    println!("Creating virtual environment");
    Command::new("python")
        .arg("-m")
        .arg("venv")
        .arg("env")
        .output()
        .expect("Failed to create virtual environment");

    println!("Successfully created the virual environment. You need to run the command {} to activate the venv", activate_command);

    println!("Initializing git repository");
    let git_init = Command::new("git")
        .arg("init")
        .output()
        .expect("Failed to initialize git");

    println!("{}", String::from_utf8_lossy(&git_init.stdout));

    println!("Creating gitignore file");
    let mut gitignore_file =
        std::fs::File::create(".gitignore").expect("Failed to create gitignore file");

    let gitignore_content = String::from("env/\n.env");
    gitignore_file
        .write_all(gitignore_content.as_bytes())
        .expect("Failed to write to gitignore file");
    println!("Gitignore successfully created");

    println!("Creating requirements file...");
    std::fs::File::create("requirements.txt").expect("Failed to create requirements file");
    println!("Successfully created requirements file");

    println!("Creating source directoryy...");
    std::fs::create_dir("src").expect("Failed to create src directory");

    println!("Creating main.py file...");
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

    println!("Successfully created main.py file");

    println!("Creating README.md file...");
    std::fs::File::create("README.md").expect("Failed to create readme file");
    
    println!("Successfully created README.md file");

    println!("Creating .env file...");
    std::fs::File::create(".env").expect("Failed to create .env file");

    println!("Successfully initialised python project");
}
