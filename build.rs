use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn get_version() -> Result<(String, String), Box<dyn std::error::Error>> {
    let date_output = Command::new("date").arg("+%Y%m%d").output()?;
    let date = String::from_utf8(date_output.stdout)?.trim().to_string();

    let git_output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()?;
    let commit = String::from_utf8(git_output.stdout)?.trim().to_string();

    Ok((date, commit))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (date, commit) = match get_version() {
        Ok(version) => version,
        Err(err) => {
            eprintln!("Failed to get version info: {}", err);
            return Err(err);
        }
    };

    let version = format!("{}-{}", date, commit);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=.git/HEAD");

    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("version.rs");

    fs::write(
        dest_path,
        format!("pub const VERSION: &str = \"{}\";", version),
    )?;

    Ok(())
}
