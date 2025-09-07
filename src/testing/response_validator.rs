use tracing::{debug, warn};
use crate::utils::EmbedColor;

#[derive(Debug, Clone)]
pub enum ResponseType {
    Success,
    Error,
    Warning,
    Info,
    Primary,
    Unknown,
}

#[derive(Debug, Clone)]
pub enum VerbosityScore {
    Concise,     // < 100 chars
    Moderate,    // 100-500 chars
    Verbose,     // 500-1000 chars
    VeryVerbose, // > 1000 chars
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub command: String,
    pub response_type: ResponseType,
    pub color_compliant: bool,
    pub actual_color: Option<u32>,
    pub expected_color: Option<u32>,
    pub verbosity_score: VerbosityScore,
    pub format_issues: Vec<String>,
    pub character_count: usize,
    pub word_count: usize,
}

#[derive(Debug, Clone)]
pub struct EmbedData {
    pub title: Option<String>,
    pub description: Option<String>,
    pub color: Option<u32>,
    pub fields: Vec<(String, String)>,
}

impl EmbedData {
    pub fn new(title: Option<String>, description: Option<String>, color: Option<u32>) -> Self {
        Self {
            title,
            description,
            color,
            fields: Vec::new(),
        }
    }
    
    pub fn with_field(mut self, name: String, value: String) -> Self {
        self.fields.push((name, value));
        self
    }
}

pub struct ResponseValidator;

impl ResponseValidator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn validate_embed_data(&self, command: &str, embed_data: &EmbedData) -> ValidationResult {
        let actual_color = embed_data.color;
        let (response_type, expected_color) = self.classify_response_data(embed_data);
        let color_compliant = self.check_color_compliance(actual_color, expected_color);
        
        if !color_compliant {
            warn!(
                "Color mismatch for {}: expected {:?}, got {:?}",
                command, expected_color, actual_color
            );
        }
        
        let (character_count, word_count) = self.calculate_content_metrics_data(embed_data);
        let verbosity_score = self.calculate_verbosity(character_count);
        let format_issues = self.check_format_issues_data(embed_data);
        
        debug!(
            "Validating response for {}: color={:?}, length={}",
            command, actual_color, character_count
        );
        
        ValidationResult {
            command: command.to_string(),
            response_type,
            color_compliant,
            actual_color,
            expected_color,
            verbosity_score,
            format_issues,
            character_count,
            word_count,
        }
    }
    
    fn classify_response_data(&self, embed_data: &EmbedData) -> (ResponseType, Option<u32>) {
        let title = embed_data.title.as_deref().unwrap_or("").to_lowercase();
        let description = embed_data.description.as_deref().unwrap_or("").to_lowercase();
        
        let content = format!("{} {}", title, description);
        
        if content.contains("error") || content.contains("failed") {
            (ResponseType::Error, Some(EmbedColor::Error.value()))
        } else if content.contains("success") || content.contains("complete") {
            (ResponseType::Success, Some(EmbedColor::Success.value()))
        } else if content.contains("warning") || content.contains("caution") {
            (ResponseType::Warning, Some(EmbedColor::Warning.value()))
        } else if content.contains("info") || content.contains("information") {
            (ResponseType::Info, Some(EmbedColor::Warning.value()))
        } else {
            (ResponseType::Primary, Some(EmbedColor::Primary.value()))
        }
    }
    
    fn check_color_compliance(&self, actual: Option<u32>, expected: Option<u32>) -> bool {
        match (actual, expected) {
            (Some(a), Some(e)) => a == e,
            (None, None) => true,
            _ => false,
        }
    }
    
    fn calculate_content_metrics_data(&self, embed_data: &EmbedData) -> (usize, usize) {
        let mut total_chars = 0;
        let mut total_words = 0;
        
        if let Some(ref title) = embed_data.title {
            total_chars += title.len();
            total_words += title.split_whitespace().count();
        }
        
        if let Some(ref description) = embed_data.description {
            total_chars += description.len();
            total_words += description.split_whitespace().count();
        }
        
        for (name, value) in &embed_data.fields {
            total_chars += name.len() + value.len();
            total_words += name.split_whitespace().count() + value.split_whitespace().count();
        }
        
        debug!("Verbosity check: {} chars, {} words", total_chars, total_words);
        (total_chars, total_words)
    }
    
    fn calculate_verbosity(&self, char_count: usize) -> VerbosityScore {
        match char_count {
            0..=100 => VerbosityScore::Concise,
            101..=500 => VerbosityScore::Moderate,
            501..=1000 => VerbosityScore::Verbose,
            _ => VerbosityScore::VeryVerbose,
        }
    }
    
    fn check_format_issues_data(&self, embed_data: &EmbedData) -> Vec<String> {
        let mut issues = Vec::new();
        
        if embed_data.title.is_none() {
            issues.push("Missing title".to_string());
        }
        
        if let Some(ref title) = embed_data.title {
            if title.len() > 256 {
                issues.push("Title exceeds 256 character limit".to_string());
            }
        }
        
        if let Some(ref description) = embed_data.description {
            if description.len() > 4096 {
                issues.push("Description exceeds 4096 character limit".to_string());
            }
        }
        
        if embed_data.fields.len() > 25 {
            issues.push("More than 25 fields".to_string());
        }
        
        for (i, (name, value)) in embed_data.fields.iter().enumerate() {
            if name.is_empty() {
                issues.push(format!("Field {} missing name", i));
            }
            if value.is_empty() {
                issues.push(format!("Field {} missing value", i));
            }
        }
        
        let total_chars = self.calculate_content_metrics_data(embed_data).0;
        if total_chars > 6000 {
            issues.push("Total embed size exceeds 6000 character limit".to_string());
        }
        
        debug!("Format validation complete: {} issues found", issues.len());
        issues
    }
    
    pub fn validate_responses(&self, command: &str, embed_data_list: &[EmbedData]) -> Vec<ValidationResult> {
        embed_data_list.iter()
            .map(|embed_data| self.validate_embed_data(command, embed_data))
            .collect()
    }
}