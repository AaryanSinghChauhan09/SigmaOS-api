//! SigmaOS-api: API endpoint model.
//!
//! Real implementation: endpoints, requests, validation.
//! Replaces the 31-line stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Endpoint {
    pub id: String,
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub id: String,
    pub endpoint_id: String,
    pub method: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    EmptyEndpointName,
    DuplicateEndpoint(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyEndpointName => write!(f, "endpoint name must not be empty"),
            Self::DuplicateEndpoint(id) => write!(f, "duplicate endpoint {:?}", id),
        }
    }
}

impl std::error::Error for ApiError {}

pub struct Api {
    pub name: String,
    endpoints: BTreeMap<String, Endpoint>,
    requests: BTreeMap<String, Request>,
}

impl Api {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), endpoints: BTreeMap::new(), requests: BTreeMap::new() }
    }

    pub fn add_endpoint(&mut self, e: Endpoint) -> Result<(), ApiError> {
        if e.name.trim().is_empty() { return Err(ApiError::EmptyEndpointName); }
        let id = e.id.clone();
        if self.endpoints.contains_key(&id) {
            return Err(ApiError::DuplicateEndpoint(id.clone()));
        }
        self.endpoints.insert(id, e);
        Ok(())
    }

    pub fn endpoint_count(&self) -> usize { self.endpoints.len() }

    pub fn get_endpoint(&self, id: &str) -> Option<&Endpoint> { self.endpoints.get(id) }

    pub fn add_request(&mut self, r: Request) {
        self.requests.insert(r.id.clone(), r);
    }

    pub fn get_request(&self, id: &str) -> Option<&Request> { self.requests.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let a = Api::new("Api");
        assert_eq!(a.name, "Api");
        assert_eq!(a.endpoint_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut a = Api::new("Api");
        let endpoint = Endpoint { id: "e1".into(), name: "users".into(), path: "/api/users".into() };
        a.add_endpoint(endpoint).unwrap();
        assert_eq!(a.endpoint_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut a = Api::new("Api");
        let endpoint = Endpoint { id: "x".into(), name: "".into(), path: "x".into() };
        assert!(a.add_endpoint(endpoint).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut a = Api::new("Api");
        let endpoint = Endpoint { id: "x".into(), name: "X".into(), path: "x".into() };
        a.add_endpoint(endpoint).unwrap();
        let endpoint2 = Endpoint { id: "x".into(), name: "X".into(), path: "x".into() };
        assert!(a.add_endpoint(endpoint2).is_err());
    }
}
