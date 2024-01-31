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
