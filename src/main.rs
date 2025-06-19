use std::io::{self, prelude::*, BufWriter}; // std::io::prelude::* => add traits like: Read, Write and BufRead, its purpose is just to import these modules mentioned
use std::fs::{self, File};
use std::path::Path;

fn main() {

    let mut input: String = String::new();

    println!("Welcome to Ludurn's explorer!\n\nType:\n1 => to enter into the file explorer\n2 => to enter into the file editor\n");
    io::stdin().read_line(&mut input).unwrap();

    let switch: u8 = input.trim().parse::<u8>().unwrap_or(0);

    if switch == 1 {
        call_file_explorer();
    } else if switch == 2 {
        call_file_editor();
    }

}

fn call_file_explorer() -> () {

    let mut input: String = String::new();

    println!("How to navegate:\nType \"cd\" <name> to access directories\nType the file name to display the action menu");
    let mut exit: bool = false;
    search_dir(&"c:".to_string());

    while exit == false {
        input.clear();
        
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() != String::from("exit") {

            if &input[..2] == "cd" {


            }

            search_dir(&input);

            continue;
        }

        exit = true;
    }

    

}

fn search_dir(input_path: &String) -> () {

    println!("\n");
    let path: &Path = Path::new(input_path);

    match path.read_dir() {

        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("{}", entry.path().to_string_lossy().replace(input_path, ""));
                }
            }
            println!("\nType \"exit\" to leave the explorer");
        }
        Err(_e) => {
            println!("Failed to read directory: file name, directory name or volume label syntax is incorrect.");
        }

    }



}

fn call_file_editor() -> () {

    let mut input: String = String::new();

    println!("Insert 1 to write or 2 to read:");
    io::stdin().read_line(&mut input).unwrap();

    let choice: u8 = input.trim().parse::<u8>().unwrap_or(0);
    input.clear();

    if choice == 1 {
        println!("Insert the text you want to write:");
        io::stdin().read_line(&mut input).unwrap();

        write_txt(&input);
        input.clear();

    } else if choice == 2 {
        println!("Insert the file name you want to read:");
        io::stdin().read_line(&mut input).unwrap();
        let file_name: &str = input.trim();

        match read_txt(file_name) {
            Ok(content) => println!("{}", content),
            Err(e) => eprintln!("Failed to read file: {}", e)
        }
        input.clear();

    }

}

fn write_txt(input: &str) -> () {

    let text_f: File = File::create("teste.txt").expect("Failed to create file");

    let mut writer: BufWriter<File> = BufWriter::new(text_f);

    writer.write(input.as_bytes()).expect("Failed to write");
    writer.flush().expect("Failed to flush");

}

fn read_txt(file_name: &str) -> Result<String, io::Error> {

    let mut path: String = String::from("E:/github_repos/rust_proj/");
    path.push_str(file_name);

    let txt: String = fs::read_to_string(path)?;

    Ok(txt)
}
