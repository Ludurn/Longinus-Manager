
struct SearchInfo {
    previous_search: String
}

impl SearchInfo {

    fn new(previous_search: String) -> SearchInfo {
        SearchInfo { previous_search }
    }

    fn get_search_info(&self) -> String {
        self.previous_search.clone()
    }
}

pub mod file_explorer {

    use std::io;
    use std::path::Path;

    use crate::flmg::SearchInfo;
    
    pub fn call_file_explorer() -> () {

        let mut input: String = String::new();

        println!("How to navegate:\nType \"cd\" <name> to access directories\nType the file name to display the action menu");

        let mut exit: bool = false;
        let directory: SearchInfo = SearchInfo::new("c:".to_string());

        search_dir(&directory.get_search_info());

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


}


pub mod file_editor {

    // std::io::prelude::* => add traits like: Read, Write and BufRead, its purpose is just to import these modules mentioned
    use std::io::{self, prelude::*, BufWriter}; 
    use std::fs::{self, File};
    use std::path::PathBuf;
    use std::env;

    pub fn call_file_editor() -> () {

        let mut input: String = String::new();

        println!("\nInsert:\n1 => to write\n2 => to read\nq => to quit\n");

        loop {

            input.clear();

            io::stdin().read_line(&mut input).unwrap();

            match input.trim() {
                "1" => {
                    input.clear();

                    println!("Insert the text you want to write:");
                    io::stdin().read_line(&mut input).unwrap();

                    write_txt(&input);

                    break;
                }
                "2" => {
                    input.clear();

                    println!("Insert the file name you want to read:");
                    io::stdin().read_line(&mut input).unwrap();

                    let file_name: &str = input.trim();

                    match read_txt(file_name) {
                        Ok(content) => println!("{}", content),
                        Err(e) => eprintln!("Failed to read file: {}", e)
                    }

                    break;
                }
                "q" => {
                    println!("\nGoodbye!");
                    break;
                }
                _ => {
                    println!("\nInvalid option. Please try again.");
                    continue;
                }

            }


        }

    }

    fn write_txt(input: &str) -> () {

        let text_f: File = File::create("teste.txt").expect("Failed to create file");

        let mut writer: BufWriter<File> = BufWriter::new(text_f);

        writer.write(input.as_bytes()).expect("Failed to write");
        writer.flush().expect("Failed to flush");

    }

    fn read_txt(file_name: &str) -> Result<String, io::Error> {

        let mut path: PathBuf = env::current_exe()?;
        path.pop(); // Removes the exec name
        path.push(file_name);

        

        let txt: String = fs::read_to_string(path)?;

        Ok(txt)
    }
    
}

