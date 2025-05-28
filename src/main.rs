mod image_to_tiles;
mod repo_manager;

use crate::image_to_tiles::generate_commit_dates;
use crate::repo_manager::RepoManager;
use colored::Colorize;
use image::ImageReader;
use image_to_tiles::convert_image_to_tiles;
use log::{error, info, warn};
use num_integer::Roots;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::thread::sleep;
use std::time::Duration;
/*
This program gets executed when you want to update the image:

start:

- if not file exists $repo_folder/sha.txt
    then:
        rm -fr $repo_folder
        clone git@github:$github_username/$repo_name.git

: sha.txt exists

- compare sha.txt with the computed sha256 of the current image
- if equal
    then:
        exit
    else:
        continue

: new image must be uploaded
- delete github repo $repo_name from github
- create new empty github repo, clone
- compute commit dates array of image
- compute sha of that image density array

- write sha in sha.txt in repo.

- commit for every date in the dates array
    - change by touching marker.txt
    - git commit -m "c: $date" --date $date

- push to github
- exit

*/

fn main() {
    // const GITHUB_USERNAME: &str = "JakobHuemer";
    // const GITHUB_EMAIL: &str = "j.huemer-fistelberger@htblaleonding.onmicrosoft.com";
    // const SSH_KEYFILE: &str = "jh-id_ed";

    const GITHUB_USERNAME: &str = "JakobFistelberger";
    const GITHUB_EMAIL: &str = "jakobfistelberger@gmail.com";
    const SSH_KEYFILE: &str = "jf-id_ed";

    const REPO_NAME: &str = "green-tiles2";
    const YEAR: i32 = 2016;
    const REPO_FOLDER: &str = "git-repo";
    const IMAGE_PATH: &str = "assets/nextlevel.png";

    let mut repo_manager = RepoManager::new(
        GITHUB_USERNAME.to_string(),
        GITHUB_EMAIL.to_string(),
        REPO_NAME.to_string(),
        REPO_FOLDER.to_string(),
        SSH_KEYFILE.to_string(),
    );

    let pixels: Vec<u8> = convert_image_to_tiles(YEAR, "assets/pixelart.jpg");

    println!("{:?}", pixels);

    println!("Now making commits");

    repo_manager.commit_tiles(YEAR, &pixels);
}

/// Determines if the current image is different from a previous image.
///
/// # Returns
/// - `false` if the previous image is exactly equal to the new image
/// - `true` for all other cases (different images, no previous image, etc.)
fn check_previous_image(prev_img_path: &Path, pixels: &Vec<u8>) -> bool {
    info!("Checking if the image is new");

    let file = match File::open(&prev_img_path) {
        Ok(f) => f,
        Err(_i) => {
            warn!(
                "{:?} could not be opened -> ABORTING",
                prev_img_path.file_name()
            );
            return false;
        }
    };

    let mut prev_img_text = String::new();
    let mut reader = BufReader::new(file);

    if let Err(_e) = reader.read_line(&mut prev_img_text) {
        warn!(
            "{:?} could not be read -> ABORTING",
            prev_img_path.file_name().unwrap()
        );
        return false;
    };

    let new_image_text = vec_to_str(pixels);

    prev_img_text.eq_ignore_ascii_case(&new_image_text)
}

fn reset_github_repo(repo_name: &str, github_token: &str) -> Result<(), String> {
    let run_gh = |args: &[&str]| -> Result<(), String> {
        Command::new("gh")
            .env("GITHUB_TOKEN", github_token)
            .args(args)
            .output()
            .map_err(|e| e.to_string())
            .and_then(|output| {
                if output.status.success() {
                    Ok(())
                } else {
                    Err(String::from_utf8_lossy(&output.stderr).into_owned())
                }
            })
    };

    run_gh(&["repo", "delete", repo_name, "--yes"])?;
    sleep(Duration::from_secs(5));
    run_gh(&["repo", "create", repo_name, "--public", "--confirm"])
}

fn vec_to_str(vec: &Vec<u8>) -> String {
    vec.iter()
        .map(|x2| format!("{}", x2))
        .reduce(|x, x1| {
            return format!("{}{}", x, x1);
        })
        .unwrap_or_else(|| "".to_string())
}
