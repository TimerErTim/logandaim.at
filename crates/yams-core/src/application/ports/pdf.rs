use async_trait::async_trait;

use crate::ResultReport;

#[derive(thiserror::Error, Debug)]
pub enum PdfRenderError {
    #[error("pdf rendering failed")]
    Rendering,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PdfDokument {
    /// Placeholder until the first logandaim.at document type is specified.
    Leer,
}

#[async_trait]
pub trait PdfRenderer: Send + Sync {
    async fn rendern(&self, dokument: &PdfDokument) -> ResultReport<Vec<u8>, PdfRenderError>;
}
