# encrypt-rust
Rust bin to encrypt and decrypt files based on their types (in progress).

## Features
- Encrypts files in a directory and its subdirectories.
- Renames encrypted files with a `.encrypted` extension.
- Decrypts files and renames them with a `.decrypted` extension.

## Usage
To compile the project, run:

## Features
- Encrypts files in a directory and its subdirectories.
- Renames encrypted files with a `.encrypted` extension.
- Decrypts files and renames them with a `.decrypted` extension.

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

### Next Steps
1. **Add Support for Different Encryption Algorithms**: Implement decryption for different encryption algorithms (e.g., Chacha20Poly1305).
2. **User Interface**: Create a simple command-line interface for easier use.
3. **Error Handling**: Improve error handling to provide more informative messages.
4. **Logging**: Add logging for better debugging and monitoring.

## Dependencies
- `aes`: For AES encryption.
- `chacha20poly1305`: For ChaCha20Poly1305 encryption.
- `rand`: For generating random keys.
- `colored`: For colored output in the console.