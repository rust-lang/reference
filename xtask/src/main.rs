use std::error::Error;
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

fn mdbook_test() -> Result<()> {
    eprintln!("Testing inline code tests...");
    let status = Command::new("mdbook")
        .arg("test")
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
        .args(["run", "--manifest-path=style-check/Cargo.toml", "--", "src"])
        .status()
        .expect("cargo should be installed");
    if !status.success() {
        return Err("style check failed".into());
    }
    Ok(())
}

fn fmt() -> Result<()> {
    eprintln!("Checking code formatting...");
    for dir in ["style-check", "mdbook-spec", "xtask"] {
        let status = Command::new("cargo")
            .args(["fmt", "--check"])
            .current_dir(dir)
            .status()
            .expect("cargo should be installed");
        if !status.success() {
            return Err(format!("fmt check failed for {dir}").into());
        }
    }
    Ok(())
}

fn cargo_test() -> Result<()> {
    eprintln!("Running cargo tests...");
    let status = Command::new("cargo")
        .arg("test")
        .current_dir("mdbook-spec")
        .status()
        .expect("cargo should be installed");
    if !status.success() {
        return Err("mdbook-spec test failed".into());
    }
    Ok(())
}

fn linkcheck(args: impl Iterator<Item = String>) -> Result<()> {
    eprintln!("Running linkcheck...");
    let status = Command::new("curl")
        .args(["-sSLo", "linkcheck.sh", "https://raw.githubusercontent.com/rust-lang/rust/master/src/tools/linkchecker/linkcheck.sh"])
        .status()
        .expect("curl should be installed");
    if !status.success() {
        return Err("failed to fetch script from GitHub".into());
    }

    let status = Command::new("sh")
        .args(["linkcheck.sh", "--all", "reference"])
        .args(args)
        .status()
        .expect("sh should be installed");
    if !status.success() {
        return Err("linkcheck failed".into());
    }
    Ok(())
}
