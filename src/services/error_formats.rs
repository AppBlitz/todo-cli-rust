use format;
pub fn format_error_command(message: &str, erro: &str, help: &str) -> String {
    format!(
        r#"{{"Message":"{}","error":"{}","help":"{}"}}"#,
        message, erro, help
    )
}
