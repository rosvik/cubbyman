use git2::*;

#[allow(dead_code)]
fn fetch() {
    let repo = Repository::open(".").unwrap();
    let mut origin = repo.find_remote("origin").unwrap();

    let branch = repo.find_branch("main", BranchType::Local).unwrap();

    let branch_ref = branch.into_reference();
    let branch_ref_name = branch_ref.name().unwrap();
    repo.set_head(branch_ref_name).unwrap();

    let mut remote_callbacks = RemoteCallbacks::new();
    remote_callbacks.credentials(|_url, _username_from_url, _allowed_types| {
        println!(
            "url: {:?}, username: {:?}, allowed: {:?}",
            _url, _username_from_url, _allowed_types
        );
        Cred::ssh_key_from_agent(_username_from_url.unwrap())
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(remote_callbacks);

    println!("Ready to fetch");
    println!("branch_ref_name: {:?}", branch_ref_name);

    println!(
        "old commit hash: {:?}",
        repo.head().unwrap().peel_to_commit().unwrap().id()
    );

    let result = origin.fetch(&[branch_ref_name], Some(&mut fetch_options), None);
    println!("result: {:?}", result);

    println!(
        "new commit hash: {:?}",
        repo.head().unwrap().peel_to_commit().unwrap().id()
    );
}

#[test]
fn test_push() {
    fetch();
}
