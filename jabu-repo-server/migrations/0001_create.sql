
CREATE TABLE IF NOT EXISTS authors (
    author varchar,
    uuid_key varchar,
    creation_date timestamp WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT authors_pk
        PRIMARY KEY (author)
);

CREATE TABLE IF NOT EXISTS artifacts (
    author varchar,
    artifact_id varchar,
    version varchar,
    description varchar,
    creation_date timestamp WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT artifacts_pk
        PRIMARY KEY (author, artifact_id, version),

    CONSTRAINT artifacts_author_fk
        FOREIGN KEY (author)
            REFERENCES authors(author)
);

CREATE TABLE IF NOT EXISTS artifact_tags (
    author varchar,
    artifact_id varchar,
    version varchar,
    tag varchar,
    
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

