use hex::{decode, encode};
use openssl::symm::{Cipher, Crypter, Mode};
use std::error::Error;

pub(crate) fn encrypt_data(key: &str, data: &str) -> Result<String, Box<dyn Error>> {
    let key_bytes = decode(key)?;
    let data_bytes = decode(data)?;

    if key_bytes.len() != 24 {
        return Err("Invalid key length. 3DES requires a 24-byte key.".into());
    }

    let cipher = Cipher::des_ede3_ecb();
    let mut crypter = Crypter::new(cipher, Mode::Encrypt, &key_bytes, None)?;
    crypter.pad(false);

    let mut encrypted = vec![0; data_bytes.len() + cipher.block_size()];
    let count = crypter.update(&data_bytes, &mut encrypted)?;
    let rest = crypter.finalize(&mut encrypted[count..])?;

    encrypted.truncate(count + rest);
    Ok(encode(&encrypted).to_uppercase())
}

pub(crate) fn decrypt_data(key: &str, data: &str) -> Result<String, Box<dyn Error>> {
    let key_bytes = decode(key)?;
    let data_bytes = decode(data)?;

    if key_bytes.len() != 24 {
        return Err("Invalid key length. 3DES requires a 24-byte key.".into());
    }

    let cipher = Cipher::des_ede3_ecb();
    let mut crypter = Crypter::new(cipher, Mode::Decrypt, &key_bytes, None)?;
    crypter.pad(false);

    let mut decrypted = vec![0; data_bytes.len() + cipher.block_size()];
    let count = crypter.update(&data_bytes, &mut decrypted)?;
    let rest = crypter.finalize(&mut decrypted[count..])?;

    decrypted.truncate(count + rest);
    Ok(encode(&decrypted).to_uppercase())
}

fn xor_hex(a: &str, b: &str) -> Result<String, &'static str> {
    if a.len() != b.len() {
        return Err("Inputs must have the same length");
    }

    let a_bytes = decode(a).map_err(|_| "Invalid hex string for 'a'")?;
    let b_bytes = decode(b).map_err(|_| "Invalid hex string for 'b'")?;

    let result: Vec<u8> = a_bytes.iter().zip(b_bytes.iter()).map(|(&a, &b)| a ^ b).collect();
    Ok(encode(&result).to_uppercase())
}

pub(crate) fn generate_pin_block(pin: &str, card_number: &str) -> Result<String, &'static str> {
    if pin.len() < 4 || pin.len() > 6 {
        return Err("Invalid PIN length");
    }
    let pin_length = format!("{:02}", pin.len());
    let pin_block = format!("{}{}{}", pin_length, pin, "F".repeat(16 - pin_length.len() - pin.len()));

    if card_number.len() < 13 {
        return Err("Invalid card number length");
    }

    let pan = format!(
        "0000{}",
        &card_number[card_number.len() - 12..]
    );
    xor_hex(&pin_block, &pan)
}

pub(crate) fn extract_pin_from_block(
    encrypted_pin: &str,
    card_number: &str,
) -> Result<String, &'static str> {
    if encrypted_pin.len() != 16 {
        return Err("Invalid encrypted PIN block length");
    }
    if card_number.len() < 13 {
        return Err("Invalid card number length");
    }

    let pan = format!("0000{}", &card_number[card_number.len() - 12..]);
    println!("PAN used for extraction: {}", pan);
    println!("Encrypted PIN for extraction: {}", encrypted_pin);

    let decrypted_pin_block = xor_hex(&pan, encrypted_pin)?;
    println!("Decrypted PIN block after XOR: {}", decrypted_pin_block);


    //Todo not sure why the pin length is in second position
    //fixme this is not correct
    // Parse the PIN length
    let pin_length = decrypted_pin_block
        .chars()
        .nth(1)
        .and_then(|c| c.to_digit(10))
        .ok_or("Invalid PIN length character")? as usize;

    if pin_length == 0 || pin_length > 6 || 1 + pin_length > decrypted_pin_block.len() {
        return Err("PIN length out of expected range");
    }
    let pin = &decrypted_pin_block[2..=pin_length+1];
    println!("Extracted PIN characters: {}", pin);

    Ok(pin.to_string())
}