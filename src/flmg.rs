use std::{rc::Rc, cell::RefCell};

#[derive(Default)]

struct SearchInfo {
    previous_search: String
}

impl SearchInfo {

    fn previous_search(&self) -> &str {
        &self.previous_search
    }

    fn set_previous_search(&mut self, search: String) {
        self.previous_search = search;
    }
}

#[derive(Default)]

struct FileName {
    file_name: String
}

impl FileName {

    fn file_name(&self) -> &str {
        &self.file_name
    }

    fn set_file_name(&mut self, name: String) {
        self.file_name = name;
    }
}

pub mod file_explorer {

    use super::*; // super::* => import everthing that's pub available, in this case the struct and impl
    use std::io;
    use std::path::Path;
    
    pub fn call_file_explorer() {

        let mut input: String = String::new();
        let srch_info = Rc::new(RefCell::new(SearchInfo::default()));

        println!("\nHow to navigate:\nType \"cd\" <name> to access directories\nType \"dir\" to display the elements of the current directory\nType \"q\" to quit\n");

        let mut exit: bool = false;

        {
            let mut cntrl_srch_info = srch_info.borrow_mut();
            cntrl_srch_info.set_previous_search(String::from("c:"));
        }
        let default_path = srch_info.borrow().previous_search().to_string();
        search_dir(srch_info.clone(), &default_path);

        while exit == false {
            input.clear();
            
            io::stdin().read_line(&mut input).unwrap();

            if input.trim() != String::from("q") {

                let prev_search: String = srch_info.borrow().previous_search().to_string();


                if input[..2].trim() == "cd" {

                    let input_path: String = format!("{}/{}", prev_search, input[3..].trim());
                    let mut adjusted_path: &str = &input_path;

                    if input_path.ends_with("..") {

                        let mut slash_count: u8 = 0;
                        let mut cut_off = 0;

                        for (i, c) in input_path.trim().char_indices().rev() {
                            if matches!(c, '/') {
                                slash_count += 1;

                                if slash_count == 2 {
                                    cut_off = i;
                                    break;
                                }
                            }
                        }

                        if slash_count >= 2 {
                            adjusted_path = &input_path[..cut_off]
                        }
                    } else if input_path.ends_with('.') {

                        adjusted_path = &input_path[0..input_path.char_indices().count()-2];
                    }
       
                    search_dir(srch_info.clone(), &adjusted_path);

                }

                if input[..3].trim() == "dir" {

                    search_dir(srch_info.clone(), &prev_search);

                }

                continue;
            }

            exit = true;
        }

    }


    fn search_dir(srch_info: Rc<RefCell<SearchInfo>>, input_path: &str) {
        
        let path: &Path = Path::new(input_path);
        

        match path.read_dir() {

            Ok(entries) => {

                println!("\n");

                {
                    let mut cntrl_srch_info = srch_info.borrow_mut();
                    cntrl_srch_info.set_previous_search(path.to_string_lossy().to_string());
                }

                for entry in entries {
                    if let Ok(entry) = entry {

                        let fl_tag = match entry.file_type() {

                            Ok(file_type) if file_type.is_dir() => "<DIR>",
                            _ => "     ",

                        };

                        let mut fl_name: &str = &entry.path().to_string_lossy().replace(input_path, "");

                        if fl_name.starts_with('\\') {

                            fl_name = &fl_name[1..];
                        }

                        println!(
                            "{}  -  {}",
                            fl_tag,
                            fl_name
                        );
                    }
                }

                println!("\nType \"q\" to quit the explorer");
                println!("\nYou're in: {}", srch_info.borrow().previous_search());
            }
            Err(_e) => {
                println!("\nFailed to read directory: file name, directory name or volume label syntax is incorrect.");
            }

        }

    }


}


pub mod file_editor {

    use super::*;
    // std::io::prelude::* => add traits like: Read, Write and BufRead, its purpose is just to import these modules mentioned
    use std::io::{self, prelude::*, BufWriter}; 
    use std::fs::{self, File};
    use std::path::{Path, PathBuf};
    use std::env;

    pub fn call_file_editor() -> () {

        let mut input: String = String::new();
        let fl_name = Rc::new(RefCell::new(FileName::default()));

        println!("\nType the file path to access the action menu or \"q\" to quit");

        let mut exit: bool = false;

        loop {

            if input.trim() != String::from("q") {
                input.clear();

                io::stdin().read_line(&mut input).unwrap();

                // Converts the input to a proper path
                let path: &Path = Path::new(input.trim());

                match file_exists(path) {
                    Ok(_c) => break,
                    Err(e) => {
                        println!("Failed to access file: {}", e);
                        continue;
                    }
                }
            }

            break exit = true;
        }



        loop {

            if (exit) {
                break;
            }

            input.clear();

            println!("\nInsert:\n1 => to write\n2 => to read\nq => to quit\n");

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

    fn file_exists(path: &Path) -> Result<&str, &str> {

        if !path.exists() {
            return Err("Path does not exist");
        }

        //canonicalize() => normalizes the path
        let absolute_path = match path.canonicalize() {
            Ok(p) => p,
            Err(_) => return Err("Path absolute unresolved")
        };

        if !path.is_file() {
            return Err("Path is not a file");
        }

        match File::open(&absolute_path) {
            Ok(_) => Ok("File is accessible"),
            Err(_) => Err("File did not open")
        }
        
    }
    
}

