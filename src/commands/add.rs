use flate2::Compression;
use flate2::write::ZlibEncoder;
// use flate2::write::{GzEncoder, ZlibEncoder};
use sha1::{Digest, Sha1};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::process;

// now need to refactor this entair add function
pub fn add(file_path: &str) {
    let root = ".minigit/objects";
    // let file_path = String::from("file.txt");
    let index = "index";

    let content = match read_file(&file_path) {
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
    store.extend_from_slice(&content);
    let hash: [u8; 20] = hash_fn(&store);
    let compress = compress_object_zlib(&store);

    let hex_format = hex::encode(hash);
    let path = Path::new(root).join(&hex_format[..2]);
    match fs::create_dir_all(&path) {
        Ok(_) => {
            println!("Created : {} ", path.display())
        }
        Err(e) => {
            eprintln!("Error creating folder : {} ", e);
            process::exit(1);
        }
    };

    let path = Path::new(root)
        .join(&hex_format[..2])
        .join(&hex_format[2..]);

    if path.exists() {
        println!("Object already exists, skipping...");
    } else {
        let mut index_file = match File::create(&path) {
            Ok(file) => {
                println!("Create file named new file {}", path.display());
                file
            }
            Err(e) => {
                eprintln!("Error creating : {} ", e);
                process::exit(1);
            }
        };

        index_file
            .write_all(&compress)
            .expect("Failed to write to File in fn add --- ");
        println!("hash {:?}", hash);
    }
    create_index(index, hash, &file_path);
}

fn create_index(path: &str, hash: [u8; 20], file_path: &str) {
    const SIGNATURE: [u8; 4] = *b"DIRC";
    const ROOT: &str = ".minigit";
    const VERSION: u32 = 2;
    const NULL: u8 = 0x00;

    let index_path = Path::new(ROOT).join(path);

    let meta = fs::metadata(file_path).unwrap();
    let entry_count: u32 = 1;

    // Metadata
    let ctime_sec = meta.ctime() as u32;
    let ctime_nsec = meta.ctime_nsec() as u32;
    let mtime_sec = meta.mtime() as u32;
    let mtime_nsec = meta.mtime_nsec() as u32;
    let dev = meta.dev() as u32;
    let ino = meta.ino() as u32;
    let mode = meta.mode();
    let uid = meta.uid();
    let gid = meta.gid();
    let file_size = meta.size() as u32;

    let path_len = file_path.as_bytes().len();
    let flags: u16 = (path_len as u16) & 0x0FFF;

    let padding = padding_cal(path_len);

    let mut buffer: Vec<u8> = Vec::new();

    // Header
    buffer.extend(&SIGNATURE);
    buffer.extend(&VERSION.to_be_bytes());
    buffer.extend(&entry_count.to_be_bytes());

    // Entry
    buffer.extend(&ctime_sec.to_be_bytes());
    buffer.extend(&ctime_nsec.to_be_bytes());
    buffer.extend(&mtime_sec.to_be_bytes());
    buffer.extend(&mtime_nsec.to_be_bytes());
    buffer.extend(&dev.to_be_bytes());
    buffer.extend(&ino.to_be_bytes());
    buffer.extend(&mode.to_be_bytes());
    buffer.extend(&uid.to_be_bytes());
    buffer.extend(&gid.to_be_bytes());
    buffer.extend(&file_size.to_be_bytes());

    // SHA-1
    buffer.extend(&hash);

    // Flags
    buffer.extend(&flags.to_be_bytes());

    // Path + NULL
    buffer.extend(file_path.as_bytes());
    buffer.push(NULL);

    // Padding
    buffer.extend(&padding);

    // CHECKSUM
    let mut hasher = Sha1::new();
    hasher.update(&buffer);
    let checksum = hasher.finalize();

    buffer.extend(&checksum);

    // write to file
    let mut file = File::create(&index_path).unwrap();
    file.write_all(&buffer).unwrap();

    println!("Index written with checksum ✅");
}

fn padding_cal(path_len: usize) -> Vec<u8> {
    let entry_len = 62 + path_len + 1; // +1 for NULL

    let pad = (8 - (entry_len % 8)) % 8;

    vec![0u8; pad]
}
fn read_file(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let content = fs::read(path)?;
    Ok(content)
}

// hashing function takes content as Vec<u8> and returns a 20-byte array representing the SHA-1 hash
fn hash_fn(content: &[u8]) -> [u8; 20] {
    let mut hasher = Sha1::new();
    hasher.update(content);
    let result = hasher.finalize();
    println!("hash_fn fn SHA-1 hash output  {:x}", result);
    result.into()
}

fn compress_object_zlib(hash: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(hash).unwrap();
    let compressed = encoder.finish().unwrap();
    compressed
}
