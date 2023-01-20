// Code overhaul template file
pub const CODE_OVERHAUL_WHAT_IT_DOES_PLACEHOLDER: &str = "WHAT_IT_DOES_HERE";
pub const CODE_OVERHAUL_NOTES_PLACEHOLDER: &str = "NOTES_HERE";
pub const CODE_OVERHAUL_MIRO_FRAME_LINK_PLACEHOLDER: &str = "MIRO_FRAME_LINK_PLACEHOLDER";
pub const CODE_OVERHAUL_ENTRYPOINT_PLACEHOLDER: &str = "ENTRYPOINT_PLACEHOLDER";
pub const CODE_OVERHAUL_CONTEXT_ACCOUNT_PLACEHOLDER: &str = "CONTEXT_ACCOUNT_PLACEHOLDER";
pub const CODE_OVERHAUL_VALIDATIONS_PLACEHOLDER: &str = "VALIDATIONS_PLACEHOLDER";
pub const CODE_OVERHAUL_HANDLER_PLACEHOLDER: &str = "HANDLER_PLACEHOLDER";
pub const CODE_OVERHAUL_SIGNERS_DESCRIPTION_PLACEHOLDER: &str = "SIGNERS_DESCRIPTION_PLACEHOLDER";
pub const CODE_OVERHAUL_EMPTY_SIGNER_PLACEHOLDER: &str = "ADD_A_DESCRIPTION_FOR_THIS_SIGNER";
pub const CODE_OVERHAUL_CONTEXT_ACCOUNTS_PLACEHOLDER: &str = "CONTEXT_ACCOUNTS_PLACEHOLDER";
pub const CODE_OVERHAUL_FUNCTION_PARAMETERS_PLACEHOLDER: &str = "FUNCTION_PARAMETER_PLACEHOLDER";
pub const CODE_OVERHAUL_NO_FUNCTION_PARAMETERS_FOUND_PLACEHOLDER: &str =
    "NO_FUNCTION_PARAMETERS_FOUND";
pub const CODE_OVERHAUL_ACCOUNTS_VALIDATION_PLACEHOLDER: &str = "ACCOUNTS_VALIDATIONS_PLACEHOLDER";
pub const CODE_OVERHAUL_PREREQUISITES_PLACEHOLDER: &str = "PREREQUISITES_PLACEHOLDER";
pub const CODE_OVERHAUL_NO_VALIDATION_FOUND_PLACEHOLDER: &str = "NO_VALIDATIONS_FOUND";

// Audit information file
pub const AUDIT_INFORMATION_PROJECT_NAME_PLACEHOLDER: &str =
    "AUDIT_INFORMATION_PROJECT_NAME_PLACEHOLDER";
pub const AUDIT_INFORMATION_CLIENT_NAME_PLACEHOLDER: &str =
    "AUDIT_INFORMATION_CLIENT_NAME_PLACEHOLDER";
pub const AUDIT_INFORMATION_COMMIT_HASH_PLACEHOLDER: &str =
    "AUDIT_INFORMATION_COMMIT_HASH_PLACEHOLDER";
pub const AUDIT_INFORMATION_MIRO_BOARD_PLACEHOLER: &str = "AUDIT_INFORMATION_MIRO_BOARD_PLACEHOLER";
pub const AUDIT_INFORMATION_STARTING_DATE_PLACEHOLDER: &str =
    "AUDIT_INFORMATION_STARTING_DATE_PLACEHOLDER";

// Audit information file
pub const AUDIT_RESULT_FILE_NAME: &str = "audit_result.md";

// Base repository
pub const BASE_REPOSTORY_URL: &str = "git@github.com:matiasbn/bat-base-repository.git";
pub const BASE_REPOSTORY_NAME: &str = "bat-base-repository";

pub const BAT_TOML_INITIAL_CONFIG_STR: &str = r#"
[required]
auditor_names = [""]
project_name = ""
client_name = ""
commit_hash_url = ""
starting_date = ""
audit_folder_path = "."
program_lib_path = ""
project_repository_url = ""
miro_board_url = ""
miro_board_id = ""
[optional]
program_instructions_path = ""
"#;
pub const AUDITOR_TOML_INITIAL_CONFIG_STR: &str = r#"
[auditor]
auditor_name = ""
miro_oauth_access_token = ""
vs_code_integration = "false"
"#;

pub const ENTRYPOINT_PNG_NAME: &str = "entrypoint.png";
pub const HANDLER_PNG_NAME: &str = "handler.png";
pub const CONTEXT_ACCOUNTS_PNG_NAME: &str = "context_accounts.png";
pub const VALIDATIONS_PNG_NAME: &str = "validations.png";

pub static CO_FIGURES: &[&str] = &[
    ENTRYPOINT_PNG_NAME,
    HANDLER_PNG_NAME,
    CONTEXT_ACCOUNTS_PNG_NAME,
    VALIDATIONS_PNG_NAME,
];

// miro config
pub const MIRO_FRAME_WIDTH: i32 = 3392;
pub const MIRO_FRAME_HEIGHT: i32 = 1908;
pub const MIRO_BOARD_COLUMNS: i32 = 7;
pub const MIRO_INITIAL_X: i32 = 4800;
pub const MIRO_INITIAL_Y: i32 = 0;
