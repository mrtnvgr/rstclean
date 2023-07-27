use std::{
    env,
    path::PathBuf,
    process::{self, Command},
};

use walkdir::WalkDir;

fn main() {
    let path: PathBuf = get_folder();

    let projects: Vec<PathBuf> = WalkDir::new(path)
        .into_iter()
        .flatten()
        .filter(|x| x.file_name() == "Cargo.toml")
        .map(|x| x.path().to_path_buf())
        .filter_map(|x| x.canonicalize().ok())
        .collect();

    for mut project in projects {
        project.pop();

        let result = Command::new("cargo")
            .arg("clean")
            .current_dir(&project)
            .status()
            .unwrap();

        if result.success() {
            println!("{}", project.display());
        } else {
            eprintln!("Failed to clean {}", project.display());
        }
    }
}

#[allow(clippy::indexing_slicing)]
fn get_folder() -> PathBuf {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path>", args[0]);
        process::exit(1);
    }

    PathBuf::from(&args[1])
}
