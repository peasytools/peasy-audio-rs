use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct PaginatedResponse<T> {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<T>,
}

#[derive(Debug, Deserialize)]
pub struct Tool {
    pub slug: String,
    pub name: String,
    #[serde(default)]
    pub tagline: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub category: Value,
    #[serde(default)]
    pub url: String,
}

impl Tool {
    /// Returns the best available description — tagline for list, description for detail.
    pub fn display_desc(&self) -> &str {
        if !self.tagline.is_empty() { &self.tagline } else { &self.description }
    }
}

#[derive(Debug, Deserialize)]
pub struct Category {
    pub slug: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub tool_count: u32,
}

#[derive(Debug, Deserialize)]
pub struct Format {
    pub slug: String,
    #[serde(default, alias = "full_name")]
    pub name: String,
    pub extension: String,
    #[serde(default)]
    pub mime_type: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Conversion {
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub target: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub tool_slug: String,
}

#[derive(Debug, Deserialize)]
pub struct GlossaryTerm {
    pub slug: String,
    pub term: String,
    #[serde(default)]
    pub definition: String,
    #[serde(default)]
    pub category: String,
}

#[derive(Debug, Deserialize)]
pub struct Guide {
    pub slug: String,
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub audience_level: String,
    #[serde(default)]
    pub word_count: u32,
}

#[derive(Debug, Deserialize)]
pub struct UseCase {
    pub slug: String,
    pub title: String,
    pub industry: String,
}

#[derive(Debug, Deserialize)]
pub struct Site {
    pub name: String,
    pub domain: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchResult {
    pub query: String,
    pub results: SearchCategories,
}

#[derive(Debug, Deserialize)]
pub struct SearchCategories {
    pub tools: Vec<Tool>,
    pub formats: Vec<Format>,
    pub glossary: Vec<GlossaryTerm>,
}

/// Options for paginated list requests.
#[derive(Default)]
pub struct ListOptions {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub category: Option<String>,
    pub search: Option<String>,
}

/// Options for list_guides with audience level.
#[derive(Default)]
pub struct ListGuidesOptions {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub category: Option<String>,
    pub audience_level: Option<String>,
    pub search: Option<String>,
}

/// Options for list_conversions with source/target.
#[derive(Default)]
pub struct ListConversionsOptions {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub source: Option<String>,
    pub target: Option<String>,
}
