use crate::errors::ExtractResult;
use crate::{EmbeddedDocument, EmbeddedExtractResult, OfficeParserConfig, PdfParserConfig, TesseractOcrConfig};

/// Batch embedded document extraction - STUB IMPLEMENTATION
pub fn extract_embedded_batch(
    _file_path: &str,
    _pdf_conf: &PdfParserConfig,
    _office_conf: &OfficeParserConfig,
    _ocr_conf: &TesseractOcrConfig,
) -> ExtractResult<EmbeddedExtractResult> {
    Err(crate::errors::Error::ParseError(
        "Batch embedded extraction not yet implemented on tika3.2 branch".to_string()
    ))
}

/// Streaming embedded document extraction with callback - STUB IMPLEMENTATION  
pub fn extract_embedded_streaming<F>(
    _file_path: &str,
    _pdf_conf: &PdfParserConfig,
    _office_conf: &OfficeParserConfig,
    _ocr_conf: &TesseractOcrConfig,
    _batch_size: usize,
    _callback: F,
) -> ExtractResult<()>
where
    F: FnMut(Vec<EmbeddedDocument>) -> ExtractResult<bool>,
{
    Err(crate::errors::Error::ParseError(
        "Streaming embedded extraction not yet implemented on tika3.2 branch".to_string()
    ))
}