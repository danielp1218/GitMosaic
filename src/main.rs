use std::io;
use std::io::Write;

mod utils {
    pub mod image;
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Flush failed!");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    //let repo_path = get_input("Enter the path to the repository: ");
    let image_path = get_input("Enter the path to the image file: ");

    utils::image::process_image(&image_path);
}
