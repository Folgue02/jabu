mod javac { 
    use crate::{config::JavaConfig, tools::JavacConfig};
    #[test]
    fn test_args() {
        let expected = vec![
            "file.java", "other/file.java", "something.java",
            "--source", "17", "--target", "17",
            "-d", "./target"
        ];
        let sources = vec![
            "file.java".to_string(),
            "other/file.java".to_string(),
            "something.java".to_string()
        ];
        let java_config = JavaConfig { target: 17, source: 17, java_version: 17};
        let output_dir = "./target".to_string();
        let javacc_config = JavacConfig::new(sources, Some(output_dir), Some(java_config));
        assert_eq!(expected, javacc_config.into_args())
    }

    #[test]
    fn test_with_only_sources() {
        let expected = vec![
            "file.java".to_string(),
            "other/file.java".to_string(), 
            "something.java".to_string()
        ];
        let sources = expected.clone();

        assert_eq!(expected, JavacConfig::from_sources(sources).into_args())
    }

    #[test]
    fn test_with_classpath() {
        let expected = vec![
            "src/App.java", "src/tools/Java.java",
            "--source", "17", "--target", "17",
            "-d", "target", 
            "-cp", "lib/file.jar"
        ];
        let classpath = vec![
            "lib/file.jar".to_string(),
            "../project/src/".to_string()
        ];

        let javac_config = JavacConfig::new(
            vec![
                "src/App.java".to_string(),
                "src/tools/Java.java".to_string()
            ],
            Some("target".to_string()),
            Some(JavaConfig::default())
        );
    }
}

mod java {
    use crate::tools::*;
    #[test]
    fn java_only_class() {
        let main_class = String::from("me.user.app.App");
        let expected = vec![
            "me.user.app.App"
        ];
        let java_tool = JavaToolConfig::new(JavaExecTarget::MainClass(main_class), Vec::new(), Vec::new());
        assert_eq!(expected, java_tool.into_args());
    }

    #[test]
    fn java_only_jar() {
        let jar_name = String::from("./file.jar");
        let expected = vec![
            "-jar", "./file.jar"
        ];
        let java_tool = JavaToolConfig::new(JavaExecTarget::Jar(jar_name), Vec::new(), Vec::new());
        assert_eq!(expected, java_tool.into_args());
    }

    #[test]
    fn java_jar_with_classpath() {
        let expected = vec![
            "-cp", "./lib/dependency.jar:./lib/other_dependency.jar",
            "-jar", "./file.jar"
        ];
        let classpath = vec![
            "./lib/dependency.jar".to_string(),
            "./lib/other_dependency.jar".to_string()
        ];
        let java_tool = JavaToolConfig::new(JavaExecTarget::Jar("./file.jar".to_string()), classpath, Vec::new());
        assert_eq!(expected, java_tool.into_args());
    }

    #[test]
    fn java_class_with_classpath() {
        let expected = vec![
            "-cp", "./lib/dependency.jar:./lib/other_dependency.jar",
            "me.user.app.App"
        ];
        let classpath = vec![
            "./lib/dependency.jar".to_string(),
            "./lib/other_dependency.jar".to_string()
        ];
        let java_tool = JavaToolConfig::new(JavaExecTarget::MainClass("me.user.app.App".to_string()), classpath, Vec::new());
        assert_eq!(expected, java_tool.into_args());
    }

    #[test]
    fn java_class_with_arguments_and_classpath() {
        let expected = vec![
            "-cp", "./lib/dependency.jar:./lib/other_dependency.jar",
            "me.user.app.App", "--first-argument", "--second-argument", "third argument"
        ];
        let classpath = vec![
            "./lib/dependency.jar".to_string(),
            "./lib/other_dependency.jar".to_string()
        ];
        let arguments = vec![
            "--first-argument".to_string(),
            "--second-argument".to_string(),
            "third argument".to_string()
        ];
        let java_tool = JavaToolConfig::new(JavaExecTarget::MainClass("me.user.app.App".to_string()), classpath, arguments);
        assert_eq!(expected, java_tool.into_args());
    }
}

mod jar {
    use std::path::PathBuf;
    use crate::tools::JarToolConfig;

    #[test]
    fn simple_compression() {
        let expected = vec![
            "--create".to_string(),
            "--file".to_string(),
            "./target.jar".to_string(),
            "-C".to_string(), "target/classes".to_string(), ".".to_string()
        ];
        let config = JarToolConfig::new("./target.jar".to_string(), "target/classes".to_string());
        assert_eq!(expected, config.into_args())
    }


    #[test]
    fn compression_with_manifest() {
        let expected = vec![
            "--create".to_string(),
            "--file".to_string(),
            "./target.jar".to_string(),
            "--manifest".to_string(), "./target/bin/MANIFEST.MF".to_string(),
            "-C".to_string(), "target/classes".to_string(), ".".to_string()
        ];
        let mut config = JarToolConfig::new("./target.jar".to_string(), "target/classes".to_string());
        config.manifest_location = Some(PathBuf::from("./target/bin/MANIFEST.MF".to_string()));
        assert_eq!(expected, config.into_args())
    }
}

mod javadoc {
    use crate::{
        config::JavaConfig, 
        tools::{
            JavadocToolConfig,
            JavaVisibilityLevel
        }
    };

    #[test]
    fn simple_javadoc_args() {
        let expected = vec![
            "src/main/App.java", "src/main/registry/Person.java",
            "-d", "target/docs",
            "--source", "17",
            "--release", "17",
            "-private"
        ];
        let input = JavadocToolConfig::new(
            vec!["src/main/App.java".to_string(), "src/main/registry/Person.java".to_string()],
            Some("target/docs".to_string()),
            Some(JavaConfig::default()),
            JavaVisibilityLevel::Private
        );
        assert_eq!(expected, input.into_args());
    }

    #[test]
    fn missing_output_dir() {
        let expected = vec![
            "src/main/App.java", "src/main/registry/Person.java",
            "--source", "17",
            "--release", "17",
            "-private"
        ];
        let input = JavadocToolConfig::new(
            vec!["src/main/App.java".to_string(), "src/main/registry/Person.java".to_string()],
            None,
            Some(JavaConfig::default()),
            JavaVisibilityLevel::Private
        );
        assert_eq!(expected, input.into_args());
    }

    #[test]
    fn missing_output_dir_missing_java_config() {
        let expected = vec![
            "src/main/App.java", "src/main/registry/Person.java",
            "-private"
        ];
        let input = JavadocToolConfig::new(
            vec!["src/main/App.java".to_string(), "src/main/registry/Person.java".to_string()],
            None,
            None,
            JavaVisibilityLevel::Private
        );
        assert_eq!(expected, input.into_args());
    }
}
