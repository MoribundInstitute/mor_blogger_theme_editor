pub mod analyzer;
pub mod module_sources;
pub mod scanner;

use crate::config::TemplatePackConfig;

/// Blogger `<b:includable>` IDs that must be present for a valid post template.
pub(crate) const REQUIRED_BLOG_INCLUDABLES: &[&str] = &[
    "main",
    "post",
    "postBody",
    "postTitle",
    "postCommentsAndAd",
    "postPagination",
];

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
    pub source: Option<String>,
}

impl Warning {
    pub fn error(code: &'static str, message: impl Into<String>) -> Self {
        Self::error_in(code, message, None)
    }

    pub fn error_in(
        code: &'static str,
        message: impl Into<String>,
        source: impl Into<Option<String>>,
    ) -> Self {
        Self {
            severity: Severity::Error,
            code,
            message: message.into(),
            source: source.into(),
        }
    }

    pub fn warn(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            code,
            message: message.into(),
            source: None,
        }
    }

    pub fn format_line(&self) -> String {
        match &self.source {
            Some(source) => format!("[{}] {} — file: {}", self.code, self.message, source),
            None => format!("[{}] {}", self.code, self.message),
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
            .map(Warning::format_line)
            .collect();

        Self {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        }
    }

    pub fn format_report(&self) -> String {
        if self.is_valid && self.warnings.iter().all(|w| w.severity != Severity::Warning) {
            return "sys.integrity — all checks passed".to_string();
        }

        let mut lines = Vec::new();
        let error_count = self
            .warnings
            .iter()
            .filter(|w| w.severity == Severity::Error)
            .count();
        let warning_count = self
            .warnings
            .iter()
            .filter(|w| w.severity == Severity::Warning)
            .count();

        lines.push(format!(
            "Template integrity report: {error_count} error(s), {warning_count} warning(s)"
        ));

        for warning in &self.warnings {
            let prefix = match warning.severity {
                Severity::Error => "ERR",
                Severity::Warning => "WRN",
            };
            lines.push(format!("  {prefix}  {}", warning.format_line()));
        }

        lines.join("\n")
    }
}

/// Run all integrity checks against a Blogger template source string.
pub fn check_integrity(source: &str, active_variants: &TemplatePackConfig) -> DiagnosticResult {
    let mut warnings = Vec::new();

    // 1. Per-module source checks (file attribution before assembly)
    module_sources::check_active_module_sources(active_variants, &mut warnings);

    // 2. Fast text-level scanning (tokens and fatal HTML entities)
    scanner::run_text_checks(source, &mut warnings);

    // 3. Deep XML structural analysis on the assembled template
    analyzer::run_xml_checks(source, active_variants, &mut warnings);

    DiagnosticResult::from_warnings(warnings)
}
