use anyhow::{anyhow, Result};
use std::path::PathBuf;

pub enum LimageCommand {
    Build,
    Run(LimageArgs),
    Version,
    Help,
}

impl LimageCommand {
    pub fn parse_args<A>(args: A) -> Result<Self>
    where
        A: Iterator<Item = String>,
    {
        let mut executable = None;
        let mut build_only = false;
        let mut arg_iter = args.fuse();

        let mut args_count = -1;
        loop {
            args_count += 1;

            let next = match arg_iter.next() {
                Some(next) => next,
                None => break,
            };

            match next.as_str() {
                "--help" | "-h" => {
                    return Ok(LimageCommand::Help);
                },
                "--version" | "-v" => {
                    return Ok(LimageCommand::Version);
                },
                "build" => {
                    build_only = true;
                },
                exe => {
                    executable = Some(PathBuf::from(exe));
                }
            }
        }

        if args_count < 2 {
            build_only = true;
        }

        let executable_or_error = executable.ok_or_else(|| anyhow!("expected path to kernel executable as first argument"))?;
        if build_only {
            Ok(Self::Build)
        } else {
            Ok(Self::Run(LimageArgs {executable: executable_or_error}))
        }
    }
}

#[derive(Debug, Clone)]
pub struct LimageArgs {
    pub executable: PathBuf
}