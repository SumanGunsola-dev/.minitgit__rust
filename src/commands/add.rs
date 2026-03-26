use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process;

// now need to refactor this entair add function
pub fn add() {
    let file = ".minigit/objects";
    let path = String::from("File.txt");
    let content = match read_file(&path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("read_file fn error {e}");
            std::process::exit(1);
        }
    };
    let size = content.len();
    let header = format!("blob {}\0", size);
    let mut store: Vec<u8> = Vec::new();
    store.extend(header.as_bytes());
    store.extend(content);
    let hash: [u8; 20] = hash_fn(&store);
    let compress = compress_hash(&store);

    let hex_format = hex::encode(hash);
    let path = Path::new(file).join(&hex_format[..2]);
    match fs::create_dir(&path) {
        Ok(_) => {
            println!("Created : {} ", path.display())
        }
        Err(e) => {
            eprintln!("Error creating folder : {} ", e);
            process::exit(1);
        }
    };

    let path = Path::new(file)
        .join(&hex_format[..2])
        .join(&hex_format[2..]);
    let mut new_file = match File::create(&path) {
        Ok(file) => {
            println!("Create file named new file {}", path.display());
            file
        }
        Err(e) => {
            eprintln!("Error creating HEAD: {} ", e);
            process::exit(1);
        }
    };

    new_file
        .write_all(&compress)
        .expect("Failed to write to HEAD");
    println!("hash {:?}", hash);
}

fn read_file(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let content = fs::read(path)?;
    Ok(content)
}

// hashing function takes content as Vec<u8> and returns a 20-byte array representing the SHA-1 hash
fn hash_fn(content: &Vec<u8>) -> [u8; 20] {
    let mut hasher = Sha1::new();
    hasher.update(content);
    let result = hasher.finalize();
    println!("hash_fn fn SHA-1 hash output  {:x}", result);
    result.into()
}

fn compress_hash(hash: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(hash).unwrap();
    let compressed = encoder.finish().unwrap();
    compressed
}
