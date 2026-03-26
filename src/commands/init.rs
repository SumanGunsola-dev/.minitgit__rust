use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{fs, process};
pub fn init() {
    let root = Path::new(".minigit");
    let folders = vec![
        "branches",
        "objects",
        "objects/info",
        "objects/pack",
        "refs",
        "refs/heads",
        "refs/tags",
    ];

    // .minigit required folders creation in .minigit
    for folder in folders {
        let path = Path::new(root).join(folder);

        match fs::create_dir_all(&path) {
            Ok(_) => {
                println!("Created : {} ", path.display())
            }
            Err(e) => {
                eprintln!("Error creating folder : {} ", e);
                process::exit(1);
            }
        }
    }

    // HEAD file creation handling and writing in .minigit
    let head_path = Path::new(root).join("HEAD");
    let mut head_file = match File::create(&head_path) {
        Ok(file) => {
            println!("Create file named HEAD {}", head_path.display());
            file
        }
        Err(e) => {
            eprintln!("Error creating HEAD: {} ", e);
            process::exit(1);
        }
    };
    head_file
        .write_all(b"ref: refs/heads/master\n")
        .expect("Failed to write to HEAD");

    // Description file creation handling and writing in .minigit
    let des_path = Path::new(root).join("description");
    let mut des_file = match File::create(&des_path) {
        Ok(file) => {
            println!("Create file named description {}", des_path.display());
            file
        }
        Err(e) => {
            eprintln!("Error creating description : {} ", e);
            process::exit(1);
        }
    };
    des_file
        .write_all(b"Unnamed repository; edit this file 'description' to name the repository.\n")
        .expect("Failed to write to description");

    // Config file creation handling and writing in .minigit
    let con_path = Path::new(root).join("config");
    let mut con_file = match File::create(&con_path) {
        Ok(file) => {
            println!("Create file named config {}", con_path.display());
            file
        }
        Err(e) => {
            eprintln!("Error creating config : {} ", e);
            process::exit(1);
        }
    };

    con_file
        .write_all(
            b"[core]\n\trepositoryformatversion = 0\n\tfilemode = true\n\tbare = false\n\tlogallrefupdates = true\n",
        )
        .expect("Failed to write to config");
}
