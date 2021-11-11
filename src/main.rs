#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]

/*!
# Proton Call

Run any Windows program through [Valve's Proton](https://github.com/ValveSoftware/Proton).

## Usage:

Defaults to the latest version of Proton.
```
proton-call -r foo.exe
```

Defaults to the latest version of Proton, all extra arguments passed to the executable.
```
proton-call -r foo.exe --goes --to program
```

`--goes --to program` are passed to the proton / the program

Uses specified version of Proton, any extra arguments will be passed to the executable.
```
proton-call -p 5.13 -r foo.exe
```

Uses custom version of Proton, give the past to directory, not the Proton executable itself.
```
proton-call -c '/path/to/Proton version' -r foo.exe
```
 */

use proton_call::{err, Config, Error, Index, Proton, Version};
use std::path::PathBuf;

/// Type to handle and parse command line arguments with `Jargon`
#[derive(Debug)]
struct Args {
    program: PathBuf,
    version: Version,
    log: bool,
    custom: Option<PathBuf>,
    args: Vec<String>,
}

/// Main function which purely handles errors
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program: String = args[0].split('/').last().unwrap_or(&args[0]).to_string();
    if let Err(e) = proton_caller(args) {
        eprintln!("{}: error: {}", program, e);
    }
}

/// Effective main function which parses arguments
fn proton_caller(args: Vec<String>) -> Result<(), Error> {
    use jargon_args::Jargon;

    let config = Config::open()?;
    let mut parser = Jargon::from_vec(args);

    if parser.contains(["-h", "--help"]) {
        help();
    } else if parser.contains(["-v", "--version"]) {
        version();
    } else if parser.contains(["-i", "--index"]) {
        let common_index = Index::new(&config.common())?;
        println!("{}", common_index);
    } else if parser.contains("--config") {
        println!("{}", config);
    } else {
        let args = Args {
            program: parser.result_arg(["-r", "--run"])?,
            version: parser.option_arg(["-p", "--proton"]).unwrap_or_default(),
            log: parser.contains(["-l", "--log"]),
            custom: parser.option_arg(["-c", "--custom"]),
            args: parser.finish(),
        };

        if args.custom.is_some() {
            custom_mode(&config, args)?;
        } else {
            normal_mode(&config, args)?;
        }
    }

    Ok(())
}

/// Runs caller in normal mode, running indexed Proton versions
fn normal_mode(config: &Config, args: Args) -> Result<(), Error> {
    let common_index = Index::new(&config.common())?;
    let proton_path = match common_index.get(args.version) {
        Some(pp) => pp,
        None => return err!("Proton {} is not found", args.version),
    };

    let proton = Proton::new(
        args.version,
        proton_path,
        args.program,
        args.args,
        args.log,
        config.data(),
        config.steam(),
    );

    if proton.run()?.success() {
        Ok(())
    } else {
        err!("Proton exited with an error")
    }
}

/// Runs caller in custom mode, using a custom Proton path
fn custom_mode(config: &Config, args: Args) -> Result<(), Error> {
    if let Some(custom) = args.custom {
        let proton = Proton::new(
            Version::from_custom(custom.as_path()),
            custom,
            args.program,
            args.args,
            args.log,
            config.data(),
            config.steam(),
        );

        if proton.run()?.success() {
            Ok(())
        } else {
            err!("Proton exited with an error")
        }
    } else {
        err!("failed to get custom path")
    }
}

#[doc(hidden)]
static HELP: &str = "\
Usage: proton-call [OPTIONS]... EXE [EXTRA]...

Options:
    -c, --custom [PATH]     Path to a directory containing Proton to use
    -h, --help              View this help message
    -i, --index             View an index of installed Proton versions
    -l, --log               Pass PROTON_LOG variable to Proton
    -p, --proton [VERSION]  Use Proton VERSION from `common`
    -r, --run EXE           Run EXE in proton
    -v, --verbose           Run in verbose mode
    -V, --version           View version information

Config:
    The config file should be located at '$XDG_CONFIG_HOME/proton.conf' or '$HOME/.config/proton.conf'
    The config requires two values.
    Data: a location to any directory to contain Proton's runtime files.
    Steam: the directory to where steam is installed (the one which contains the steamapps directory).
    Common: the directory to where your proton versions are stored, usually Steam's steamapps/common directory.
    Example:
        data = \"/home/avery/Documents/Proton/env/\"
        steam = \"/home/avery/.steam/steam/\"
        common = \"/home/avery/.steam/steam/steamapps/common/\"
";

#[doc(hidden)]
fn help() {
    println!("{}", HELP);
}

#[doc(hidden)]
fn version() {
    println!(
        "Proton Caller (proton-call) {} Copyright (C) 2021 {}",
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS")
    );
}