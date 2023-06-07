use std::{env, process::{Command, Stdio}};

fn main() {
    let args = env::args().nth(1);

    match args.as_deref() {
        Some("serve") => serve(get_env()),
        _ => print_help(),
    }
}

struct Envs {
    git_hash: Option<String>,
}

fn get_env() -> Envs {
    // git rev-parse --short HEAD
    let git_hash = Command::new("git")
        .arg("rev-parse")
        .arg("--short")
        .arg("HEAD")
        // Silence terminal output
        .stdout(Stdio::piped())
        .spawn()
        .expect("Could not get git hash: git is likely not on the path")
        .wait_with_output()
        .expect("Error waiting for git output");

    // If the status is a failure, this build might not be in a git repo (someone downloaded a zip from github).
    let git_hash = git_hash
        .status
        .success()
        .then(|| String::from_utf8_lossy(&git_hash.stdout).to_string());

    Envs { git_hash }
}

fn serve(envs: Envs) {
    let open = env::args()
        .nth(2)
        .as_deref()
        .filter(|&arg| arg == "--open")
        .is_some();

    let mut command = Command::new("zola");
    command.arg("serve");

    if let Some(git_hash) = envs.git_hash {
        command.env("SITE_COMMIT", git_hash);
    }

    if open {
        command.arg("--open");
    }

    command.spawn().expect("Unable to serve page");
}

fn print_help() {
    println!(
        r#"Tasks:
---------------------------------------------------------------------------
serve [--open]      serve the web page locally.
                    If --open is present, a browser window will be opened.
"#
    )
}
