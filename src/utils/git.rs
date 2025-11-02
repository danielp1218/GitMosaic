use chrono::{DateTime, Days, FixedOffset};
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

pub fn setup_git_repo(repo_name: &str, path: &str) {
    Command::new("git")
        .current_dir(path)
        .arg("init")
        .arg(repo_name)
        .status()
        .expect("Failed to initialize git repository");
}

pub fn base_commit_cmd(repo_name: &str, path: &str) -> Command {
    let mut commit_cmd = Command::new("git");
    commit_cmd
        .current_dir(Path::new(path).join(repo_name))
        .args([
            "commit",
            "--allow-empty",
            "-a",
            "--allow-empty-message",
            "-m",
            "\"\"",
        ]);
    commit_cmd
}

pub fn push_to_remote(repo_name: &str, path: &str, remote_url: &str) {
    if remote_url.is_empty() {
        Command::new("gh")
            .current_dir(path)
            .args([
                "repo",
                "create",
                format!("--source={}", repo_name).as_str(),
                "--private",
                "--push",
            ])
            .status()
            .expect("Failed to create GitHub repository");
        return;
    }

    Command::new("git")
        .current_dir(Path::new(path).join(repo_name))
        .args(["remote", "add", "origin", remote_url])
        .status()
        .expect("Failed to add remote");

    Command::new("git")
        .current_dir(Path::new(path).join(repo_name))
        .args(["pull", "origin", "main", "--rebase"])
        .status()
        .expect("Failed to pull from remote");

    Command::new("git")
        .current_dir(Path::new(path).join(repo_name))
        .args(["push", "-u", "origin", "main"])
        .status()
        .expect("Failed to push to remote");
}

pub fn commit_with_date(commit_cmd: &mut Command, date: DateTime<FixedOffset>) {
    commit_cmd
        .env("GIT_COMMITTER_DATE", date.to_rfc3339())
        .env("GIT_AUTHOR_DATE", date.to_rfc3339());
    commit_cmd
        .stdout(Stdio::null())
        .status()
        .expect("commit failed");
}

pub fn create_date(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> String {
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hour, minute, second
    )
}

pub fn increment_date(date: DateTime<FixedOffset>, days: u64) -> DateTime<FixedOffset> {
    date.checked_add_days(Days::new(days))
        .expect("Date increment failed")
}

pub fn get_max_daily_contributions(year: i32) -> u32 {
    let from = create_date(year, 1, 1, 0, 0, 0);
    let to = create_date(year, 12, 31, 23, 59, 59);

    let contrib_query = format!(
        r#"
        {{
        viewer {{
            contributionsCollection(from: "{from}", to: "{to}") {{
            contributionCalendar {{
                weeks {{
                contributionDays {{
                    contributionCount
                }}
                }}
            }}
            }}
        }}
        }}
        "#
    );

    let output = Command::new("gh")
        .args([
            "api",
            "graphql",
            "-f",
            &format!("query={}", contrib_query),
            "--jq",
            ".data.viewer.contributionsCollection.contributionCalendar.weeks
                | map(.contributionDays)
                | add
                | max_by(.contributionCount)",
        ])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute gh command")
        .wait_with_output()
        .expect("Failed to read gh output");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // The output is expected to be a JSON object like: {"contributionCount": X }
    let stdout = stdout
        .trim()
        .replace("{", "")
        .replace("}", "")
        .replace("\"", "");
    let parts: Vec<&str> = stdout.split(':').collect();
    let stdout = parts
        .get(1)
        .expect("Failed to get contribution count")
        .trim();
    stdout
        .trim()
        .parse::<u32>()
        .expect("Failed to parse max contributions")
}
