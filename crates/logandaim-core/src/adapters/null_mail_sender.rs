use crate::ResultReport;
use crate::ports::{Mail, MailError, MailSender};

/// A `MailSender` for a deployment with no relay configured.
///
/// Every send fails, which the calling use case logs as a warning (NFR-5). Deliberately
/// louder than a fake that swallows mail: an operator who forgot to configure SMTP gets one
/// warning per attempted mail instead of silent success.
#[derive(Debug, Default)]
pub struct NullMailSender;

#[async_trait::async_trait]
impl MailSender for NullMailSender {
    async fn send(&self, _mail: Mail) -> ResultReport<(), MailError> {
        Err(error_stack::Report::new(MailError::NichtKonfiguriert))
    }
}
