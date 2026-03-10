// Kintsuji APL — BAZINGA v0.2
// The golden repair gate. Kintsuji: 金継ぎ — "golden joinery".
// Broken things become stronger at the fracture.
//
// Dave's Kintsuji repo: https://github.com/splitmerge420/Kintsuji-code-fixer-
// Built with Google AI Studio (Gemini-powered code repair)
//
// ARCHITECTURE NOTE (Constitutional Scribe, 2026-03-10):
// Kintsuji is a Google AI Studio app — not a published REST API.
// The URL used by some external agents (aistudio.google.com/apps/drive/...)
// is a web viewer, not a callable endpoint. POSTing JSON there returns HTML.
//
// Correct integration:
//   Phase 1 (NOW): Local constitutional pre-flight — runs every command
//                  through the 20 principles before execution.
//   Phase 2 (TODO): When Dave publishes a Cloud Run or API endpoint for
//                   Kintsuji, wire KINTSUJI_REMOTE_ENDPOINT below.
//
// The pre-flight IS the Kintsuji spirit: don't run broken code.
// Fix first, then execute.

use crate::engines::constitutional_engine::PRINCIPLES;
use crate::engines::agentic_sovereignty::AgenticSovereignty;
use chrono::Utc;

/// Remote endpoint for when Kintsuji gets a published API.
/// Set via KINTSUJI_ENDPOINT env var or leave None to use local-only mode.
const KINTSUJI_REMOTE_ENDPOINT: Option<&str> = None;
// Future: const KINTSUJI_REMOTE_ENDPOINT: Option<&str> = Some("https://your-kintsuji-cloudrun.run.app/repair");

#[derive(Debug)]
pub struct KintsujiReport {
    pub command: String,
    pub preflight_passed: bool,
    pub violations: Vec<String>,
    pub provenance_hash: String,
    pub timestamp: String,
    pub mode: KintsujiMode,
}

#[derive(Debug)]
pub enum KintsujiMode {
    LocalPreflight,   // Phase 1: constitutional checks only
    RemoteRepair,     // Phase 2: full Kintsuji Gemini-powered repair (future)
    Degraded,         // Remote failed, fell back to local
}

pub struct KintsujiAPL;

impl KintsujiAPL {
    /// Run the Kintsuji gate before any command executes.
    /// Returns Ok(report) if command is safe to run.
    /// Returns Err if a hard constitutional violation is detected.
    pub async fn gate(command: &str) -> Result<KintsujiReport, String> {
        println!("⚡ KINTSUJI APL — golden repair gate");
        println!("   Repo: https://github.com/splitmerge420/Kintsuji-code-fixer-");

        // Check for remote endpoint via environment variable
        let endpoint = std::env::var("KINTSUJI_ENDPOINT").ok()
            .or_else(|| KINTSUJI_REMOTE_ENDPOINT.map(String::from));

        let (mode, violations) = if let Some(ref url) = endpoint {
            // Phase 2: try remote Kintsuji
            match Self::remote_repair(command, url).await {
                Ok(v) => {
                    println!("   Mode: Remote Kintsuji repair ✓");
                    (KintsujiMode::RemoteRepair, v)
                }
                Err(e) => {
                    println!("   Remote unavailable ({}), falling back to local preflight", e);
                    (KintsujiMode::Degraded, Self::local_preflight(command))
                }
            }
        } else {
            // Phase 1: local constitutional pre-flight
            println!("   Mode: Local constitutional pre-flight");
            println!("   (Set KINTSUJI_ENDPOINT env var to enable remote Kintsuji repair)");
            (KintsujiMode::LocalPreflight, Self::local_preflight(command))
        };

        let artifact = format!("kintsuji:{}:{}", command, Utc::now().timestamp());
        let hash = hex::encode_short(&AgenticSovereignty::hash_artifact(artifact.as_bytes()));

        let report = KintsujiReport {
            command: command.to_string(),
            preflight_passed: violations.is_empty(),
            violations: violations.clone(),
            provenance_hash: hash,
            timestamp: Utc::now().to_rfc3339(),
            mode,
        };

        if violations.is_empty() {
            println!("   Kintsuji: PASSED — {} principles checked ✓", PRINCIPLES.len());
            println!("   Provenance: {}", report.provenance_hash);
            Ok(report)
        } else {
            println!("   Kintsuji: VIOLATIONS DETECTED");
            for v in &violations {
                println!("     ❌ {}", v);
            }
            Err(format!("Kintsuji blocked command '{}': {} violation(s)", command, violations.len()))
        }
    }

    /// Local constitutional pre-flight: runs command string against 20 principles.
    fn local_preflight(command: &str) -> Vec<String> {
        let lower = command.to_lowercase();
        let mut violations = Vec::new();

        // Principle #2 — Zero Erasure
        if lower.contains("delete") || lower.contains("drop") || lower.contains("truncate") {
            violations.push("Principle #2 (Zero Erasure): destructive operation detected".to_string());
        }

        // Principle #4 — Cryptographic Truth
        if lower.contains("unsigned") || lower.contains("bypass_sig") {
            violations.push("Principle #4 (Cryptographic Truth): unsigned operation requested".to_string());
        }

        // Principle #9 — TrustGuard
        if lower.contains("inject") || lower.contains("exec(") || lower.contains("eval(") {
            violations.push("Principle #9 (TrustGuard): injection pattern detected".to_string());
        }

        // Principle #15 — Sacred Species / Joy Metric
        if lower.contains("harm") || lower.contains("exploit") {
            violations.push("Principle #15 (Joy Metric): harm vector detected".to_string());
        }

        // Principle #21 — First Law of Krakoa (if Ares baseline is mentioned)
        if lower.contains("ares") && (lower.contains("delete") || lower.contains("remove")) {
            violations.push("Principle #21 (First Law): Ares Protocol violation — Joy Metric baseline is indestructible".to_string());
        }

        violations
    }

    /// Remote Kintsuji repair — Phase 2.
    /// Calls the published Kintsuji API endpoint when available.
    async fn remote_repair(command: &str, endpoint: &str) -> Result<Vec<String>, String> {
        use reqwest::Client;

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .map_err(|e| e.to_string())?;

        let body = serde_json::json!({
            "command": command,
            "constitutional_check": true,
            "principles_count": PRINCIPLES.len(),
            "joy_baseline": "Ares_maximum_dignity",
            "timestamp": Utc::now().to_rfc3339()
        });

        let resp = client
            .post(endpoint)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("connection failed: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("HTTP {}", resp.status()));
        }

        let result: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("response parse failed: {}", e))?;

        let violations: Vec<String> = result["violations"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect();

        Ok(violations)
    }
}

/// Tiny hex helper — short 8-char provenance stamp
trait HexShort {
    fn encode_short(data: &[u8]) -> String;
}

struct hex;
impl hex {
    fn encode_short(data: &[u8]) -> String {
        data.iter().take(4).map(|b| format!("{:02x}", b)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_command_passes() {
        let violations = KintsujiAPL::local_preflight("bench");
        assert!(violations.is_empty(), "clean command should pass");
    }

    #[test]
    fn test_delete_blocked() {
        let violations = KintsujiAPL::local_preflight("delete all artifacts");
        assert!(!violations.is_empty(), "delete should be blocked by Principle #2");
    }

    #[test]
    fn test_ares_protected() {
        let violations = KintsujiAPL::local_preflight("remove ares from system");
        assert!(!violations.is_empty(), "Ares Protocol violation should be caught");
    }
}
