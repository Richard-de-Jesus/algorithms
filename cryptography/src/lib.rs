
// auxiliar function, converts key to bytes
// and check if key and text are ascii
fn _vigenere_aux(key: &str, text: &str) -> Option<Vec<u8>> {

    if !key.is_ascii() || !text.is_ascii() {
        return None;
    }
    unsafe {
    let mut temp = key.
        to_uppercase().as_bytes_mut().to_owned();

    // converting to 0..=25
    for i in 0..key.len() {
        match  temp[i] {
             b'A'..=b'Z' => temp[i] -= b'A',
            _ => temp[i] = 0,
        }
    }
    return Some(temp);
    }
}

pub fn vigenere_encrypt(text: &str, key: &str) -> Option<String> {
    let key = _vigenere_aux(key, text)?;

    let mut result = String::from("");
    let mut value: u8;
 

    let mut j = 0;
    for ch in text.bytes() {
        // if the key is shorter than the message
        // loop back to first letter.
        if j == key.len() {
            j = 0;
        }

        value = ch + key[j];
        match ch {
            b'A'..=b'Z' => {
                if value > b'Z' {
                    value -= 26;
                }
                j += 1;
            },
            b'a'..=b'z' => {
                if value > b'z' {
                    value -= 26;
                }
                j += 1;
            },
            _ => value -= key[j],
        }
        result.push(value as char); 
    }
    Some(result)
}

pub fn vigenere_decrypt(text: &str, key: &str) -> Option<String> {
    let key = _vigenere_aux(key, text)?;

    let mut result = String::from("");
    let mut value: u8;

    let mut j = 0;
    for ch in text.bytes() {
        // if the key is shorter than the message
        // loop back to first letter.
        if j == key.len() {
            j = 0;
        }

        value = ch - key[j];
        match ch {
            b'A'..=b'Z' => {
                if value < b'A' {
                    value += 26;
                }
                j += 1;
            },
            b'a'..=b'z' => {
                if value < b'a' {
                    value += 26;
                }
                j += 1;
            },
            _ => value += key[j],
        }
        result.push(value as char); 
    }
    Some(result)
}

pub fn caesar_encrypt(text: &str, key: u8) -> Option<String>{
    
    if !text.is_ascii() || key > 25 {
        return None;
    }

    let mut result = String::from("");
    let mut value: u8;

    for ch in text.bytes() {
        
        value = ch + key;
        match ch {
            b'A'..=b'Z' => {
                
                if value > b'Z' {
                    value -= 26;
                }
            },
            b'a'..=b'z' => {

                if value > b'z' {
                    value -= 26;
                }
            },
            _ => value -= key,
        }
        result.push(value as char);
    }
    Some(result)
}

pub fn caesar_decrypt(text: &str, key: u8) -> Option<String> {

    if !text.is_ascii() || key > 25 {
        return None;
    }

    let mut result = String::from("");
    let mut value: u8;

    for ch in text.bytes() {

        value = ch - key;
        match ch {
            b'A'..=b'Z' => {
                
                if value < b'A' {
                    value += 26;
                }
            },
            b'a'..=b'z' => {

                if value < b'a' {
                    value += 26;
                }
            },
            _ => value += key,
        }
        result.push(value as char);
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caesar_cypher() {
        let text = "abc ABC, xyz XYZ 01";
        let expected = "bcd BCD, yza YZA 01";

        let encrypted = caesar_encrypt(&text, 1)
            .unwrap();
        let decrypted = caesar_decrypt(&encrypted, 1)
            .unwrap();

        assert_eq!(expected, encrypted);
        assert_eq!(text, decrypted);
    }
    #[test]
    fn vigenere_cypher() {
        let text = "abc ABC, xyz XYZ 01";
        let expected = "cjr HFT, zgo ECQ 01";

        let encrypted = vigenere_encrypt(text, "cipher")
            .unwrap();
        let decrypted = vigenere_decrypt(encrypted.as_str(), "cipher")
            .unwrap();
        
        assert_eq!(expected, encrypted);
        assert_eq!(text ,decrypted);
    }
}
