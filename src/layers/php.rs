use std::collections::HashMap;
use std::env;
use std::path::Path;

use anyhow::Error;
use flate2::read::GzDecoder;
use tar::Archive;

use libcnb::{BuildContext, GenericMetadata, GenericPlatform};
use libcnb::data::layer_content_metadata::LayerContentMetadata;
use libcnb::layer_lifecycle::LayerLifecycle;

use crate::PhpBuildpackMetadata;

pub struct PhpLayerLifecycle;

impl LayerLifecycle<GenericPlatform, PhpBuildpackMetadata, GenericMetadata, HashMap<String, String>, anyhow::Error> for PhpLayerLifecycle {
    fn create(&self, layer_path: &Path, build_context: &BuildContext<GenericPlatform, PhpBuildpackMetadata>) -> Result<LayerContentMetadata<GenericMetadata>, anyhow::Error> {
        streamuntgz(&build_context.buildpack_descriptor.metadata.php_url, &layer_path)?;

        Ok(LayerContentMetadata::default().launch(true))
    }

    fn layer_lifecycle_data(&self, layer_path: &Path, _layer_content_metadata: LayerContentMetadata<GenericMetadata>) -> Result<HashMap<String, String>, Error> {
        let mut php_env: HashMap<String, String> = HashMap::new();

        php_env.insert(
            String::from("PATH"),
            format!(
                "{}:{}:{}",
                layer_path.join("bin").as_path().to_str().unwrap(),
                layer_path.join("sbin").as_path().to_str().unwrap(),
                env::var("PATH").unwrap_or(String::new()),
            ),
        );

        php_env.insert(
            String::from("PHPRC"),
            format!(
                "{}",
                layer_path.join("etc/php").as_path().to_str().unwrap(),
            )
        );

        php_env.insert(
            String::from("PHP_INI_SCAN_DIR"),
            format!(
                "{}:{}",
                env::var("PHP_INI_SCAN_DIR").unwrap_or(String::new()),
                layer_path.join("etc/php/conf.d").as_path().to_str().unwrap(),
            ),
        );

        Ok(php_env)
    }
}

fn streamuntgz(uri: impl AsRef<str>, dst: impl AsRef<Path>) -> anyhow::Result<()> {
    let response = ureq::get(uri.as_ref()).call().into_reader();
    let tar = GzDecoder::new(response);
    let mut archive = Archive::new(tar);
    archive.unpack(dst.as_ref())?;

    Ok(())
}
