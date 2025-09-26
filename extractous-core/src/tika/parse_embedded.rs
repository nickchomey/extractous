use crate::errors::ExtractResult;
use crate::{EmbeddedExtractResult, OfficeParserConfig, PdfParserConfig, TesseractOcrConfig};

/// Extract embedded documents from a file - STUB IMPLEMENTATION
pub fn extract_embedded_from_file(
    _file_path: &str,
    _pdf_conf: &PdfParserConfig,
    _office_conf: &OfficeParserConfig,
    _ocr_conf: &TesseractOcrConfig,
) -> ExtractResult<EmbeddedExtractResult> {
    Err(crate::errors::Error::ParseError(
        "Embedded extraction not yet implemented on tika3.2 branch".to_string()
    ))
}

/// Extract embedded documents from bytes - STUB IMPLEMENTATION  
pub fn extract_embedded_from_bytes(
    _buffer: &[u8],
    _pdf_conf: &PdfParserConfig,
    _office_conf: &OfficeParserConfig,
    _ocr_conf: &TesseractOcrConfig,
) -> ExtractResult<EmbeddedExtractResult> {
    Err(crate::errors::Error::ParseError(
        "Embedded extraction from bytes not yet implemented on tika3.2 branch".to_string()
    ))
}