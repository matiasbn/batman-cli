use std::fs;

use colored::Colorize;
use walkdir::WalkDir;

use crate::{
    commands::tm,
    config::BatConfig,
    structs::FileInfo,
    utils::{self, helpers},
};

pub fn update_accounts() -> Result<(), String> {
    let program_state_folder_path = BatConfig::get_validated_config()?
        .optional
        .program_state_path;
    // read the state folder
    let state_folder_files_info =
        helpers::get::get_only_files_from_folder(program_state_folder_path)?;
    // get the structs of every file
    let state_structs = helpers::get::get_structs_in_files(state_folder_files_info)?;
    let mut account_structs: Vec<String> = vec![];
    let mut not_account_structs: Vec<String> = vec![];
    for state_struct in state_structs {
        let formatted_to_rust_comment = helpers::format::format_to_rust_comment(&state_struct);
        let is_account = prompt_if_account(state_struct.clone())?;
        if is_account {
            account_structs.push(formatted_to_rust_comment);
        } else {
            not_account_structs.push(formatted_to_rust_comment);
        }
    }
    let tm_file_path = BatConfig::get_auditor_threat_modeling_path()?;
    println!("tm path {}", tm_file_path);
    helpers::parse::parse_lines_between_two_strings_in_file(
        tm_file_path,
        account_structs.join("\n").as_str(),
        "### Accounts",
        "### Other",
    )?;
    Ok(())
}

fn prompt_if_account(state_struct: String) -> Result<bool, String> {
    let prompt_text = format!(
        "Is this struct a {}?: \n{}",
        format!("Solana account").red(),
        format!("{state_struct}").green()
    );
    let decision = utils::cli_inputs::select_yes_or_no(&prompt_text)?;
    Ok(decision)
}
