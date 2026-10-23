use async_trait::async_trait;
use tracing::debug;

use crate::ResultReport;
use crate::ports::{PdfDokument, PdfRenderError, PdfRenderer};

/// Minimal valid one-page PDF with empty content. Used as the App default.
pub const BLANK_PDF: &[u8] = b"%PDF-1.1\n\
1 0 obj<< /Type /Catalog /Pages 2 0 R >>endobj\n\
2 0 obj<< /Type /Pages /Kids [3 0 R] /Count 1 >>endobj\n\
3 0 obj<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] >>endobj\n\
trailer<< /Root 1 0 R >>\n\
%%EOF\n";

pub struct BlankPdfRenderer;

#[async_trait]
impl PdfRenderer for BlankPdfRenderer {
    async fn rendern(&self, _dokument: &PdfDokument) -> ResultReport<Vec<u8>, PdfRenderError> {
        debug!(bytes_len = BLANK_PDF.len(), "blank pdf renderer");
        Ok(BLANK_PDF.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test(pollster::test)]
    async fn blank_renderer_returns_pdf_prefix() {
        let bytes = BlankPdfRenderer.rendern(&PdfDokument::Leer).await.unwrap();
        assert!(bytes.starts_with(b"%PDF"));
    }
}
