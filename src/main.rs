//! Program to convert an image for a specific year
//! to

use github_pixel_art::interactive::InteractionManager as IaManager;

fn main() {
    // const GITHUB_USERNAME: &str = "JakobHuemer";
    // const GITHUB_EMAIL: &str = "j.huemer-fistelberger@htblaleonding.onmicrosoft.com";
    // const SSH_KEYFILE: &str = "jh-id_ed";

    const GITHUB_USERNAME: &str = "JakobFistelberger";
    const GITHUB_EMAIL: &str = "jakobfistelberger@gmail.com";
    const SSH_KEYFILE: &str = "jf-id_ed";

    const REPO_NAME: &str = "green-tiles2";
    const YEAR: i32 = 2015;
    const REPO_FOLDER: &str = "git-repo";
    const IMAGE_PATH: &str = "assets/nextlevel.png";

    IaManager::welcome();

    // let mut repo_manager = RepoManager::new(
    //     GITHUB_USERNAME.to_string(),
    //     GITHUB_EMAIL.to_string(),
    //     REPO_NAME.to_string(),
    //     REPO_FOLDER.to_string(),
    //     SSH_KEYFILE.to_string(),
    // );
    //
    // let pixels: Vec<u8> = convert_image_to_tiles(YEAR, "assets/pixelart.jpg");
    //
    // println!("{:?}", pixels);
    //
    // println!("Now making commits");
    //
    // repo_manager.commit_tiles(YEAR, &pixels);
}
