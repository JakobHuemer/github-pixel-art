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

                    let device_code_builder = builder.with_device().await;

                    match device_code_builder {
                        Ok(mut code_builder) => {
                            println!(
                                "Open {} and enter the code: {}",
                                code_builder.verification_uri, code_builder.user_code
                            );
                            code_builder.wait_for_user().await;
                        }
                        Err(err) => {
                            println!("{:?}", err);
                        }
                    };
                }
                HomeOption::OpenProject => {}
                HomeOption::OpenRemoteProject => {}
                HomeOption::Exit => {
                    println!("Bye!");
                    exit(0);
                } // open project
            },
            Err(_) => {
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
