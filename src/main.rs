use std::fs;
use std::env;
use std::path::{Path, PathBuf};
use chacha20poly1305::Nonce;
use colored::*;
use encrypt_rust::encryption::encrypt_chacha;
use encrypt_rust::encryption::decrypt_chacha;

pub struct DirInfo { // objeto con dos listas, una para los ficheros vistos y otra con los directorios 
    pub files: Vec<PathBuf>,
    pub dirs: Vec<PathBuf>
}

impl DirInfo { 
    pub fn new() -> Self { // constructor para iniciar vacío
        DirInfo { 
            files: Vec::new(), 
            dirs: Vec::new() 
        }
    }

    pub fn explore(&mut self, path: &Path) {

        if !path.is_dir() { // comprueba que sea una directorio válido -> cambiar a pattern matching
            eprintln!("{:?} no es un directorio válido", path);
            return;
        }

        self.dirs.push(path.to_path_buf()); // guarda el directorio en la lista 

        let entries = match fs::read_dir(path) { // leer las entradas de directorio
            Ok(entries) => entries,
            Err(e) => {
                eprintln!("Error leyendo {:?}: {}", path, e);
                return;
            }       
        };

        for entry in entries {
            match entry {
                Ok(entry) => {
                    let entry_path = entry.path();

                    if entry_path.is_file() {
                        self.files.push(entry_path);

                    } else if entry_path.is_dir() {
                        self.explore(&entry_path); 
                        // llamada recursiva
                    }         
                }
                Err(e) => eprintln!("Error leyendo entrada en {:?}: {}", path, e),
            }
        }
    }
}

pub fn encrypt_files(dir_info: &DirInfo, key: &[u8; 32], nonce: &[u8; 12]) {

    for file in &dir_info.files {
        let file_data = fs::read(file).expect("Failed to read file");
        let encrypted_data = encrypt_chacha(&file_data, key, nonce);

        fs::write(file, &encrypted_data).expect("Failed to write encrypted file");
        
        let new_name = file.with_extension("encrypted");
        fs::rename(file, &new_name).expect("Failed to add the extension");
    }
}

pub fn decrypt_files(dir_info: &DirInfo, key: &[u8; 32], nonce: &[u8; 12]) {

    for file in &dir_info.files {
        let file_data = fs::read(file).expect("Failed to read file");
        let decrypted_data = decrypt_chacha(&file_data, key, nonce);

        fs::write(file, &decrypted_data).expect("Failed to write decrypted file");
        
        let new_name = file.with_extension("");
        fs::rename(file, &new_name).expect("Failed to delete extension");
    }
}

fn main() {

    // añadir variables de cifrado y descifrado key: &[u8; 32], nonce: &[u8; 12]

    let args: Vec<String> = env::args().collect();
    let path_request = &args[1];
    println!("🚩 Encryping data from {path_request} 🚩\n");

    let mut info = DirInfo::new();
    info.explore(Path::new(path_request));

    println!("Directorios encontrados: ");
    for d in &info.dirs {
        println!("{}/", format!("{}", d.display()).blue());

    }
    println!("Ficheros encontrados: ");
    for f in &info.files {
       println!("{}/", format!("{}", f.display()).yellow());
    }
}
