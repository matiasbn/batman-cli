use std::{fs, path::Path, str};

use serde::Deserialize;

use crate::commands::{
    create::{create_auditor_toml, AUDITOR_TOML_INITIAL_PATH, BAT_TOML_INITIAL_PATH},
    miro::MiroConfig,
};

use crate::utils::helpers::canonicalize_path;

#[derive(Debug, Deserialize, Clone)]
pub struct BatConfig {
    pub required: RequiredConfig,
    pub optional: OptionalConfig,
    pub auditor: AuditorConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RequiredConfig {
    pub project_name: String,
    pub client_name: String,
    pub commit_hash_url: String,
    pub starting_date: String,
    pub miro_board_url: String,
    pub miro_board_id: String,
    pub auditor_names: Vec<String>,
    pub audit_folder_path: String,
    pub program_lib_path: String,
    pub project_repository_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OptionalConfig {
    pub program_instructions_path: String,
    pub program_state_path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuditorConfig {
    pub auditor_name: String,
    pub miro_oauth_access_token: String,
    pub vs_code_integration: bool,
}

impl BatConfig {
    pub fn get_validated_config() -> Result<BatConfig, String> {
        let bat_config = Self::get_bat_config().unwrap();
        Self::validate_bat_config(bat_config.clone(), true)?;
        Ok(bat_config)
    }

    pub fn get_init_config() -> Result<BatConfig, String> {
        let bat_config: BatConfig = Self::get_bat_config().unwrap();
        Self::validate_bat_config(bat_config.clone(), false)?;
        Ok(bat_config)
    }

    fn get_bat_config() -> Result<BatConfig, String> {
        // Bat.toml
        let bat_toml_path = Path::new(&BAT_TOML_INITIAL_PATH);
        if !bat_toml_path.is_file() {
            panic!("Bat.toml file not found at {bat_toml_path:?}");
        }
        let bat_toml_file = fs::read(bat_toml_path).unwrap();
        let bat_tom_file_string = str::from_utf8(bat_toml_file.as_slice()).unwrap();

        // BatAuditor.toml
        let auditor_toml_path = Path::new(&AUDITOR_TOML_INITIAL_PATH);
        if !auditor_toml_path.is_file() {
            // if BatAuditor does not exist, create it
            create_auditor_toml();
            println!("BatAuditor.toml file not detected, creating")
        }
        let auditor_toml_file = fs::read(auditor_toml_path).unwrap();
        let auditor_tom_file_string = str::from_utf8(auditor_toml_file.as_slice()).unwrap();

        // Get the BatConfig complete
        let config: BatConfig =
            toml::from_str((bat_tom_file_string.to_string() + auditor_tom_file_string).as_str())
                .unwrap();
        Ok(config)
    }

    fn validate_bat_config(bat_config: BatConfig, validate_auditor: bool) -> Result<(), String> {
        let BatConfig {
            required, auditor, ..
        } = bat_config;
        // Validate required
        if required.project_name.is_empty() {
            panic!("required parameter project_name is empty at Bat.toml");
        }
        if required.client_name.is_empty() {
            panic!("required parameter client_name is empty at Bat.toml");
        }
        if required.commit_hash_url.is_empty() {
            panic!("required parameter commit_hash_url is empty at Bat.toml");
        }
        if required.starting_date.is_empty() {
            panic!("required parameter starting_date is empty at Bat.toml");
        }
        if required.miro_board_url.is_empty() {
            panic!("required parameter miro_board_url is empty at Bat.toml");
        }
        if required.program_lib_path.is_empty() {
            panic!("required parameter program_lib_path is empty at Bat.toml");
        }
        if required.audit_folder_path.is_empty() {
            panic!("required parameter audit_folder_path is empty at Bat.toml");
        }
        if required.auditor_names.is_empty() {
            panic!("required parameter auditor_names is empty at Bat.toml");
        }
        if required.project_repository_url.is_empty() {
            panic!("required parameter project_repository_url is empty at Bat.toml",);
        }

        // Validate auditor
        if validate_auditor && auditor.auditor_name.is_empty() {
            panic!("required parameter auditor_name is empty at BatAuditor.toml");
        }
        Ok(())
    }

    pub fn get_auditor_name() -> Result<String, String> {
        let auditor_name = Self::get_validated_config()?.auditor.auditor_name;
        Ok(auditor_name)
    }
}

// pub trait TestConfig {
//     fn get_test_config() -> BatConfig;
// }

// impl TestConfig for BatConfig {
//     fn get_test_config() -> BatConfig {
//         let required = RequiredConfig {
//             project_name: "test_project".to_string(),
//             auditor_names: vec!["matias".to_string(), "porter".to_string()],
//             audit_folder_path: "../audit-notes".to_string(),
//             program_lib_path:
//                 "../star-atlas-programs/sol-programs/scream/programs/player_profile/src/lib.rs"
//                     .to_string(),
//             project_repository_url: "git@git.kudelski.com:Matias.Barrios/test_project.git"
//                 .to_string(),
//             miro_board_url: "https://miro.com/app/board/".to_string(),
//             miro_board_id: "uXjVPzsgmiY=".to_string(),
//             client_name: "client_name".to_string(),
//             commit_hash_url: "example.miro.url".to_string(),
//             starting_date: "01/01/2023".to_string(),
//         };
//         let optional = OptionalConfig {
//             program_instructions_path: "".to_string(),
//         };
//         let auditor = AuditorConfig {
//             auditor_name: "matias".to_string(),
//             miro_oauth_access_token: "!".to_string(),
//             vs_code_integration: true,
//         };

//         BatConfig {
//             required,
//             optional,
//             auditor,
//         }
//     }
// }

// #[test]
// fn test_get_test_config() {
//     let batconfig = BatConfig::get_test_config();
//     println!("{batconfig:#?}");
// }
