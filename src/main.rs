use inquire::Password;
use sha2::Digest;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

fn main() {
    let to_encode: String = Password::new("")
        .without_confirmation()
        .prompt()
        .unwrap();

    let hash_hex = calculate_sha256(&to_encode);
    copy_to_clipboard(&hash_hex);
}

fn calculate_sha256(input: &str) -> String {
    let mut hasher = sha2::Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();

    let hash_hex = result.iter()
    .map(|byte| format!("{:02x}", byte))
    .collect::<String>();

    hash_hex
}

fn copy_to_clipboard(text: &str) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(text.to_owned()).unwrap();
}