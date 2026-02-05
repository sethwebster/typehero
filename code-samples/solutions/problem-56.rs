struct SemVer {
    major: u32,
    minor: u32,
    patch: u32,
}

fn parse_semver(version: &str) -> Result<SemVer, String> {
    let parts: Vec<&str> = version.split('.').collect();

    if parts.len() != 3 {
        return Err("Invalid semver format".to_string());
    }

    let major = parts[0].parse().map_err(|_| "Invalid major")?;
    let minor = parts[1].parse().map_err(|_| "Invalid minor")?;
    let patch = parts[2].parse().map_err(|_| "Invalid patch")?;

    Ok(SemVer { major, minor, patch })
}

// Example: parse_semver("1.2.3") => Ok(SemVer { major: 1, minor: 2, patch: 3 })
