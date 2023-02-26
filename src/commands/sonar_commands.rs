use crate::batbelt;
use crate::batbelt::bat_dialoguer::BatDialoguer;
use crate::batbelt::command_line::execute_command;
use crate::batbelt::git::GitCommit;
use crate::batbelt::metadata::functions_metadata::FunctionMetadata;
use crate::batbelt::metadata::structs_metadata::StructMetadata;
use crate::batbelt::metadata::traits_metadata::TraitMetadata;
use crate::batbelt::metadata::{BatMetadataParser, BatMetadataType};
use crate::batbelt::path::{BatFile, BatFolder};
use clap::Subcommand;
use colored::Colorize;
use error_stack::{Result, ResultExt};
use std::path::Path;

use crate::batbelt::sonar::{BatSonar, SonarResultType};
use crate::batbelt::templates::TemplateGenerator;

use super::CommandError;

#[derive(Subcommand, Debug, strum_macros::Display)]
pub enum SonarCommand {
    /// Updates the functions.md and structs.md files with data
    Run,
    /// Gets the path to a metadata information from metadata files
    PrintPath {
        /// select all options as true
        #[arg(short, long)]
        select_all: bool,
    },
}

impl SonarCommand {
    pub fn execute_command(&self) -> Result<(), CommandError> {
        match self {
            SonarCommand::Run => self.execute_run(),
            SonarCommand::PrintPath { select_all } => self.execute_print_path(*select_all),
        }
    }

    fn execute_print_path(&self, select_all: bool) -> Result<(), CommandError> {
        let mut continue_printing = true;
        while continue_printing {
            let selected_bat_metadata_type =
                BatMetadataType::prompt_metadata_type_selection().change_context(CommandError)?;
            match selected_bat_metadata_type {
                BatMetadataType::Struct => {
                    let selections = StructMetadata::prompt_multiselection(select_all, true)
                        .change_context(CommandError)?;
                    for selection in selections {
                        self.print_formatted_path(
                            selection.name,
                            selection.path,
                            selection.start_line_index,
                        )
                    }
                }
                BatMetadataType::Function => {
                    let selections = FunctionMetadata::prompt_multiselection(select_all, true)
                        .change_context(CommandError)?;
                    for selection in selections {
                        self.print_formatted_path(
                            selection.name,
                            selection.path,
                            selection.start_line_index,
                        )
                    }
                }
                BatMetadataType::Trait => {
                    let selections = TraitMetadata::prompt_multiselection(select_all, true)
                        .change_context(CommandError)?;
                    for selection in selections {
                        self.print_formatted_path(
                            selection.name,
                            selection.path,
                            selection.start_line_index,
                        )
                    }
                }
            }
            let prompt_text = format!("Do you want to continute {}", "printing paths?".yellow());
            continue_printing = BatDialoguer::select_yes_or_no(prompt_text)?;
        }
        Ok(())
    }

    fn print_formatted_path(&self, name: String, path: String, start_line_index: usize) {
        println!(
            "{}: {}:{}",
            name.blue(),
            path.trim_start_matches("../"),
            start_line_index
        )
    }

    fn execute_run(&self) -> Result<(), CommandError> {
        let metadata_path = BatFolder::Metadata
            .get_path(false)
            .change_context(CommandError)?;
        execute_command("rm", &["-rf", &metadata_path])?;
        execute_command("mkdir", &[&metadata_path])?;
        TemplateGenerator::create_auditor_metadata_files().change_context(CommandError)?;
        BatSonar::display_looking_for_loader(SonarResultType::Struct);
        self.structs()?;
        BatSonar::display_looking_for_loader(SonarResultType::Function);
        self.functions()?;
        BatSonar::display_looking_for_loader(SonarResultType::Trait);
        self.traits()?;
        Ok(())
    }

    fn functions(&self) -> Result<(), CommandError> {
        let mut functions_metadata_markdown = BatMetadataType::Function
            .get_markdown()
            .change_context(CommandError)?;
        let functions_metadata =
            FunctionMetadata::get_metadata_from_program_files().change_context(CommandError)?;
        let functions_markdown_content = functions_metadata
            .into_iter()
            .map(|function_metadata| function_metadata.get_markdown_section_content_string())
            .collect::<Vec<_>>()
            .join("\n\n");
        functions_metadata_markdown.content = functions_markdown_content;
        functions_metadata_markdown
            .save()
            .change_context(CommandError)?;
        batbelt::git::create_git_commit(
            GitCommit::UpdateMetadata {
                metadata_type: BatMetadataType::Function,
            },
            None,
        )
        .unwrap();
        Ok(())
    }

    fn structs(&self) -> Result<(), CommandError> {
        let mut structs_metadata_markdown = BatMetadataType::Struct
            .get_markdown()
            .change_context(CommandError)?;
        let structs_metadata =
            StructMetadata::get_metadata_from_program_files().change_context(CommandError)?;
        let structs_markdown_content = structs_metadata
            .into_iter()
            .map(|struct_metadata| struct_metadata.get_markdown_section_content_string())
            .collect::<Vec<_>>()
            .join("\n\n");
        structs_metadata_markdown.content = structs_markdown_content;
        structs_metadata_markdown
            .save()
            .change_context(CommandError)?;
        batbelt::git::create_git_commit(
            GitCommit::UpdateMetadata {
                metadata_type: BatMetadataType::Struct,
            },
            None,
        )
        .unwrap();
        Ok(())
    }
    fn traits(&self) -> Result<(), CommandError> {
        let mut traits_metadata_markdown = BatMetadataType::Trait
            .get_markdown()
            .change_context(CommandError)?;
        let traits_metadata =
            TraitMetadata::get_metadata_from_program_files().change_context(CommandError)?;
        let traits_markdown_content = traits_metadata
            .into_iter()
            .map(|struct_metadata| struct_metadata.get_markdown_section_content_string())
            .collect::<Vec<_>>()
            .join("\n\n");
        traits_metadata_markdown.content = traits_markdown_content;
        traits_metadata_markdown
            .save()
            .change_context(CommandError)?;
        batbelt::git::create_git_commit(
            GitCommit::UpdateMetadata {
                metadata_type: BatMetadataType::Trait,
            },
            None,
        )
        .unwrap();
        Ok(())
    }
}
