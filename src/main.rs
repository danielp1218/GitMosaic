use std::io;
use std::io::Write;

mod utils {
    pub mod image;
    pub mod git;
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

    let repo_name = get_input("Enter the repository name to create: ");
    let local_path = get_input("Enter the local path to create the repository in: ");
    let remote_url = get_input("Enter the remote URL (leave blank to create a new GitHub repo): ");
    utils::git::setup_git_repo(&repo_name, &local_path);


}
