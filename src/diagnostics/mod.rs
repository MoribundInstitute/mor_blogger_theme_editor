pub mod analyzer;
pub mod scanner;

use crate::config::TemplatePackConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone)]
pub struct Warning {
    pub severity: Severity,
    pub code: &'static str,
    pub message: String,
}

impl Warning {
    pub fn error(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            code,
            message: message.into(),
        }
    }

    pub fn warn(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            code,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DiagnosticResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<Warning>,
}

impl DiagnosticResult {
    pub fn from_warnings(warnings: Vec<Warning>) -> Self {
        let errors: Vec<String> = warnings
            .iter()
            .filter(|w| w.severity == Severity::Error)
            .map(|w| format!("[{}] {}", w.code, w.message))
            .collect();

        Self {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        }
    }
}

/// Run all integrity checks against a Blogger template source string.
pub fn check_integrity(source: &str, active_variants: &TemplatePackConfig) -> DiagnosticResult {
    let mut warnings = Vec::new();

    // 1. Fast Text-Level Scanning (Catches tokens and fatal HTML entities)
    scanner::run_text_checks(source, &mut warnings);

    // 2. Deep XML Structural Analyzing
    analyzer::run_xml_checks(source, active_variants, &mut warnings);

    DiagnosticResult::from_warnings(warnings)
}
