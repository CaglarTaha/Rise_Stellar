use serde_json::Value;


pub fn parse_json(data: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let parsed: Value = serde_json::from_str(data)?;
    Ok(parsed)
}

pub fn format_error_message(message: &str) -> String {
    format!("Error: {}", message)
}
