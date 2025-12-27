use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use url::Url;

pub struct Invocation {
    pub url: Url
}

pub struct FunctionInvocations {
    functions: Arc<RwLock<HashMap<String, Arc<Invocation>>>>,
}

impl FunctionInvocations {
    pub fn new() -> Self {
        Self { 
            functions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl FunctionInvocations {

    /// Get a process by instance_id
    pub async fn get(&self, instance_id: &str) -> Option<Arc<Invocation>> {
        let functions = self.functions.read().await;
        functions.get(instance_id).cloned()
    }
    /// Get all processes
    pub async fn get_all(&self) -> HashMap<String,Arc<Invocation>> {
        let functions = self.functions.read().await;
        functions.clone()
    }

    /// Insert a process (idempotent overwrite)
    pub async fn insert(
        &self,
        instance_id: String,
        url: Url,
    ) -> Arc<Invocation> {
        let new_invocation = Arc::new(Invocation{
            url
        });
        let mut functions = self.functions.write().await;
        functions.insert(instance_id, new_invocation.clone());
        new_invocation
    }
}
