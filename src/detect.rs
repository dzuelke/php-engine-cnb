use libcnb::{data::build_plan::BuildPlan, DetectOutcome, DetectContext, GenericPlatform, Result};

use crate::PhpBuildpackMetadata;

/// `bin/detect`
pub fn detect(context: DetectContext<GenericPlatform, PhpBuildpackMetadata>) -> Result<DetectOutcome, anyhow::Error> {
    let triggers = vec!["composer.json", "index.php"];

    for fname in triggers.iter() {
        if context.app_dir.join(fname).exists() {
            let buildplan = BuildPlan::new();
            return Ok(DetectOutcome::Pass(buildplan));
        }
    }

    Ok(DetectOutcome::Fail)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;
    use libcnb::{
        data::buildpack::BuildpackToml, DetectContext, DetectOutcome, GenericMetadata,
        GenericPlatform, Platform,
    };
    use std::fs;

    struct TestContext {
        pub ctx: DetectContext<GenericPlatform, PhpBuildpackMetadata>,
        _tmp_dir: tempfile::TempDir,
    }

    impl TestContext {
        pub fn new() -> Self {
            let tmp_dir = tempfile::tempdir().unwrap();
            let app_dir = tmp_dir.path().join("app");
            let buildpack_dir = tmp_dir.path().join("buildpack");
            let platform_dir = tmp_dir.path().join("platform");

            for dir in [&app_dir, &buildpack_dir, &platform_dir] {
                fs::create_dir_all(dir).unwrap();
            }

            let stack_id = String::from("io.buildpacks.stacks.bionic");
            let platform = GenericPlatform::from_path(&platform_dir).unwrap();
            let buildpack_descriptor: BuildpackToml<PhpBuildpackMetadata> =
                toml::from_str(include_str!("../buildpack.toml")).unwrap();
            let ctx = DetectContext {
                app_dir,
                buildpack_dir,
                stack_id,
                platform,
                buildpack_descriptor,
            };

            TestContext {
                ctx,
                _tmp_dir: tmp_dir,
            }
        }
    }

    #[test]
    fn it_fails_if_no_php() {
        let ctx = TestContext::new();
        let result = detect(ctx.ctx);

        assert_matches!(result.unwrap(), DetectOutcome::Fail);
    }

    #[test]
    fn it_passes_detect_if_finds_composerjson() {
        let ctx = TestContext::new();
        fs::write(ctx.ctx.app_dir.join("composer.json"), "").unwrap();
        let result = detect(ctx.ctx);

        assert_matches!(result.unwrap(), DetectOutcome::Pass(_));
    }

    #[test]
    fn it_passes_detect_if_finds_indexphp() {
        let ctx = TestContext::new();
        fs::write(ctx.ctx.app_dir.join("index.php"), "").unwrap();
        let result = detect(ctx.ctx);

        assert_matches!(result.unwrap(), DetectOutcome::Pass(_));
    }
}
