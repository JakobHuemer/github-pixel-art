//! Manages and sets up the repository locally and remotely
//! used for the commit tiles

mod git;
mod github;

use git2::{BranchType, Cred, PushOptions, RemoteCallbacks, Repository, Signature, Time};
use std::fs::remove_dir_all;
use std::path::Path;
use time::util::is_leap_year;
use time::{Date, Month, PrimitiveDateTime};

pub struct RepoManager {
    repo: Repository,
    repo_name: String,
    repo_folder: String,
    username: String,
    email: String,
    keyfile: String,
}

impl RepoManager {
    pub fn new(
        username: String,
        email: String,
        repo_name: String,
        repo_folder: String,
        keyfile: String,
    ) -> RepoManager {
        // clear destination
        // clone repo

        if Path::new(&repo_folder).exists() {
            remove_dir_all(Path::new(&repo_folder)).expect("Failed to clear repo folder");
        }

        let mut callbacks = RemoteCallbacks::new();
        let cb_keyfile = keyfile.clone();
        callbacks.credentials(move |_url, username_from_url, _allowed_types| {
            Cred::ssh_key(
                username_from_url.unwrap(),
                None,
                Path::new(&format!("./{}", cb_keyfile)),
                None,
            )
        });

        callbacks.transfer_progress(|progress| {
            println!(
                "Progress Objects: {}/{}",
                progress.received_objects(),
                progress.total_objects()
            );
            println!(
                "Progress Deltas: {}/{}",
                progress.indexed_deltas(),
                progress.total_deltas()
            );
            println!("-----------------");
            true
        });

        // prepare fetch options
        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(callbacks);

        // prepare builder
        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fo);

        let mut repo = builder
            .clone(
                format!("git@github.com:{}/{}", username, repo_name).as_str(),
                Path::new(&repo_folder),
            )
            .expect("Clone Failed");

        RepoManager {
            repo,
            username,
            email,
            keyfile,
            repo_name,
            repo_folder,
        }
    }

    /// removes all previous commits and adds commits for the tiles
    ///
    pub fn commit_tiles(&mut self, year: i32, tiles: &Vec<u8>) {
        // start from 1st january in year
        // for each tile increment the day

        let max_days = if is_leap_year(year) { 366 } else { 365 };
        if tiles.len() > max_days {
            panic!(
                "{} are more days than the given year {} has!",
                tiles.len(),
                year
            );
        }

        let mut current_date = Date::from_calendar_date(year, Month::January, 1)
            .expect("Failed to create date from commits tiles");

        // delete and create main branch
        // if finding main branch fails, skip deletion
        let mut main_branch = self.repo.find_branch("main", BranchType::Local);

        // self.repo.set_head("reads/heads/main");
        // let _ = main_branch.is_ok_and(|mut t1| {
        //     t1.delete().expect("Failed to delete main branch");
        //     true
        // });

        let tree_id = self
            .repo
            .index()
            .expect("Failed to get repo index")
            .write_tree()
            .expect("Failed to get write tree");

        let mut tree = self
            .repo
            .find_tree(tree_id)
            .expect("Failed to find tree with id");

        let mut is_first = true;

        // TODO: fix that the first day will only have a single commit

        for t in tiles.iter() {
            if *t != 0 {
                let signature = Signature::new(
                    self.username.as_str(),
                    self.email.as_str(),
                    &Time::new(
                        PrimitiveDateTime::new(current_date.clone(), time::Time::MIDNIGHT)
                            .assume_utc()
                            .unix_timestamp(),
                        0,
                    ),
                )
                .expect("Failed to create signature");

                if is_first {
                    is_first = false;
                    println!("Adding initial commit");

                    // add initial commit
                    let commit_id = self
                        .repo
                        .commit(
                            None,
                            &signature,
                            &signature,
                            format!("inital commit on {}", current_date.to_string()).as_str(),
                            &tree,
                            &[],
                        )
                        .expect("Failed to create initial commit");

                    if let Ok(mut branch) = self.repo.find_branch("main", BranchType::Local) {
                        let target = branch.get().target().expect("Branch has no target");
                        self.repo
                            .set_head_detached(target)
                            .expect("Failed to detach HEAD");
                        branch.delete().expect("Failed to delete main branch");
                    };

                    let commit = self
                        .repo
                        .find_commit(commit_id)
                        .expect("Failed to find initial commit");

                    self.repo
                        .branch("main", &commit, false)
                        .expect("Failed to create new main branch");

                    self.repo
                        .set_head("refs/heads/main")
                        .expect("Failed to set head to main branch");

                    let tree_id = self
                        .repo
                        .index()
                        .expect("Failed to get repo index")
                        .write_tree()
                        .expect("Failed to get write tree");

                    tree = self
                        .repo
                        .find_tree(tree_id)
                        .expect("Failed to find tree with id");

                    // // write single initial file to the repo
                    // let file_name = "marker.txt";
                    // let file = File::create(format!("{}/{}", self.repo_folder, file_name));
                    // // add file
                    // if let Err(e) = file {
                    //     panic!("Failed to create file {}: {}", file_name, e);
                    // }
                    //
                    // println!("Adding file: {}", file_name);
                    //
                    // let file = file.unwrap();
                    // let mut index = self.repo.index().expect("Failed to get repo index");
                    // index
                    //     .add_all(
                    //         Path::new(format!("{}", file_name).as_str()),
                    //         IndexAddOption::DEFAULT,
                    //         None,
                    //     )
                    //     .expect("Failed to add all files to index");
                    //
                    // println!("{:?}", index.len());
                } else {
                    // normal commit
                    let reference = Some("HEAD");
                    let commit_message = format!("commit with date: {}", current_date.to_string());

                    for i in 0..*t {
                        // println!("Adding commit for: {}", current_date.to_string());
                        self.repo
                            .commit(
                                reference.clone(),
                                &signature,
                                &signature,
                                commit_message.as_str(),
                                &tree,
                                &[&self
                                    .repo
                                    .head()
                                    .expect("Failed to get head")
                                    .peel_to_commit()
                                    .expect("Failed to peel to commit")],
                            )
                            .expect("failed to commit for commit");
                    }
                };
            }

            current_date = current_date
                .next_day()
                .expect("Could not increment to next day while committing tiles");
        }

        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key(
                username_from_url.unwrap(),
                None,
                Path::new(&format!("./{}", self.keyfile)),
                None,
            )
        });

        let mut po = PushOptions::new();
        po.remote_callbacks(callbacks);

        // push changes
        self.repo
            .find_remote("origin")
            .expect("Failed to find remote origin")
            .push(&["refs/heads/main:refs/heads/main"], Some(&mut po))
            .expect("Failde to push");
    }
}
