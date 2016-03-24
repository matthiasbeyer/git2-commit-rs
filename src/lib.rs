extern crate git2;

use std::path::Path;
use std::env;
use git2::{Config, Repository, Signature, Error, PushOptions, RemoteCallbacks, Cred, BranchType};

pub struct Author {
    pub name: String,
    pub email: String
}

pub fn get_signature() -> Result<Author, Error> {
    let config = try!(Config::open_default());
    let author = try!(config.get_string("user.name"));
    let email = try!(config.get_string("user.email"));
    Ok(Author {
        name: author.to_string(),
        email: email.to_string()
    })
}


pub fn add<P: AsRef<Path>>(repo: &str, files: &[P], force: bool) -> Result<(), Error> {
    let repo = try!(Repository::open(repo));
    let mut index = try!(repo.index());

    for path in files {
        let path = path.as_ref();
        if force || !repo.status_should_ignore(path).unwrap() {
            let _ = try!(index.add_path(path.as_ref()));
        }
    }

    index.write()
}

pub fn commit(repo: &str, name: &str, email: &str, message: &str) -> Result<(), Error> {
    let signature = try!(Signature::now(name, email));
    let update_ref = Some("HEAD");

    let repo = try!(Repository::open(repo));

    let oid = try!(repo.refname_to_id("HEAD"));
    let parent_commit = try!(repo.find_commit(oid));
    let parents = vec![&parent_commit];

    let mut index = try!(repo.index());
    let tree_oid = try!(index.write_tree());
    let tree = try!(repo.find_tree(tree_oid));

    repo
        .commit(update_ref, &signature, &signature, message, &tree, &parents)
        .map(|_| ())
}

pub fn tag(repo: &str, name: &str, email: &str, tag_name: &str, message: &str) -> Result<(), Error> {
    let repo = try!(Repository::open(repo));
    let obj = try!(repo.revparse_single("HEAD"));
    let signature = try!(Signature::now(name, email));

    repo.tag(tag_name, &obj, &signature, message, false)
        .map(|_| ())
}

pub fn push(repo: &str, url: &str, refs: &[&str]) -> Result<(), Error> {
    let repo = try!(Repository::open(repo));
    let mut remote = try!(repo.remote_anonymous(url));

    let mut cbs = RemoteCallbacks::new();
    cbs.credentials(|_url, _username, _allowed| {
        let token = env::var("GH_TOKEN").expect("GH_TOKEN should contain a valid OAuth token");
        Cred::userpass_plaintext(&token, "")
    });
    let mut opts = PushOptions::new();
    opts.remote_callbacks(cbs);

    remote.push(refs, Some(&mut opts))
}

pub fn branch(repo: &str, branch_type: BranchType) -> Result<(), Error> {
    let repo = try!(Repository::open(repo));

    let head = try!(repo.head());
    let short = head.shorthand().unwrap_or("empty");

    if branch_type == BranchType::Local {
        if head.is_branch() {
            println!("* {}", short);
        } else {
            let oid = head.target().expect("Invalid OID");
            println!("* ({} detached at {})", short, oid);
        }
    }

    let branches = repo.branches(Some(branch_type)).expect("No branches found");

    for (branch, _) in branches {
        let name = branch.name().expect("Invalid branch name");
        let name = name.expect("Branch name not UTF-8");

        if name != short {
            println!("  {}", name);
        }
    }

    Ok(())
}