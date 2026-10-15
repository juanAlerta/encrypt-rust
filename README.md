# encrypt-rust
Rust bin to encrypt and decrypt files based on their types (in progress).

## Features
- Encrypts files in a directory and its subdirectories.
- Renames encrypted files with a `.encrypted` extension.
- Decrypts files and renames them with a `.decrypted` extension.
- Lists files and directories.

## Usage
To compile the project, run:
```bash
cargo build --release
```

### Encryption
To encrypt files:
```bash
./target/release/encrypt-rust /path/to/directory encrypt [key]
```
- `/path/to/directory`: The directory containing files to be encrypted.
- `[key]`: Optional hexadecimal key. If not provided, a random key is generated and printed.

### Decryption
To decrypt files:
```bash
./target/release/encrypt-rust /path/to/directory decrypt [key]
```
- `/path/to/directory`: The directory containing files to be decrypted.
- `[key]`: Optional hexadecimal key. If not provided, the key must match the encryption key used.

### List Files and Directories
To list files and directories:
```bash
./target/release/encrypt-rust /path/to/directory list
```
- `/path/to/directory`: The directory to be listed.

### Next Steps
1. Configurar método de envío de clave y nonce. 
    1.1 Pensar usar una sola clave general porque una por fichero es poco realista?
    1.2 Servidor privado. Complicado envío seguro? 

2. Recibir claves para descifrado
    2.1 Probablemente pasando un fichero como argumento en CLI.
    


## Dependencies
- `aes`: For AES encryption.
- `chacha20poly1305`: For ChaCha20Poly1305 encryption.
- `rand`: For generating random keys.
- `colored`: For colored output in the console.