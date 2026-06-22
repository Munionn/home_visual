use validator::validate_email as val_email;

pub fn validate_email(email: &str) -> bool {
    val_email(email)
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
