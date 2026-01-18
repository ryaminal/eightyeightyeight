use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// A trait for resolving sensitive configuration values.
pub trait SecretResolver {
    /// Resolves the secret to a string.
    fn resolve(&self) -> Result<String>;
}

/// Resolves a secret from a literal string.
/// Supports both raw hex strings and "literal:HEX" format for explicit typing.
pub struct LiteralResolver {
    value: String,
}

impl LiteralResolver {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl SecretResolver for LiteralResolver {
    fn resolve(&self) -> Result<String> {
        Ok(self
            .value
            .strip_prefix("literal:")
            .unwrap_or(&self.value)
            .to_string())
    }
}

/// Resolves a secret from an environment variable.
/// Format: "env:VAR_NAME"
pub struct EnvVarResolver {
    var_name: String,
}

impl EnvVarResolver {
    pub fn new(var_name: impl Into<String>) -> Self {
        Self {
            var_name: var_name.into(),
        }
    }
}

impl SecretResolver for EnvVarResolver {
    fn resolve(&self) -> Result<String> {
        std::env::var(&self.var_name)
            .with_context(|| format!("Failed to read environment variable: {}", self.var_name))
    }
}

/// Resolves a secret from a file.
/// Format: "file:/path/to/secret"
pub struct FileResolver {
    path: String,
}

impl FileResolver {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}

impl SecretResolver for FileResolver {
    fn resolve(&self) -> Result<String> {
        let path = Path::new(&self.path);
        fs::read_to_string(path)
            .with_context(|| format!("Failed to read secret file: {:?}", path))
            .map(|s| s.trim().to_string())
    }
}

/// Resolves a secret from AWS Secrets Manager.
/// Format: "aws:REGION:SECRET_ID"
pub struct AwsSecretManagerResolver {
    region: String,
    secret_id: String,
}

impl AwsSecretManagerResolver {
    pub fn new(source: &str) -> Self {
        let parts: Vec<&str> = source.splitn(2, ':').collect();
        let region = parts.get(0).unwrap_or(&"us-east-1").to_string();
        let secret_id = parts.get(1).unwrap_or(&"").to_string();
        Self { region, secret_id }
    }
}

impl SecretResolver for AwsSecretManagerResolver {
    fn resolve(&self) -> Result<String> {
        // TODO: Implement AWS Secrets Manager integration using aws-sdk-secretsmanager
        Err(anyhow::anyhow!(
            "AWS Secrets Manager resolution not yet implemented. Region: {}, Secret: {}",
            self.region,
            self.secret_id
        ))
    }
}

/// Resolves a secret from HashiCorp Vault.
/// Format: "vault:PATH:KEY"
pub struct VaultResolver {
    path: String,
    key: String,
}

impl VaultResolver {
    pub fn new(source: &str) -> Self {
        let parts: Vec<&str> = source.splitn(2, ':').collect();
        let path = parts.get(0).unwrap_or(&"").to_string();
        let key = parts.get(1).unwrap_or(&"").to_string();
        Self { path, key }
    }
}

impl SecretResolver for VaultResolver {
    fn resolve(&self) -> Result<String> {
        // TODO: Implement HashiCorp Vault integration
        Err(anyhow::anyhow!(
            "HashiCorp Vault resolution not yet implemented. Path: {}, Key: {}",
            self.path,
            self.key
        ))
    }
}

/// Factory function to create the appropriate resolver based on the input string.
pub fn get_resolver(source: &str) -> Box<dyn SecretResolver> {
    if let Some(var_name) = source.strip_prefix("env:") {
        Box::new(EnvVarResolver::new(var_name))
    } else if let Some(path) = source.strip_prefix("file:") {
        Box::new(FileResolver::new(path))
    } else if let Some(aws_src) = source.strip_prefix("aws:") {
        Box::new(AwsSecretManagerResolver::new(aws_src))
    } else if let Some(vault_src) = source.strip_prefix("vault:") {
        Box::new(VaultResolver::new(vault_src))
    } else {
        // Default to literal if no prefix or explicit "literal:" prefix
        Box::new(LiteralResolver::new(source))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_literal_resolver() {
        let resolver = LiteralResolver::new("deadbeef");
        assert_eq!(resolver.resolve().unwrap(), "deadbeef");

        let resolver_explicit = LiteralResolver::new("literal:deadbeef");
        assert_eq!(resolver_explicit.resolve().unwrap(), "deadbeef");
    }

    #[test]
    fn test_env_var_resolver() {
        unsafe {
            std::env::set_var("TEST_SECRET_KEY", "secret_value");
        }
        let resolver = EnvVarResolver::new("TEST_SECRET_KEY");
        assert_eq!(resolver.resolve().unwrap(), "secret_value");
        unsafe {
            std::env::remove_var("TEST_SECRET_KEY");
        }
    }

    #[test]
    fn test_env_var_resolver_missing() {
        let resolver = EnvVarResolver::new("NON_EXISTENT_VAR");
        assert!(resolver.resolve().is_err());
    }

    #[test]
    fn test_file_resolver() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "file_secret").unwrap();
        let path = file.path().to_str().unwrap();

        let resolver = FileResolver::new(path);
        assert_eq!(resolver.resolve().unwrap(), "file_secret");
    }

    #[test]
    fn test_file_resolver_trim_whitespace() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "  file_secret_trimmed  \n").unwrap();
        let path = file.path().to_str().unwrap();

        let resolver = FileResolver::new(path);
        assert_eq!(resolver.resolve().unwrap(), "file_secret_trimmed");
    }

    #[test]
    fn test_get_resolver_factory() {
        let env_resolver = get_resolver("env:MY_VAR");
        // We can't easily check the type of a trait object, but we can verify behavior
        // (or just trust the factory logic which is simple enough)
        
        // Quick integration check
        unsafe {
            std::env::set_var("MY_VAR", "my_val");
        }
        assert_eq!(env_resolver.resolve().unwrap(), "my_val");
        unsafe {
            std::env::remove_var("MY_VAR");
        }

        let lit_resolver = get_resolver("literal:foo");
        assert_eq!(lit_resolver.resolve().unwrap(), "foo");
        
        let implicit_lit = get_resolver("bar");
        assert_eq!(implicit_lit.resolve().unwrap(), "bar");

        let aws_resolver = get_resolver("aws:us-west-2:my-secret");
        assert!(aws_resolver.resolve().is_err());

        let vault_resolver = get_resolver("vault:secret/data/myapp:api_key");
        assert!(vault_resolver.resolve().is_err());
    }
}
