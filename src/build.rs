use libcnb::{BuildContext, GenericPlatform};
use libcnb::data::launch::{Launch, Process};
use libcnb::layer_lifecycle::execute_layer_lifecycle;

use crate::PhpBuildpackMetadata;
use crate::layers::php::PhpLayerLifecycle;

/// `bin/build`
pub fn build(context: BuildContext<GenericPlatform, PhpBuildpackMetadata>) -> libcnb::Result<(), anyhow::Error> {
    println!("---> PHP Buildpack");
    println!("---> Downloading and extracting PHP");

    execute_layer_lifecycle("php", PhpLayerLifecycle, &context)?;

    write_launch(&context);
    Ok(())
}

fn write_launch(context: &BuildContext<GenericPlatform, PhpBuildpackMetadata>) -> anyhow::Result<()> {
    let mut launch = Launch::new();
    let web = Process::new("web", "php", vec!["-S", "0.0.0.0:8080"], false)?;
    launch.processes.push(web);

    context.write_launch(launch)?;
    Ok(())
}
