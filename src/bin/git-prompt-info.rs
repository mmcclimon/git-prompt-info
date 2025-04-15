use clap::App;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::{exit, Command};

fn main() {
  App::new("git-prompt-info").get_matches();

  let out = Command::new("git")
    .args(&[
      "--no-optional-locks",
      "status",
      "--branch",
      "--porcelain=v2",
    ])
    .output();

  if out.is_err() || !out.as_ref().unwrap().status.success() {
    bail();
  }

  let out = out.unwrap();

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

  let is_weird = is_weird();

  let prep = if head == "(detached)" { "at" } else { "on" };
  let branch = if head == "(detached)" { sha } else { head };

  println!(
    "1 {} {} {} {}",
    prep,
    branch,
    if is_dirty { 1 } else { 0 },
    if is_weird { 1 } else { 0 },
  );
}

fn is_weird() -> bool {
  let out = Command::new("git")
    .args(&["rev-parse", "--git-dir"])
    .output();

  if out.is_err() || !out.as_ref().unwrap().status.success() {
    bail();
  }

  use std::os::unix::ffi::OsStrExt;
  let raw = out.unwrap().stdout;
  let git_dir = PathBuf::from(OsStr::from_bytes(&raw[..raw.len() - 1]));

  for f in &[
    "rebase-apply",
    "rebase-merge",
    "MERGE_HEAD",
    "CHERRY_PICK_HEAD",
    "REVERT_HEAD",
  ] {
    if git_dir.join(f).exists() {
      return true;
    }
  }

  false
}

fn bail() -> ! {
  println!("0");
  exit(0);
}
