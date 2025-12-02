use validator::ValidationError;

pub fn validate_stack(stack: &Vec<String>) -> Result<(), ValidationError> {
    for item in stack {
        if item.len() > 32 {
            let mut err = ValidationError::new("length");
            err.add_param("value".into(), &item);
            return Err(err);
        }
    }
    Ok(())
}
