pub use error_auditor::ErrorAuditor;
pub use resource_error::EmailServer;
pub use resource_error::ResourceError;
pub use error_auditor::Backtrace;
pub use error_auditor::BacktracePart;
pub use error_auditor::Error;
pub use error_auditor::Other;
pub use error_auditor::Runtime;

pub type ErrorAuditor_ = ErrorAuditor<ResourceError>;
