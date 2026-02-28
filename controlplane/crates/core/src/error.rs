use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("internal error")]
    Internal(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[error("unauthenticated")]
    Unauthenticated,

    #[error("permission denied")]
    PermissionDenied,

    #[error("already exists: {0}")]
    AlreadyExists(String),
}

impl From<Error> for tonic::Status {
    fn from(err: Error) -> Self {
        match err {
            Error::Internal(msg) => {
                tracing::error!("internal error: {msg}");
                tonic::Status::internal("internal error")
            }
            Error::NotFound(msg) => tonic::Status::not_found(msg),
            Error::InvalidArgument(msg) => tonic::Status::invalid_argument(msg),
            Error::Unauthenticated => tonic::Status::unauthenticated("unauthenticated"),
            Error::PermissionDenied => tonic::Status::permission_denied("permission denied"),
            Error::AlreadyExists(msg) => tonic::Status::already_exists(msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal_error_maps_to_internal_status() {
        let err = Error::Internal("db failed".into());
        let status: tonic::Status = err.into();
        assert_eq!(status.code(), tonic::Code::Internal);
        assert_eq!(status.message(), "internal error");
    }

    #[test]
    fn not_found_error_maps_to_not_found_status() {
        let err = Error::NotFound("organization xyz".into());
        let status: tonic::Status = err.into();
        assert_eq!(status.code(), tonic::Code::NotFound);
        assert_eq!(status.message(), "organization xyz");
    }

    #[test]
    fn invalid_argument_error_maps_to_invalid_argument_status() {
        let err = Error::InvalidArgument("name is required".into());
        let status: tonic::Status = err.into();
        assert_eq!(status.code(), tonic::Code::InvalidArgument);
        assert_eq!(status.message(), "name is required");
    }

    #[test]
    fn unauthenticated_error_maps_to_unauthenticated_status() {
        let err = Error::Unauthenticated;
        let status: tonic::Status = err.into();
        assert_eq!(status.code(), tonic::Code::Unauthenticated);
    }

    #[test]
    fn permission_denied_error_maps_to_permission_denied_status() {
        let err = Error::PermissionDenied;
        let status: tonic::Status = err.into();
        assert_eq!(status.code(), tonic::Code::PermissionDenied);
    }

    #[test]
    fn already_exists_error_maps_to_already_exists_status() {
        let err = Error::AlreadyExists("org already exists".into());
        let status: tonic::Status = err.into();
        assert_eq!(status.code(), tonic::Code::AlreadyExists);
        assert_eq!(status.message(), "org already exists");
    }
}
