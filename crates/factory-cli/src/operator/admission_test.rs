use anyhow::Result;
use regex::Regex;

/// Quantity parser for memory and CPU bounds matching Kubernetes CEL rules
pub struct QuantityValidator;

impl QuantityValidator {
    /// Validates maxRam quantity: must be > 0 and <= 30Mi (31457280 bytes)
    pub fn validate_max_ram(val: &str) -> Result<bool> {
        let val = val.trim();
        if val.is_empty() || val.len() > 16 {
            return Ok(false);
        }
        match Self::parse_memory_bytes(val) {
            Ok(bytes) => Ok(bytes > 0 && bytes <= 31_457_280),
            Err(_) => Ok(false),
        }
    }

    /// Validates maxCpu quantity: must be > 0 and <= 250m (250 millicores)
    pub fn validate_max_cpu(val: &str) -> Result<bool> {
        let val = val.trim();
        if val.is_empty() || val.len() > 16 {
            return Ok(false);
        }
        match Self::parse_cpu_millis(val) {
            Ok(millis) => Ok(millis > 0 && millis <= 250),
            Err(_) => Ok(false),
        }
    }

    fn parse_memory_bytes(val: &str) -> Result<i64> {
        let re = Regex::new(r"^(\d+)(Ki|Mi|Gi|m|k|M|G)?$")?;
        let caps = re
            .captures(val)
            .ok_or_else(|| anyhow::anyhow!("Invalid memory format"))?;
        let num: i64 = caps[1].parse()?;
        let unit = caps.get(2).map(|m| m.as_str()).unwrap_or("");
        let bytes = match unit {
            "" => num,
            "Ki" => num * 1024,
            "Mi" => num * 1024 * 1024,
            "Gi" => num * 1024 * 1024 * 1024,
            "k" => num * 1000,
            "M" => num * 1000 * 1000,
            "G" => num * 1000 * 1000 * 1000,
            _ => anyhow::bail!("Unsupported unit"),
        };
        Ok(bytes)
    }

    fn parse_cpu_millis(val: &str) -> Result<i64> {
        if let Some(stripped) = val.strip_suffix('m') {
            let millis: i64 = stripped.parse()?;
            Ok(millis)
        } else {
            let cores: f64 = val.parse()?;
            Ok((cores * 1000.0).round() as i64)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_memory_bounds() {
        assert!(QuantityValidator::validate_max_ram("30Mi").unwrap());
        assert!(QuantityValidator::validate_max_ram("20Mi").unwrap());
        assert!(QuantityValidator::validate_max_ram("1024Ki").unwrap());
        assert!(QuantityValidator::validate_max_ram("1048576").unwrap()); // 1Mi in bytes
    }

    #[test]
    fn test_invalid_memory_bounds() {
        assert!(!QuantityValidator::validate_max_ram("31Mi").unwrap());
        assert!(!QuantityValidator::validate_max_ram("64Mi").unwrap());
        assert!(!QuantityValidator::validate_max_ram("0").unwrap());
        assert!(!QuantityValidator::validate_max_ram("-10Mi").unwrap());
        assert!(!QuantityValidator::validate_max_ram("invalid").unwrap());
    }

    #[test]
    fn test_valid_cpu_bounds() {
        assert!(QuantityValidator::validate_max_cpu("250m").unwrap());
        assert!(QuantityValidator::validate_max_cpu("100m").unwrap());
        assert!(QuantityValidator::validate_max_cpu("0.25").unwrap());
        assert!(QuantityValidator::validate_max_cpu("0.1").unwrap());
    }

    #[test]
    fn test_invalid_cpu_bounds() {
        assert!(!QuantityValidator::validate_max_cpu("251m").unwrap());
        assert!(!QuantityValidator::validate_max_cpu("500m").unwrap());
        assert!(!QuantityValidator::validate_max_cpu("1").unwrap()); // 1000m
        assert!(!QuantityValidator::validate_max_cpu("0").unwrap());
        assert!(!QuantityValidator::validate_max_cpu("-50m").unwrap());
    }
}
