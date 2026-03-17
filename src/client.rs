use crate::error::{PeasyError, Result};
use crate::types::*;

const DEFAULT_BASE_URL: &str = "https://peasyaudio.com";

pub struct Client {
    http: reqwest::Client,
    base_url: String,
}

impl Client {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: DEFAULT_BASE_URL.to_string(),
        }
    }

    pub fn with_base_url(base_url: &str) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    async fn get(&self, path: &str, params: &[(&str, String)]) -> Result<Vec<u8>> {
        let url = format!("{}{}", self.base_url, path);
        let filtered: Vec<(&str, &str)> = params
            .iter()
            .filter(|(_, v)| !v.is_empty())
            .map(|(k, v)| (*k, v.as_str()))
            .collect();

        let resp = self.http.get(&url).query(&filtered).send().await?;
        let status = resp.status().as_u16();

        if status == 404 {
            return Err(PeasyError::NotFound {
                resource: "resource".into(),
                identifier: path.into(),
            });
        }
        if status != 200 {
            let body = resp.text().await.unwrap_or_default();
            return Err(PeasyError::Api { status, body });
        }
        let bytes = resp.bytes().await?;
        Ok(bytes.to_vec())
    }

    fn list_params(opts: &ListOptions) -> Vec<(&'static str, String)> {
        let mut p = Vec::new();
        if let Some(v) = opts.page {
            p.push(("page", v.to_string()));
        }
        if let Some(v) = opts.limit {
            p.push(("limit", v.to_string()));
        }
        if let Some(ref v) = opts.category {
            p.push(("category", v.clone()));
        }
        if let Some(ref v) = opts.search {
            p.push(("search", v.clone()));
        }
        p
    }

    pub async fn list_tools(&self, opts: &ListOptions) -> Result<PaginatedResponse<Tool>> {
        let body = self.get("/api/v1/tools/", &Self::list_params(opts)).await?;
        Ok(serde_json::from_slice(&body)?)
    }

    pub async fn get_tool(&self, slug: &str) -> Result<Tool> {
        let body = self.get(&format!("/api/v1/tools/{}/", slug), &[]).await?;
        Ok(serde_json::from_slice(&body)?)
    }

    pub async fn list_categories(
        &self,
        opts: &ListOptions,
    ) -> Result<PaginatedResponse<Category>> {
        let mut params = Vec::new();
        if let Some(v) = opts.page {
            params.push(("page", v.to_string()));
        }
        if let Some(v) = opts.limit {
            params.push(("limit", v.to_string()));
        }
        let body = self.get("/api/v1/categories/", &params).await?;
        Ok(serde_json::from_slice(&body)?)
    }

    pub async fn list_formats(&self, opts: &ListOptions) -> Result<PaginatedResponse<Format>> {
        let body = self
            .get("/api/v1/formats/", &Self::list_params(opts))
            .await?;
        Ok(serde_json::from_slice(&body)?)
    }

    pub async fn get_format(&self, slug: &str) -> Result<Format> {
        let body = self
            .get(&format!("/api/v1/formats/{}/", slug), &[])
            .await?;
        Ok(serde_json::from_slice(&body)?)
    }

    pub async fn list_conversions(
        &self,
        opts: &ListConversionsOptions,
    ) -> Result<PaginatedResponse<Conversion>> {
        let mut params = Vec::new();
        if let Some(v) = opts.page {
            params.push(("page", v.to_string()));
        }
        if let Some(v) = opts.limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(ref v) = opts.source {
            params.push(("source", v.clone()));
        }
        if let Some(ref v) = opts.target {
            params.push(("target", v.clone()));
        }
        let body = self.get("/api/v1/conversions/", &params).await?;
        Ok(serde_json::from_slice(&body)?)
    }

    pub async fn list_glossary(
        &self,
        opts: &ListOptions,
    ) -> Result<PaginatedResponse<GlossaryTerm>> {
        let body = self
            .get("/api/v1/glossary/", &Self::list_params(opts))
            .await?;
        Ok(serde_json::from_slice(&body)?)
    }

    pub async fn get_glossary_term(&self, slug: &str) -> Result<GlossaryTerm> {
        let body = self
            .get(&format!("/api/v1/glossary/{}/", slug), &[])
            .await?;
        Ok(serde_json::from_slice(&body)?)
    }

    pub async fn list_guides(
        &self,
        opts: &ListGuidesOptions,
    ) -> Result<PaginatedResponse<Guide>> {
        let mut params = Vec::new();
        if let Some(v) = opts.page {
            params.push(("page", v.to_string()));
        }
        if let Some(v) = opts.limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(ref v) = opts.category {
            params.push(("category", v.clone()));
        }
        if let Some(ref v) = opts.audience_level {
            params.push(("audience_level", v.clone()));
        }
        if let Some(ref v) = opts.search {
            params.push(("search", v.clone()));
        }
        let body = self.get("/api/v1/guides/", &params).await?;
        Ok(serde_json::from_slice(&body)?)
    }

    pub async fn get_guide(&self, slug: &str) -> Result<Guide> {
        let body = self
            .get(&format!("/api/v1/guides/{}/", slug), &[])
            .await?;
        Ok(serde_json::from_slice(&body)?)
    }

    pub async fn list_use_cases(
        &self,
        opts: &ListOptions,
    ) -> Result<PaginatedResponse<UseCase>> {
        let mut params = Vec::new();
        if let Some(v) = opts.page {
            params.push(("page", v.to_string()));
        }
        if let Some(v) = opts.limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(ref v) = opts.category {
            params.push(("industry", v.clone()));
        }
        if let Some(ref v) = opts.search {
            params.push(("search", v.clone()));
        }
        let body = self.get("/api/v1/use-cases/", &params).await?;
        Ok(serde_json::from_slice(&body)?)
    }

    pub async fn search(&self, query: &str, limit: Option<u32>) -> Result<SearchResult> {
        let mut params = vec![("q", query.to_string())];
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        let body = self.get("/api/v1/search/", &params).await?;
        Ok(serde_json::from_slice(&body)?)
    }

    pub async fn list_sites(&self) -> Result<PaginatedResponse<Site>> {
        let body = self.get("/api/v1/sites/", &[]).await?;
        Ok(serde_json::from_slice(&body)?)
    }

    pub async fn openapi_spec(&self) -> Result<serde_json::Value> {
        let body = self.get("/api/openapi.json", &[]).await?;
        Ok(serde_json::from_slice(&body)?)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
