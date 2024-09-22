use actix_identity::Identity;

use super::errors::RedHttpError;

pub fn validate_session(identity: Option<Identity>) -> Result<String, RedHttpError> {
    let uuid = identity
        .ok_or_else(|| RedHttpError::SessionError)?
        .id()
        .map_err(|_e| RedHttpError::SessionError)?;
    Ok(uuid)
}
