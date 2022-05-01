use clap::{App, Arg};
use std::process::Command;

struct Info<'a> {
    is_dirty: bool,
    prep: &'a str,
    branch: &'a str,
}

const GRAY: &str = "\x1b[38;5;242m";
const TEAL: &str = "\x1b[38;5;023m";
const STAR: &str = "\x1b[38;5;124m";
const RESET: &str = "\x1b[m";

fn main() {
    let matches = App::new("git-prompt-info")
        .arg(Arg::with_name("zsh").long("zsh").help("output zsh colors"))
        .get_matches();

    let out = Command::new("git")
        .args(&["--no-optional-locks", "status", "--branch", "--porcelain=v2"])
        .output();

    if out.is_err() {
        println!("0");
        return;
    }

    let out = out.unwrap();
    let is_zsh = matches.is_present("zsh");

    if !out.status.success() {
        return do_exit_early(is_zsh);
    }

    let mut sha = "??";
    let mut head = "??";
    let mut is_dirty = false;

    let stdout = String::from_utf8(out.stdout).expect("bad string");

    for line in stdout.lines() {
        if line.starts_with("# branch.oid") {
            sha = &line.rsplit(" ").next().unwrap()[0..8];
            continue;
        }

        if line.starts_with("# branch.head") {
            head = line.rsplit(" ").next().unwrap();
            continue;
        }

        if !line.starts_with("#") {
            is_dirty = true;
            break;
        }
    }

    let prep = if head == "(detached)" { "at" } else { "on" };
    let branch = if head == "(detached)" { sha } else { head };

    let info = Info {
        is_dirty,
        prep,
        branch,
    };

    if matches.is_present("zsh") {
        return do_zsh(info);
    }

    println!("1 {} {} {}", prep, branch, if is_dirty { 1 } else { 0 });
}

fn do_exit_early(is_zsh: bool) {
    if is_zsh {
        print!("");
        return;
    }

    println!("0")
}

fn do_zsh(info: Info) {
    println!(
        " {}{} {}{}{}{}{}",
        GRAY,
        info.prep,
        TEAL,
        info.branch,
        STAR,
        if info.is_dirty { "*" } else { "" },
        RESET
    );
}
