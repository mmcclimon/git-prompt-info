use clap::App;
use std::process::Command;

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

  if out.is_err() {
    println!("0");
    return;
  }

  let out = out.unwrap();

  if !out.status.success() {
    println!("0");
    return;
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

  println!("1 {} {} {}", prep, branch, if is_dirty { 1 } else { 0 });
}
