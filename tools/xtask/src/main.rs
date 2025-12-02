use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::process::exit;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let cmd = args.next();
    const OPTIONS: &str = "linkcheck, style-check, test-all";
    match cmd.as_deref() {
        Some("test-all") => {
            mdbook_test()?;
            style_check()?;
            fmt()?;
            linkcheck(args)?;
            cargo_test()?;
            eprintln!("all tests passed!");
        }
        Some("linkcheck") => linkcheck(args)?,
        Some("style-check") => style_check()?,
        Some("-h" | "--help") => eprintln!("valid options: {OPTIONS}"),
        Some(x) => {
            eprintln!("error: unknown command `{x}` (valid options: {OPTIONS})");
            exit(1);
        }
        None => {
            eprintln!("error: specify a command (valid options: {OPTIONS})");
            exit(1);
        }
    }
    Ok(())
}

fn root_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn mdbook_test() -> Result<()> {
    eprintln!("Testing inline code tests...");
    let status = Command::new("mdbook")
        .arg("test")
        .current_dir(root_dir())
        .status()
        .expect("mdbook should be installed");
    if !status.success() {
        return Err("inline code tests failed".into());
    }
    Ok(())
}

fn style_check() -> Result<()> {
    eprintln!("Running style checks...");
    let status = Command::new("cargo")
        .args(["run", "--package=style-check", "--", "src"])
        .current_dir(root_dir())
        .status()
        .expect("cargo should be installed");
    if !status.success() {
        return Err("style check failed".into());
    }
    Ok(())
}

fn fmt() -> Result<()> {
    eprintln!("Checking code formatting...");
    let status = Command::new("cargo")
        .args(["fmt", "--check"])
        .current_dir(root_dir())
        .status()
        .expect("cargo should be installed");
    if !status.success() {
        return Err("fmt check failed".into());
    }
    Ok(())
}

fn cargo_test() -> Result<()> {
    eprintln!("Running cargo tests...");
    let status = Command::new("cargo")
        .arg("test")
        .current_dir(root_dir())
        .status()
        .expect("cargo should be installed");
    if !status.success() {
        return Err("cargo tests failed".into());
    }
    Ok(())
}

fn linkcheck(args: impl Iterator<Item = String>) -> Result<()> {
    eprintln!("Running linkcheck...");
    let root = root_dir();
    let status = Command::new("curl")
        .args(["-sSLo", "linkcheck.sh", "https://raw.githubusercontent.com/rust-lang/rust/master/src/tools/linkchecker/linkcheck.sh"])
        .current_dir(&root)
        .status()
        .expect("curl should be installed");
    if !status.success() {
        return Err("failed to fetch script from GitHub".into());
    }

    let status = Command::new("sh")
        .args(["linkcheck.sh", "--all", "reference"])
        .args(args)
        .current_dir(&root)
        .status()
        .expect("sh should be installed");
    if !status.success() {
        return Err("linkcheck failed".into());
    }
    Ok(())
}
