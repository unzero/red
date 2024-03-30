use actix_identity::Identity;

use super::errors::RedHttpError;

fn validate_session(identity: Option<Identity>) -> Result<String, RedHttpError> {
    let uuid = identity.ok_or_else(|| RedHttpError::session_error() )?
        .id().map_err(|_e| RedHttpError::session_error() )?;
    Ok(uuid)
}
