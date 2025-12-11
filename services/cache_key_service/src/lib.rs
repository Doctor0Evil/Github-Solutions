use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheKeyContext {
    pub repo_id: i64,
    pub workflow_id: i64,
    pub job_id: i64,
    pub action_slug: String,
    pub runner_os: String,
    pub runner_arch: String,
    pub dependency_fingerprint_sha256: String,
    pub aln_policy_revision: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePolicyDecision {
    pub allow_store: bool,
    pub allow_restore: bool,
    pub ttl_seconds: i64,
    pub rationale: String,
}

pub fn derive_cache_key(ctx: &CacheKeyContext) -> String {
    // Stable, deterministic key – safe for multi-region runners
    let mut hasher = Sha256::new();
    hasher.update(ctx.repo_id.to_be_bytes());
    hasher.update(ctx.workflow_id.to_be_bytes());
    hasher.update(ctx.job_id.to_be_bytes());
    hasher.update(ctx.action_slug.as_bytes());
    hasher.update(ctx.runner_os.as_bytes());
    hasher.update(ctx.runner_arch.as_bytes());
    hasher.update(ctx.dependency_fingerprint_sha256.as_bytes());
    hasher.update(ctx.aln_policy_revision.as_bytes());
    let digest = hasher.finalize();
    format!("acorch:v1:{:x}", digest)
}

pub fn evaluate_policy(
    ctx: &CacheKeyContext,
    object_size_bytes: i64,
    estimated_compute_ms: i64,
) -> CachePolicyDecision {
    // Simple ALN-aligned heuristic: only store objects that provide real savings
    let min_compute_savings_ms = 500;     // 0.5s
    let max_size_bytes: i64 = 2 * 1024 * 1024 * 1024; // 2 GiB

    if object_size_bytes <= 0 || object_size_bytes > max_size_bytes {
        return CachePolicyDecision {
            allow_store: false,
            allow_restore: false,
            ttl_seconds: 0,
            rationale: "object_size_out_of_bounds".into(),
        };
    }

    if estimated_compute_ms < min_compute_savings_ms {
        return CachePolicyDecision {
            allow_store: false,
            allow_restore: true,
            ttl_seconds: 0,
            rationale: "low_compute_savings".into(),
        };
    }

    // Baseline: 24h TTL, extended for more expensive artifacts
    let mut ttl_seconds = 24 * 3600;
    if estimated_compute_ms >= 10_000 {
        ttl_seconds = 3 * 24 * 3600;
    }
    if estimated_compute_ms >= 60_000 {
        ttl_seconds = 7 * 24 * 3600;
    }

    CachePolicyDecision {
        allow_store: true,
        allow_restore: true,
        ttl_seconds,
        rationale: "policy_accept".into(),
    }
}

pub fn is_expired(now: DateTime<Utc>, created: DateTime<Utc>, ttl_seconds: i64) -> bool {
    let age = now.signed_duration_since(created).num_seconds();
    age >= ttl_seconds
}
