
CREATE TABLE IF NOT EXISTS authors (
    author varchar NOT NULL,
    uuid_key varchar NOT NULL,
    creation_date timestamp WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,

    CONSTRAINT authors_pk
        PRIMARY KEY (author)
);

CREATE TABLE IF NOT EXISTS artifacts (
    author varchar NOT NULL,
    artifact_id varchar NOT NULL,
    version varchar NOT NULL,
    description varchar NOT NULL,
    creation_date timestamp WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,

    CONSTRAINT artifacts_pk
        PRIMARY KEY (author, artifact_id, version),

    CONSTRAINT artifacts_author_fk
        FOREIGN KEY (author)
            REFERENCES authors(author)
);

CREATE TABLE IF NOT EXISTS artifact_tags (
    author varchar NOT NULL,
    artifact_id varchar NOT NULL,
    version varchar NOT NULL,
    tag varchar NOT NULL,
    
    CONSTRAINT artifact_tags_fk
        FOREIGN KEY (author, artifact_id, version)
            REFERENCES artifacts(author, artifact_id, version),
            /*
    CONSTRAINT artifact_tags_author_tk
        FOREIGN KEY (author)
            REFERENCES  artifacts(author),

    CONSTRAINT artifact_tags_artifact_id_tk
        FOREIGN KEY (artifact_id)
            REFERENCES artifacts(artifact_id),

    CONSTRAINT artifact_tags_version_tk
        FOREIGN KEY (version)
            REFERENCES artifacts(version),
            */

    CONSTRAINT artifact_tags_pk
        PRIMARY KEY (author, artifact_id, version, tag)
);

