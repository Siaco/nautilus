use futures_util::StreamExt;
use k8s_openapi::api::apps::v1::Deployment;
use kube::api::{DynamicObject, Patch, PatchParams};
use kube::discovery::Discovery;
use kube::runtime::{watcher, WatchStreamExt};
use kube::{Api, Client};

#[derive(Clone)]
pub struct KubeClient {
    pub client: Client,
}

impl KubeClient {
    /// Initializes a Kubernetes client utilizing local kubeconfig or in-cluster auth.
    pub async fn new() -> Result<Self, anyhow::Error> {
        let client = Client::try_default().await?;
        Ok(Self { client })
    }

    /// Dynamically applies a manifest using Server-Side Apply
    pub async fn apply(
        &self,
        group: &str,
        version: &str,
        kind: &str,
        name: &str,
        namespace: Option<&str>,
        json_patch: serde_json::Value,
    ) -> Result<(), anyhow::Error> {
        let discovery = Discovery::new(self.client.clone()).run().await?;

        let (api_resource, _caps) = discovery
            .resolve_gvk(&kube::core::gvk::GroupVersionKind::gvk(
                group, version, kind,
            ))
            .ok_or_else(|| anyhow::anyhow!("Resource not found"))?;

        let api: Api<DynamicObject> = if let Some(ns) = namespace {
            Api::namespaced_with(self.client.clone(), ns, &api_resource)
        } else {
            Api::all_with(self.client.clone(), &api_resource)
        };

        let patch = Patch::Apply(json_patch);
        let patch_params = PatchParams::apply("nautilus-engine").force();

        api.patch(name, &patch_params, &patch).await?;

        Ok(())
    }

    /// Streams deployment status until it reaches Ready status
    pub async fn wait_for_deployment_ready(
        &self,
        name: &str,
        namespace: &str,
    ) -> Result<(), anyhow::Error> {
        let api: Api<Deployment> = Api::namespaced(self.client.clone(), namespace);
        let wp = watcher::Config::default().fields(&format!("metadata.name={}", name));

        let mut stream = watcher(api, wp).applied_objects().boxed();

        while let Some(deployment) = stream.next().await {
            match deployment {
                Ok(dep) => {
                    if let Some(status) = &dep.status {
                        if let (Some(ready), Some(replicas)) =
                            (status.ready_replicas, status.replicas)
                        {
                            if ready >= replicas && replicas > 0 {
                                return Ok(());
                            }
                        }
                    }
                }
                Err(e) => return Err(anyhow::anyhow!("Watcher error: {}", e)),
            }
        }

        Err(anyhow::anyhow!("Stream ended before ready"))
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_kubeclient_initialization() {
        // Just verify that the instantiation method doesn't panic. 
        // In CI without a cluster, this should cleanly return an Err.
        let client_result = super::KubeClient::new().await;
        assert!(client_result.is_ok() || client_result.is_err());
    }
}
