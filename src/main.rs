mod pin_utils;

fn main() {
    generate_pin_block(
        "1234567890123456",
        "1234",
        "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF",
    );
}

fn generate_pin_block(pan: &str, pin: &str, encryption_key: &str) {
    match pin_utils::generate_pin_block(pin, pan) {
        Ok(pin_block) => {
            println!("Generated PIN Block (Hex): {}", pin_block);
            match pin_utils::encrypt_data(encryption_key, &pin_block) {
                Ok(encrypted_block) => {
                    println!("Encrypted PIN Block: {}", encrypted_block);
                    match pin_utils::decrypt_data(encryption_key, &encrypted_block) {
                        Ok(decrypted_block) => {
                            println!("Decrypted PIN Block: {}", decrypted_block);
                            match pin_utils::extract_pin_from_block(&decrypted_block, pan) {
                                Ok(extracted_pin) => {
                                    println!("Extracted PIN: {}", extracted_pin);
                                }
                                Err(e) => println!("Failed to extract PIN: {}", e),
                            }
                        }
                        Err(e) => println!("Error decrypting PIN block: {}", e),
                    }
                }
                Err(e) => println!("Error encrypting PIN block: {}", e),
            }
        }
        Err(e) => println!("Error generating PIN block: {}", e),
    }
}
