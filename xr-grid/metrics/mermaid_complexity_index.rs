pub struct MermaidComplexityIndex {
    pub node_count: u32,
    pub edge_count: u32,
    pub avg_label_len: f32,
    pub math_token_density: f32,   // math tokens per label char, infra-only
    pub hazard_line_fraction: f32, // fraction of lines with non-None MermaidHazardClass
    pub safe_for_render: bool,     // envelope-level flag, not person-level
}

// Intended use:
// - Computed from a block of Mermaid + its MermaidSafetyProfile stream.
// - Compared against stage/state ceilings derived from OrganicCpuSpecSnapshot
//   and NeuroswarmGuard StabilityScore, but only as infra throttling:
//   e.g., if StabilityScore low or AutonomicInstabilityIndex high,
//         require MCI.node_count and edge_count below conservative limits,
//         or prefer textual summaries.
// Hex-stamp: 0xd3a7c5e19b404f8aa1e6f0c9b2a4837d
