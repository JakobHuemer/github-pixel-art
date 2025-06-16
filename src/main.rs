use github_pixel_art::{
    interactive::{HomeOption, InteractionManager as IaManager},
    repo_manager::github::GitHubAuthBuilder,
};
use std::process::exit;

// tokio main
#[tokio::main]
async fn main() {
    IaManager::welcome();

    loop {
        let res = IaManager::home();

        match res {
            Ok(option) => match option {
                HomeOption::CreateProject => {
                    // for testing create a github auth
                    let builder = GitHubAuthBuilder::new("JakobHuemer".to_string());

                    // use device code flow

                    match builder.with_device().await {
                        Ok(device_code_builder) => {
                            println!("Code is: {}", device_code_builder.user_code);
                        }
                        Err(auth_error) => {
                            println!("an error occoured: {:?}", auth_error);
                        }
                    }
                }
                HomeOption::OpenProject => {}
                HomeOption::OpenRemoteProject => {}
                HomeOption::Exit => {
                    println!("Bye!");
                    exit(0);
                } // open project
            },
            Err(_) => {
                // println!("Oops! Something happened\n");
                exit(1);
            }
        };
    }

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
