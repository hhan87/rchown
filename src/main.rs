use std::env;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
use nix::unistd::{chown};
use nix::unistd::User;
use nix::unistd::Group;

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<i64> {
    let mut count : i64 = 0;
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                println!("Processing path: {}", path.display());
                count += visit_dirs(&path, cb)?;
            } else {
                count += 1;
                cb(&entry);
            }
        }
    }
    Ok(count)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <owner_name> <group_name> <directory>", args[0]);
        std::process::exit(1);
    }

    let owner_name = &args[1];
    let group_name = &args[2];
    let root_dir = Path::new(&args[3]);

    let owner_uid = User::from_name(owner_name).expect("Failed to find user")
        .expect("No such user")
        .uid;

    let group_gid = Group::from_name(group_name).expect("Failed to find group")
        .expect("No such group")
        .gid;

    let count = visit_dirs(&root_dir, &|entry: &DirEntry| {
        chown(entry.path().as_path(), Some(owner_uid), Some(group_gid))
            .expect("Failed to change ownership");
    })?;
    println!("Total processed file count : {}", count);
    Ok(())
}

