use rustyline::{DefaultEditor, error::ReadlineError};
use chrono::{DateTime, Datelike};
use indicatif::ProgressBar;

mod utils {
    pub mod image;
    pub mod git;
}

fn get_input(rl: &mut DefaultEditor, prompt: &str, validation: fn(&str) -> bool, error_message: &str) -> String {
    loop {
        let readline = rl.readline(prompt);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                if validation(&line) {
                    return line;
                } else {
                    println!("{}", error_message);
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                // terminate program
                std::process::exit(0);
            },
            Err(err) => {
                println!("Error: {:?}", err);
                std::process::exit(1);
            }
        }
    }
}


fn main() {
    //let repo_path = get_input("Enter the path to the repository: ");
    
    let mut rl = DefaultEditor::new().expect("Failed to create readline editor");

    let mut quantized_image: Vec<Vec<u8>>;

    loop {
        let image_path = get_input(&mut rl, "Enter the path to the image file: ", |input| {
            utils::image::valid_image_path(input)
        }, "Invalid image path. Please enter a valid path.");
        quantized_image = utils::image::process_image(&image_path);
        let continue_input = get_input(&mut rl, "Do you want to proceed with this image? (y/n): ", |input| {
            input.eq_ignore_ascii_case("y") || input.eq_ignore_ascii_case("n")
        }, "Invalid input. Please enter 'y' or 'n'.");

        if continue_input.eq_ignore_ascii_case("y") {
            break;
        }
    }


    let repo_name = get_input(&mut rl, "Enter the repository name to create: ", |input| !input.is_empty(),
        "Invalid repository name. Please enter a valid name.");
    let local_path = get_input(&mut rl, "Enter the local path to create the repository in: ", |input| !input.is_empty(),
        "Invalid local path. Please enter a valid path.");
    let remote_url = get_input(&mut rl, "Enter the remote URL (leave blank to create a new GitHub repo): ", |_| true,
"");
    utils::git::setup_git_repo(&repo_name, &local_path);

    let year_input = get_input(&mut rl, "Enter the year to insert the drawing (e.g., 2023): ", |input| {
        input.parse::<i32>().is_ok() && input.parse::<i32>().unwrap() > 1970
    }, "Invalid year. Please enter a valid year.");

    let year: i32 = year_input.parse::<i32>().unwrap();
    let max_contributions = utils::git::get_max_daily_contributions(year);
    println!("Max daily contributions for year {}: {}", year, max_contributions);
    let mult = std::cmp::max(max_contributions / 4, 1);
    println!("Using contribution multiplier: {}", mult);

    let commits_needed: u32 = quantized_image.iter().flatten().map(|&lvl| lvl as u32 * mult).sum::<u32>() * mult;
    println!("Total commits needed: {}", commits_needed);

    let h_offset = get_input(&mut rl, "Enter horizontal offset (number of weeks to shift right): ", |input| {
        input.parse::<i32>().is_ok()
    }, "Invalid offset. Please enter a valid integer.").parse::<i32>().unwrap();

    let mut commit_cmd = utils::git::base_commit_cmd(&repo_name, &local_path);
    let start_date = DateTime::parse_from_rfc3339(
        &utils::git::create_date(year, 1, 1, 0, 0, 0)
    ).unwrap();

    // must start on a sunday to be aligned on contrib graph
    let mut current_date = utils::git::increment_date(
        start_date,
        (h_offset * 7) as u64 + (6-start_date.weekday().num_days_from_monday()) as u64
    );

    println!("Starting commits from date: {}", current_date);
    println!("{}", current_date.weekday());

    let pb = ProgressBar::new(commits_needed as u64);
    pb.set_style(indicatif::ProgressStyle::with_template(
        "[{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta})"
    ).unwrap());

    for x in 0..quantized_image[0].len() {
        for y in 0..quantized_image.len() {
            let activity_lvl = quantized_image[y][x];
            for _ in 0..(mult * activity_lvl as u32) {
                utils::git::commit_with_date(&mut commit_cmd, current_date);
                pb.inc(1);
            }
            current_date = utils::git::increment_date(
                current_date,
                1
            );
        }
    }
    pb.finish_with_message("All commits created.");

    println!("Pushing to remote...");
    utils::git::push_to_remote(&repo_name, &local_path, &remote_url);

    if (get_input(&mut rl, "Clean up local repository? (y/n): ", |input| {
        input.eq_ignore_ascii_case("y") || input.eq_ignore_ascii_case("n")
    }, "Invalid input. Please enter 'y' or 'n'.")).eq_ignore_ascii_case("y") {
        let repo_path = std::path::Path::new(&local_path).join(&repo_name);
        if repo_path.exists() {
            std::fs::remove_dir_all(repo_path).expect("Failed to remove local repository");
            println!("Local repository removed.");
        }
    }

    println!("Process completed! Go to your GitHub profile to see your image.");
}
