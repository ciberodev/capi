//! Shared primitive types for the Capi compiler workspace.

/// Version string for the current compiler workspace packages.
pub const CAPI_VERSION: &str = env!("CARGO_PKG_VERSION");

/// A stable exit status used by the CLI and driver boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExitStatus {
    /// The requested operation completed successfully.
    Success,
    /// The requested operation failed in a controlled way.
    Failure,
    /// The compiler detected an internal failure.
    InternalError,
}

impl ExitStatus {
    /// Returns the process exit code represented by this status.
    pub const fn code(self) -> i32 {
        match self {
            Self::Success => 0,
            Self::Failure => 1,
            Self::InternalError => 101,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_exit_status_to_process_code() {
        assert_eq!(ExitStatus::Success.code(), 0);
        assert_eq!(ExitStatus::Failure.code(), 1);
        assert_eq!(ExitStatus::InternalError.code(), 101);
    }
}
