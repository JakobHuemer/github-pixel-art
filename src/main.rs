use github_pixel_art::repo_manager::github::GitHubAuthBuilder;
use std::process::exit;

// tokio main
#[tokio::main]
async fn main() {
    loop {
        // for testing create a github auth
        let builder = GitHubAuthBuilder::new();

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
}
