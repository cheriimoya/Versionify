use git2::{Repository, Signature};
use rspotify::{model::PlaylistId, prelude::*, ClientCredsSpotify, Credentials};
use std::{env, fs::write};

#[tokio::main]
async fn main() {
    // Setup logger and get all necessary variables
    env_logger::init();
    let creds = Credentials::from_env().unwrap();
    let git_repo_path = env::var("GIT_REPO_PATH").unwrap();
    let playlist_id = env::var("PLAYLIST_ID").unwrap();

    // Open the git repo, if we cannot open it, we cannot continue
    let repo = match Repository::init(&git_repo_path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };

    // Get spotify token
    let mut spotify = ClientCredsSpotify::new(creds);
    spotify.request_token().await.unwrap();

    // Try to load playlist
    let playlist_uri = PlaylistId::from_id(&playlist_id).unwrap();
    let playlist = spotify.playlist(&playlist_uri, None, None).await;
    if playlist.is_err() {
        panic!("No playlist found with this id");
    }
    let playlist = playlist.unwrap();

    // Write the json representation of the playlist to a file
    let path_to_playlist_file = format!("{}/{}.json", git_repo_path, playlist_id);
    write(
        &path_to_playlist_file,
        serde_json::to_string_pretty(&playlist).unwrap(),
    )
    .unwrap();

    // Create initial commit if repo is empty
    let commiter_signature =
        Signature::now("Spotify Playlist Versioning", "email@example.com").unwrap();
    if repo.is_empty().unwrap() {
        println!("Repo is empty, creating initial commit");
        let tree = repo.index().unwrap().write_tree().unwrap();
        repo.commit(
            Some("HEAD"),
            &commiter_signature,
            &commiter_signature,
            "Initial commit",
            &repo.find_tree(tree).unwrap(),
            &[],
        )
        .unwrap();
    }

    // This adding the newly created file to the index, think `git add`
    let mut index = repo.index().unwrap();
    index
        .add_path(std::path::Path::new(&format!("{}.json", playlist_id)))
        .unwrap();
    let tree = index.write_tree().unwrap();

    // This actually commits (`git commit`) the file
    let tree = repo.find_tree(tree).unwrap();
    let commit = repo.head().unwrap().peel_to_commit().unwrap();
    let commit_id = repo
        .commit(
            Some("HEAD"),
            &commiter_signature,
            &commiter_signature,
            "Changes to Playlist",
            &tree,
            &[&commit],
        )
        .unwrap();

    // This is needed to not leave a dirty workdir/index
    repo.reset(
        &repo.find_object(commit_id, Some(git2::ObjectType::Commit)).unwrap(),
        git2::ResetType::Mixed,
        None,
    )
    .unwrap();
}
