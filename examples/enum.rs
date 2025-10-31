use serde::Deserialize;

#[derive(Debug)]
pub enum AuditEndpointError { AuditLogActionNotSupported(i32) }
impl std::error::Error for AuditEndpointError {}
impl std::fmt::Display for AuditEndpointError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "AuditEndpointError: {}", self.to_string())
  }
}

#[derive(Debug)]
pub enum AuditLogAction { 
  LoggedIn,
  LoggedOut,
}
impl TryFrom<i32> for AuditLogAction {
  type Error = AuditEndpointError;
  fn try_from(value: i32) -> Result<Self, Self::Error> {
    Ok(match value {
      0 => Self::LoggedIn,
      1 => Self::LoggedOut,
      _ => return Err(AuditEndpointError::AuditLogActionNotSupported(value))
    })
  }
}
impl<'de> Deserialize<'de> for AuditLogAction {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where D: serde::Deserializer<'de> {
    let value = i32::deserialize(deserializer)?;
    AuditLogAction::try_from(value).map_err(serde::de::Error::custom)
  }
}

#[tokio::main]
async fn main() {
  let content = r#"[0, 1]"#;
  let content: Vec<AuditLogAction> = serde_json::from_str(content).unwrap();
  println!("{content:?}");
}