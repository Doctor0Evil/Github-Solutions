package com.github.runner.telemetry

import java.net.HttpURLConnection
import java.net.URL
import java.time.Instant

data class CacheHitEvent(
    val repoId: Long,
    val workflowId: Long,
    val jobId: Long,
    val actionSlug: String,
    val runnerOs: String,
    val runnerArch: String,
    val latencyMs: Long,
    val objectSizeBytes: Long,
    val savedComputeMs: Long,
    val policyId: String,
    val occurredAt: Instant = Instant.now()
)

object TelemetryAgent {

    private const val ENDPOINT = "https://telemetry.github.internal/actions-cache/events"

    fun emitCacheHit(event: CacheHitEvent) {
        val payload = """
            {
              "type": "cache_hit",
              "repo_id": ${event.repoId},
              "workflow_id": ${event.workflowId},
              "job_id": ${event.jobId},
              "action_slug": "${event.actionSlug}",
              "runner_os": "${event.runnerOs}",
              "runner_arch": "${event.runnerArch}",
              "latency_ms": ${event.latencyMs},
              "object_size_bytes": ${event.objectSizeBytes},
              "saved_compute_ms": ${event.savedComputeMs},
              "policy_id": "${event.policyId}",
              "occurred_at": "${event.occurredAt}"
            }
        """.trimIndent()

        val url = URL(ENDPOINT)
        val conn = url.openConnection() as HttpURLConnection
        conn.requestMethod = "POST"
        conn.setRequestProperty("Content-Type", "application/json")
        conn.doOutput = true
        conn.outputStream.use { it.write(payload.toByteArray()) }
        conn.inputStream.close()
    }
}
