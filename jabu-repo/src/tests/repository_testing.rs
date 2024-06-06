use crate::repository::Repository;

use jabu_config::model::ArtifactSpec;

fn sample_artifact() -> ArtifactSpec {
    ArtifactSpec::new("group", "artifact", "0.1.0")
}

fn create_temp_repository() -> std::io::Result<Repository> {
    let tmp_dir = tempdir::TempDir::new("jaburepository")?
        .path()
        .to_path_buf();
    Ok(Repository::new(tmp_dir))
}

#[test]
fn jar_artifact_path_forming() {
    let repo = create_temp_repository().unwrap();
    let artifact = ArtifactSpec {
        author: "group".to_string(),
        artifact_id: "artifact".to_string(),
        version: "1.0.0".to_string(),
    };

    let jar_path = repo.jar_path(&artifact);
    let expected = repo
        .base_path()
        .join(artifact.author)
        .join(artifact.artifact_id)
        .join(artifact.version + ".jar");

    assert_eq!(expected, jar_path);
}

#[test]
fn jaburon_path_forming() {
    let repo = create_temp_repository().unwrap();
    let artifact = sample_artifact();
    let expected = repo
        .base_path()
        .join(artifact.author.clone())
        .join(artifact.artifact_id.clone())
        .join(format!("{}.ron", artifact.version));

    assert_eq!(expected, repo.jaburon_path(&artifact));
}

#[test]
fn get_author_artifacts() -> Result<(), Box<dyn std::error::Error>> {
    let repo = create_temp_repository().unwrap();
    let artifacts = [
        ArtifactSpec::new("author", "artifact", "1.0.0"),
        ArtifactSpec::new("author", "another_artifact", "1.0.0"),
    ];

    artifacts
        .iter()
        .try_for_each(|artifact| repo.save_artifact(artifact, "", ""))?;
    let result = repo
        .get_author_artifacts("author")
        .expect("This array should contain two items (artifact & another_artifact)");
    let expected = vec!["artifact".to_string(), "another_artifact".to_string()];

    assert_eq!(expected, result.as_slice());

    Ok(())
}

#[test]
fn get_artifact_versions() -> Result<(), Box<dyn std::error::Error>> {
    let repo = create_temp_repository()?;
    let artifacts = [
        ArtifactSpec::new("author", "artifact", "1.0.0"),
        ArtifactSpec::new("author", "artifact", "0.3.4"),
    ];

    artifacts
        .iter()
        .try_for_each(|artifact| repo.save_artifact(artifact, "", ""))?;

    let result = repo
        .get_artifact_versions("author", "artifact")
        .expect("This method should have found the given artifact.");

    let expected: Vec<String> = artifacts
        .into_iter()
        .map(|artifact| artifact.version)
        .collect();

    expected
        .iter()
        .for_each(|version| assert!(result.contains(version), "{expected:?} != {result:?}"));

    Ok(())
}
