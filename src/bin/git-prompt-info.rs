use std::io;
use std::process::{exit, Command, Output};

#[derive(Debug, Default)]
struct PromptInfo {
  sha: String,
  head: String,
  is_dirty: bool,
  is_weird: bool,
}

fn main() {
  let info = PromptInfo::new();
  println!("{}", info);
}

impl PromptInfo {
  fn new() -> Self {
    let out = Command::new("git")
      .args(&[
        "--no-optional-locks",
        "status",
        "--branch",
        "--porcelain=v2",
      ])
      .output();

    maybe_bail(&out);

    let mut info: PromptInfo = Default::default();

    let stdout = String::from_utf8(out.unwrap().stdout).expect("bad string");

    for line in stdout.lines() {
      if line.starts_with("# branch.oid") {
        info.sha = line.rsplit(" ").next().unwrap()[0..8].into();
        continue;
      }

      if line.starts_with("# branch.head") {
        info.head = line.rsplit(" ").next().unwrap().into();
        continue;
      }

      if !line.starts_with("#") {
        info.is_dirty = true;
        break;
      }
    }

    info.fill_weird();
    info
  }

  fn fill_weird(&mut self) {
    let out = Command::new("git")
      .args(&["rev-parse", "--git-dir"])
      .output();

    maybe_bail(&out);

    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    use std::path::PathBuf;

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
        self.is_weird = true;
        return;
      }
    }
  }
}

impl std::fmt::Display for PromptInfo {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    let detached = self.head == "(detached)";

    let prep = if detached { "at" } else { "on" };
    let branch = if detached { &self.sha } else { &self.head };

    return write!(
      f,
      "1 {} {} {} {}",
      prep,
      branch,
      if self.is_dirty { 1 } else { 0 },
      if self.is_weird { 1 } else { 0 },
    );
  }
}

fn maybe_bail(out: &io::Result<Output>) {
  if out.is_err() || !out.as_ref().unwrap().status.success() {
    println!("0");
    exit(0);
  }
}
