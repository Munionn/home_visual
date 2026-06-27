pub fn is_valid_email(email: &str) -> bool {
    if email.len() < 3 || email.len() > 320 {
        return false;
    }

    let mut parts = email.split('@');
    let local = match parts.next() {
        Some(p) => p,
        None => return false,
    };
    let domain = match parts.next() {
        Some(p) => p,
        None => return false,
    };
    if parts.next().is_some() {
        return false;
    }

    if local.is_empty() || local.len() > 64 {
        return false;
    }
    if local.starts_with('.') || local.ends_with('.') || local.contains("..") {
        return false;
    }

    if domain.is_empty() || domain.len() > 255 {
        return false;
    }
    if domain.starts_with('.') || domain.ends_with('.') || domain.contains("..") {
        return false;
    }

    if !domain.contains('.') {
        return false;
    }

    for label in domain.split('.') {
        if label.is_empty() {
            return false;
        }
        if !label.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return false;
        }
        if !label
            .chars()
            .next()
            .map(|c| c.is_ascii_alphanumeric())
            .unwrap_or(false)
        {
            return false;
        }
        if !label
            .chars()
            .last()
            .map(|c| c.is_ascii_alphanumeric())
            .unwrap_or(false)
        {
            return false;
        }
    }

    true
}
pub fn validate_room_dimensions(width: f64, height: f64) -> Result<(), String> {
    if width <= 0.0 || height <= 0.0 {
        return Err("Dimensions must be positive values.".to_string());
    }
    if width > 100.0 || height > 100.0 {
        return Err("Dimensions cannot exceed 100 meters.".to_string());
    }
    Ok(())
}

pub fn validate_coordinates(x: f64, y: f64) -> Result<(), String> {
    if x.is_nan() || y.is_nan() || x.is_infinite() || y.is_infinite() {
        return Err("Coordinates must be valid finite numbers.".to_string());
    }
    Ok(())
}
