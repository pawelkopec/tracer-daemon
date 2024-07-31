use anyhow::Result;
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

use crate::config_manager::{Target, TargetMatch};

const TRACER_BASH_RC_PATH: &str = ".config/tracer/.bashrc";
const WRAPPER_SOURCE_COMMAND: &str = "source ~/.config/tracer/.bashrc";

pub fn get_task_wrapper(
    current_tracer_exe_path: PathBuf,
    command_name: &str,
    display_name: &str,
) -> String {
    format!(
        "alias {}=\"{} & {} log-short-lived-process \\\"{}\\\"; wait\"\n",
        command_name,
        command_name,
        current_tracer_exe_path.as_os_str().to_str().unwrap(),
        display_name,
    )
}

pub fn rewrite_wrapper_bashrc_file(
    current_tracer_exe_path: PathBuf,
    targets: Vec<&Target>,
) -> Result<()> {
    let path = homedir::get_my_home()?.unwrap();

    let mut bashrc_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path.join(TRACER_BASH_RC_PATH))?;

    for command in targets.into_iter().map(|target| {
        let name = target.get_display_name();
        let command_to_alias = match &target.match_type {
            TargetMatch::ShortLivedProcessExecutable(alias) => alias.clone(),
            _ => "unknown_command".to_string(),
        };
        format!(
            "{}\n",
            get_task_wrapper(
                current_tracer_exe_path.clone(),
                &command_to_alias,
                &name.unwrap_or(command_to_alias.clone())
            )
        )
    }) {
        bashrc_file.write_all(command.as_bytes()).unwrap();
    }

    Ok(())
}

pub fn modify_bashrc_file(bashrc_file_path: &str) -> Result<()> {
    let path = homedir::get_my_home()?.unwrap();

    let mut bashrc_file = OpenOptions::new()
        .read(true)
        .append(true)
        .open(path.join(bashrc_file_path))?;

    let reader = BufReader::new(&bashrc_file);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains(WRAPPER_SOURCE_COMMAND) {
            return Ok(());
        }
    }

    bashrc_file
        .write_all(WRAPPER_SOURCE_COMMAND.as_bytes())
        .unwrap();

    Ok(())
}