# PIN Block Generation and Decryption

This project implements a PIN block generation and decryption system using 3DES encryption in ECB mode for securing PINs used in card transactions. It includes utilities for generating a PIN block from a PIN and card number, encrypting this block, decrypting it back, and extracting the original PIN from the decrypted block.

## Overview

The PIN block is generated according to ISO 9564 format 0, where:
- The first two digits represent the length of the PIN.
- The following digits are the PIN itself.
- The rest is padded with 'F' characters to make it 16 hexadecimal characters long.

This block is then XORed with a portion of the card number (PAN) for added security. The result is encrypted using 3DES for storage or transmission, and later decrypted to extract the original PIN.

## Prerequisites

- Rust (stable version)
- OpenSSL library (for 3DES encryption/decryption)

Make sure you have OpenSSL installed on your system, and you might need to set up the environment variable `OPENSSL_DIR` if you're on Windows.

## Usage

You can use the functions in your Rust project by including them in your `Cargo.toml`:

```toml
[dependencies]
openssl = "0.10"
hex = "0.4"
```

# Functions
- generate_pin_block
Purpose: Generates a PIN block from a given PIN and card number.
Parameters:
pin: &str - The PIN to encode.
card_number: &str - The card number used to XOR with the PIN block.
Returns: Result<String, &'static str> - Encoded PIN block in hex format.

- encrypt_data
Purpose: Encrypts data using 3DES in ECB mode.
Parameters:
key: &str - The encryption key (must be 24 bytes).
data: &str - The data to encrypt (already in hex).
Returns: Result<String, Box<dyn Error>> - Encrypted data in uppercase hex.

- decrypt_data
Purpose: Decrypts data that was encrypted with encrypt_data.
Parameters:
key: &str - The decryption key (must match the encryption key).
data: &str - The data to decrypt (in hex).
Returns: Result<String, Box<dyn Error>> - Decrypted data in uppercase hex.

- extract_pin_from_block
Purpose: Extracts the original PIN from a decrypted PIN block.
Parameters:
encrypted_pin: &str - The decrypted PIN block in hex.
card_number: &str - The card number used to XOR back the PIN.
Returns: Result<String, &'static str> - The extracted PIN.

# Installation
To use this code in your project:

```bash
sh
cargo add openssl hex
```
# Examples
Here's an example of how to use these functions:

```rust
use pin_utils::{generate_pin_block, encrypt_data, decrypt_data, extract_pin_from_block};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pin = "1234";
    let card_number = "1234567890123456";
    let encryption_key = "0123456789ABCDEF0123456789ABCDEF"; // Example 24-byte key for 3DES

    let pin_block = generate_pin_block(pin, card_number)?;
    println!("Generated PIN Block: {}", pin_block);

    let encrypted_block = encrypt_data(encryption_key, &pin_block)?;
    println!("Encrypted PIN Block: {}", encrypted_block);

    let decrypted_block = decrypt_data(encryption_key, &encrypted_block)?;
    println!("Decrypted PIN Block: {}", decrypted_block);

    let extracted_pin = extract_pin_from_block(&decrypted_block, card_number)?;
    println!("Extracted PIN: {}", extracted_pin);

    Ok(())
}
```
# Contributing
Contributions are welcome! If you find a bug or want to propose a feature, please open an issue or submit a pull request.
