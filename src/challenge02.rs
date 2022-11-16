async fn fetch_encrypted() -> reqwest::Result<String> {
    let file = reqwest::get("https://codember.dev/encrypted.txt")
        .await?
        .text()
        .await?;
    Ok(file)
}

fn decrypt_word(mut encrypted: String) -> String {
    let mut decrypted = "".to_string();

    while !encrypted.is_empty() {
        let is_two_digits_a_lowercase_char = encrypted.get(0..2).unwrap().parse::<u8>().unwrap() >= 97;
        let split_at_value = match is_two_digits_a_lowercase_char {
            true => 2,
            false => 3,
        };
        let (first, last) = encrypted.split_at(split_at_value);

        decrypted.push(first.parse::<u8>().unwrap() as char);
        encrypted = last.to_string();
    }

    decrypted
}

fn decrypt(content: String) -> String {
    content
        .split(' ')
        .map(|word| decrypt_word(word.to_string()))
        .collect::<Vec<String>>()
        .join(" ")
}

pub async fn result() -> reqwest::Result<()> {
    let message = decrypt(fetch_encrypted().await?);

    println!("challenge02");
    println!("{message}");
    Ok(())
}
