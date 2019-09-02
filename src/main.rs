use chrono::prelude::*;
use std::io;
use std::process::{exit, Command};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "update-dependencies")]
struct Opt {
    branches: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();

    git("checkout master").unwrap();

    let branch_name = format!("update-dependencies-{}", date());
    let _ = git(&format!("branch {}", branch_name));
    git(&format!("checkout {}", branch_name)).unwrap();

    merge_each_brach(&opt.branches);

    let _ = git(&format!("push --set-upstream origin {}", branch_name));
}

fn merge_each_brach(branches: &[String]) {
    if branches.is_empty() {
        return;
    }

    let first = &branches[0];
    let rest = &branches[1..];

    let status: Result<(), Option<io::Error>> = (|| {
        git(&format!("checkout -b {} origin/{}", first, first))?;
        git("checkout -")?;
        git(&format!("merge --no-edit --no-ff {}", first))?;
        Ok(())
    })();

    match status {
        Ok(()) => {
            merge_each_brach(rest);
        }
        Err(_) => {
            eprintln!("Failed to merge {}", first);
            eprintln!("Fix the issue and continue from the next branch with");

            let next_command = format!("update-dependencies {}", rest.join(" "));
            eprintln!("\n{}", next_command);

            exit(1)
        }
    }
}

fn git(command: &str) -> Result<(), Option<io::Error>> {
    let status = (|| {
        let args = command.split(" ");
        let mut child = Command::new("git").args(args).spawn()?;
        child.wait()
    })();

    match status {
        Ok(s) => {
            if s.success() {
                Ok(())
            } else {
                Err(None)
            }
        }
        Err(e) => Err(Some(e)),
    }
}

fn date() -> String {
    let now = Utc::now();
    format!("{}-{}-{}", now.year(), now.month(), now.day())
}
