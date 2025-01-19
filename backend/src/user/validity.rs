use super::User;

const HANDLE_BOUND_LOWER: usize = 3;
const HANDLE_BOUND_UPPER: usize = 24;
const PASSWORD_LEN_BOUND_LOWER: usize = 8;
const PASSWORD_LEN_BOUND_UPPER: usize = 96;

#[derive(Debug, thiserror::Error)]
pub enum ValidityError {
    #[error("Handle must be between {HANDLE_BOUND_LOWER} and {HANDLE_BOUND_UPPER} characters ({HANDLE_BOUND_LOWER}..={HANDLE_BOUND_UPPER})")]
    HandleLengthInvalid,
    #[error("Handle must be entirely letters, digits or special characters (dots, hyphens, underscores).")]
    HandleInvalidChars,
    #[error("Handle must not have leading or trailing special characters.")]
    HandleLeadingTrailingSpecialChars,
    #[error("Handle must not have consecutive special characters.")]
    HandleConsecutiveSpecialChars,

    #[error("Password must be between {PASSWORD_LEN_BOUND_LOWER} and {PASSWORD_LEN_BOUND_UPPER} characters ({PASSWORD_LEN_BOUND_LOWER}..={PASSWORD_LEN_BOUND_UPPER})")]
    PasswordLengthInvalid,
}

impl User {
    pub fn is_valid_handle(handle: &str) -> Result<(), ValidityError> {
        use ValidityError as VA;

        if !str_within_bounds(handle, HANDLE_BOUND_LOWER, HANDLE_BOUND_UPPER) {
            return Err(VA::HandleLengthInvalid);
        }

        if !str_only_valid_chars(handle) {
            return Err(VA::HandleInvalidChars);
        }

        if !str_no_lead_trail_special_chars(handle) {
            return Err(VA::HandleLeadingTrailingSpecialChars);
        }

        if !str_no_consecutive_special_chars(handle) {
            return Err(VA::HandleConsecutiveSpecialChars);
        }

        Ok(())
    }
    pub fn is_valid_password(password: &str) -> Result<(), ValidityError> {
        use ValidityError as VA;

        if !str_within_bounds(password, PASSWORD_LEN_BOUND_LOWER, PASSWORD_LEN_BOUND_UPPER) {
            return Err(VA::PasswordLengthInvalid);
        }

        Ok(())
    }
}

#[inline]
fn str_within_bounds(str: &str, lower_bound: usize, upper_bound: usize) -> bool {
    str.len() >= lower_bound && str.len() <= upper_bound
}

/// is dot, underscore or hyphen
#[inline]
fn is_duh(char: char) -> bool {
    char == '.' || char == '_' || char == '-'
}

#[inline]
fn str_only_valid_chars(str: &str) -> bool {
    str.chars().all(|c| c.is_ascii_alphanumeric() || is_duh(c))
}

#[inline]
fn str_no_lead_trail_special_chars(str: &str) -> bool {
    let mut iter = str.chars();
    if let Some(char) = iter.next() {
        if is_duh(char) {
            return false;
        }
    }
    if let Some(char) = iter.last() {
        if is_duh(char) {
            return false;
        }
    }

    true
}

#[inline]
fn str_no_consecutive_special_chars(str: &str) -> bool {
    let chars: Vec<char> = str.chars().collect();
    chars
        .windows(2)
        .all(|pair| !(is_duh(pair[0]) && is_duh(pair[1])))
}
