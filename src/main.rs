use std::env;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
use nix::sys::stat::Mode;
use nix::sys::stat::fchmodat;
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
                println!("Processing directory: {}", path.display());
                count += visit_dirs(&path, cb)?;
            }
            cb(&entry);
            count += 1;
        }
    }
    Ok(count)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} <owner_name> <group_name> <chmod_value> <directory>", args[0]);
        std::process::exit(1);
    }

    let owner_name = &args[1];
    let group_name = &args[2];
    let chmod_value = u32::from_str_radix(&args[3], 8).expect("Invalid chmod value");
    let root_dir = Path::new(&args[4]);

    let owner_uid = User::from_name(owner_name).expect("Failed to find user")
        .expect("No such user")
        .uid;

    let group_gid = Group::from_name(group_name).expect("Failed to find group")
        .expect("No such group")
        .gid;

    let count = visit_dirs(&root_dir, &|entry: &DirEntry| {
        let path = entry.path();
        chown(path.as_path(), Some(owner_uid), Some(group_gid))
            .expect("Failed to change ownership");
        fchmodat(None, path.as_path(), Mode::from_bits_truncate(chmod_value), nix::sys::stat::FchmodatFlags::NoFollowSymlink)

            .expect("Failed to change file mode");
    })?;
    println!("Total processed file count : {}", count);
    Ok(())
}
