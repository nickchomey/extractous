use crate::errors::ExtractResult;
use crate::{EmbeddedExtractResult, OfficeParserConfig, PdfParserConfig, TesseractOcrConfig};

/// Optimized embedded document extraction - STUB IMPLEMENTATION
pub fn extract_embedded_optimized(
    _file_path: &str,
    _pdf_conf: &PdfParserConfig,
    _office_conf: &OfficeParserConfig,
    _ocr_conf: &TesseractOcrConfig,
) -> ExtractResult<EmbeddedExtractResult> {
    Err(crate::errors::Error::ParseError(
        "Optimized embedded extraction not yet implemented on tika3.2 branch".to_string()
    ))
}