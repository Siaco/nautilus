use bollard::image::BuildImageOptions;
use bollard::Docker;
use futures_util::stream::StreamExt;

#[derive(Clone)]
pub struct DockerClient {
    pub docker: Docker,
}

impl DockerClient {
    /// Initializes a connection to the Docker daemon.
    pub fn new() -> Result<Self, anyhow::Error> {
        let docker = Docker::connect_with_local_defaults()
            .map_err(|e| anyhow::anyhow!("Failed to connect to docker: {}", e))?;
        Ok(Self { docker })
    }

    /// Builds a container image and streams the logs.
    pub async fn build_image(
        &self,
        tag: &str,
        dockerfile_content: &str,
        log_sender: Option<tokio::sync::mpsc::Sender<String>>,
    ) -> Result<(), anyhow::Error> {
        // Because bollard requires a tarball for context, we construct an in-memory tarball containing the Dockerfile.
        let mut header = tar::Header::new_gnu();
        header.set_path("Dockerfile").unwrap();
        header.set_size(dockerfile_content.len() as u64);
        header.set_cksum();

        let mut ar = tar::Builder::new(Vec::new());
        ar.append(&header, dockerfile_content.as_bytes()).unwrap();
        let tarball = ar.into_inner().unwrap();

        let options = BuildImageOptions {
            dockerfile: "Dockerfile",
            t: tag,
            rm: true,
            ..Default::default()
        };

        let mut image_build_stream =
            self.docker
                .build_image(options, None, Some(bytes::Bytes::from(tarball)));

        while let Some(msg) = image_build_stream.next().await {
            match msg {
                Ok(info) => {
                    if let Some(stream) = info.stream {
                        if let Some(s) = &log_sender {
                            let _ = s.send(stream.clone()).await;
                        } else {
                            print!("{}", stream);
                        }
                    } else if let Some(error) = info.error {
                        if let Some(s) = &log_sender {
                            let _ = s.send(format!("ERROR: {}", error)).await;
                        }
                        return Err(anyhow::anyhow!("Docker build error: {}", error));
                    }
                }
                Err(e) => return Err(anyhow::anyhow!("Build stream error: {}", e)),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_docker_compiles() {
        assert!(true);
    }
}
