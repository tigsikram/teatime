use build_it::Builder;
use serde::Serialize;

use crate::{error::Result, model::repos::BranchProtection, Client};

#[derive(Debug, Clone, Serialize, Builder)]
#[build_it(into)]
pub struct ListBranchProtectionBuilder {
    /// owner of the repo
    #[skip]
    #[serde(skip)]
    owner: String,
    /// name of the repo
    #[skip]
    #[serde(skip)]
    repo: String,
}

impl ListBranchProtectionBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
        }
    }
    pub async fn send(&self, client: &Client) -> Result<Vec<BranchProtection>> {
        let owner = &self.owner;
        let repo = &self.repo;
        let req = client
            .get(format!("/repos/{owner}/{repo}/branch_protections"))
            .query(self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
