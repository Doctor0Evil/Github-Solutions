CREATE TABLE actions_cache_registry (
    cache_object_id          UUID PRIMARY KEY,
    repo_id                  BIGINT       NOT NULL,
    workflow_id              BIGINT       NOT NULL,
    job_id                   BIGINT       NOT NULL,
    action_slug              VARCHAR(255) NOT NULL,
    runner_os                VARCHAR(64)  NOT NULL,
    runner_arch              VARCHAR(32)  NOT NULL,
    dependency_fingerprint   CHAR(64)     NOT NULL,  -- SHA-256
    aln_policy_revision      VARCHAR(32)  NOT NULL,

    created_at_utc           TIMESTAMPTZ  NOT NULL DEFAULT now(),
    last_access_utc          TIMESTAMPTZ  NOT NULL DEFAULT now(),
    ttl_seconds              INTEGER      NOT NULL,
    hard_delete_after_utc    TIMESTAMPTZ  NOT NULL,

    object_size_bytes        BIGINT       NOT NULL,
    estimated_compute_ms     BIGINT       NOT NULL,

    cache_hits               BIGINT       NOT NULL DEFAULT 0,
    cache_misses             BIGINT       NOT NULL DEFAULT 0,

    storage_class            VARCHAR(64)  NOT NULL,
    redundancy_level         VARCHAR(32)  NOT NULL,
    encryption_profile       VARCHAR(64)  NOT NULL,

    is_evicted               BOOLEAN      NOT NULL DEFAULT FALSE,
    eviction_reason          VARCHAR(64),

    CHECK (ttl_seconds > 0),
    CHECK (object_size_bytes >= 0),
    CHECK (estimated_compute_ms >= 0)
);

CREATE INDEX idx_actions_cache_registry_repo_workflow
    ON actions_cache_registry (repo_id, workflow_id, job_id);

CREATE INDEX idx_actions_cache_registry_last_access
    ON actions_cache_registry (is_evicted, last_access_utc);

CREATE INDEX idx_actions_cache_registry_expiry
    ON actions_cache_registry (hard_delete_after_utc);
