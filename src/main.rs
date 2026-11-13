mod build_cache;
mod java_config;

use clap::Parser;

#[derive(Clone, Debug, clap::Subcommand)]
enum Command {
    #[command(name = "build")]
    Build,
    #[command(name = "jar")]
    Jar,
    #[command(name = "clean")]
    Clean,
    #[command(name = "run")]
    Run {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<std::ffi::OsString>,
    },
}

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    if let Err(e) = io_main() {
        eprintln!("{e}")
    }
}

fn io_main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let config = java_config::get_java_config()?;
    match cli.command {
        Command::Build => build(&config),
        Command::Jar => jar(&config),
        Command::Run { args } => run(&config, &args),
        Command::Clean => clean(&config),
    }
}

fn build(config: &java_config::JavaConfig) -> std::io::Result<()> {
    let source = config.source_path();
    let output = config.output_path();

    let mut sources = Vec::new();
    collect_java_files(source.to_path_buf(), &mut sources)?;

    if sources.is_empty() {
        Err(std::io::Error::other("source dir is empty"))?
    }

    sources.sort();

    std::fs::create_dir_all(output)?;

    let current_cache = build_cache::BuildCache::new(
        build_cache::fingerprint_files(&sources)?,
        config.main_class.clone(),
    );
    let cache_path = config.cache_path();
    let previous_cache = build_cache::BuildCache::load(&cache_path)?;

    let rebuild = previous_cache.as_ref() != Some(&current_cache);

    if !rebuild {
        println!("nothing to compile...");
        return Ok(());
    }

    let mut os_cmd = std::process::Command::new("javac");
    os_cmd.arg("-d").arg(output);

    if !config.jars.is_empty() {
        unimplemented!()
    }

    os_cmd.args(&sources);

    let status = os_cmd.status()?;

    if !status.success() {
        Err(std::io::Error::other(format!("javac exited with {status}")))?
    }

    current_cache.save(&cache_path)?;

    Ok(())
}

fn jar(config: &java_config::JavaConfig) -> std::io::Result<()> {
    let output = config.output_path();

    let mut os_cmd = std::process::Command::new("jar");

    os_cmd.arg("--create");

    os_cmd.arg("--file").arg("a.jar");

    os_cmd.arg("--main-class").arg(&config.main_class);

    os_cmd.arg("-C").arg(output).arg(".");

    let status = os_cmd.status()?;

    if !status.success() {
        Err(std::io::Error::other(format!("jar exited with {status}")))?
    }

    Ok(())
}

fn run(config: &java_config::JavaConfig, args: &[std::ffi::OsString]) -> std::io::Result<()> {
    build(config)?;

    let status = std::process::Command::new("java")
        .arg("--class-path")
        .arg(config.output_path())
        .arg(&config.main_class)
        .args(args)
        .status()?;

    if !status.success() {
        Err(std::io::Error::other(format!("java exited with {status}")))?
    }

    Ok(())
}

fn clean(config: &java_config::JavaConfig) -> std::io::Result<()> {
    std::fs::remove_dir_all(config.output_path())
}

fn collect_java_files(
    dir: std::path::PathBuf,
    files: &mut Vec<std::path::PathBuf>,
) -> std::io::Result<()> {
    let mut dirs = std::collections::VecDeque::from([dir]);

    while let Some(dir) = dirs.pop_front() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let file_type = entry.file_type()?;

            if file_type.is_dir() {
                dirs.push_back(path);
            } else if file_type.is_file() && path.extension().is_some_and(|e| e == "java") {
                files.push(path);
            }
        }
    }

    Ok(())
}
