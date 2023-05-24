use serde::Deserialize;

struct CCP {
    servers: Vec<String>,
    path: String,
    logging: bool,
}

#[derive(Deserialize)]
struct Response {
    #[serde(alias = "Content")]
    content: String,
    #[serde(alias = "CreationMethod")]
    creation_method: String,
    #[serde(alias = "Address")]
    address: String,
    #[serde(alias = "Safe")]
    safe: String,
    #[serde(alias = "UserName")]
    username: String,
    #[serde(alias = "Name")]
    object_name: String,
    #[serde(alias = "PolicyID")]
    policy_id: String,
    #[serde(alias = "DeviceType")]
    device_type: String,
    #[serde(alias = "CPMDisabled")]
    cpm_disabled: String,
    #[serde(alias = "Folder")]
    folder: String,
    #[serde(alias = "PasswordChangeInProgress")]
    password_change_in_progress: String,
}

struct ApiOptions {
    safe: Option<String>,
    folder: Option<String>,
    object: Option<String>,
    username: Option<String>,
    address: Option<String>,
    database: Option<String>,
    policy_id: Option<String>,
    reason: Option<String>,
    connection_timeout: Option<u32>,
    query: Option<String>,
    query_format: Option<String>,
    fail_request_on_password_change: Option<bool>,
}

impl CCP {
    fn new(servers: Vec<String>, app_id: String) -> CCP {
        CCP {
            servers,
            path: app_id,
            logging: false,
        }
    }

    fn build_path(path: String, options: Option<ApiOptions>) -> String {
        path
    }
}
