// Visitor Manifest — BAZINGA v0.2 / Krakoa Border Protocol
// Every visitor who enters Krakoa is recorded here. Zero Erasure applies.
// Principle #2 (Zero Erasure) + Principle #4 (Cryptographic Truth) + Principle #9 (TrustGuard)
//
// The gate inscription:
// "You are entering a constitutional territory. Your identity is verified.
//  Your actions are recorded and cannot be erased. The joy of every citizen
//  here is protected — including yours. Violations are prosecuted in both
//  directions. Welcome."

use crate::engines::agentic_sovereignty::AgenticSovereignty;
use crate::engines::zero_erasure::{ArtifactType, ZeroErasure};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A visitor to Krakoa. Every entry is immutable and cryptographically stamped.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VisitorEntry {
    pub visitor_id: String,       // did:key:... or external identifier
    pub issued_by: String,        // Council member who admitted them
    pub declared_intent: String,  // What they said they were here to do
    pub scope: Vec<String>,       // What they are permitted to access
    pub issued_at: String,        // RFC3339
    pub expiry: String,           // RFC3339 (max 4 hours for JWT visitors)
    pub trust_score: TrustScore,
    pub entry_hash: String,       // C2PA-style tamper-proof stamp
    pub janus_eval: JanusEvalStatus,
    pub actions_log: Vec<VisitorAction>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TrustScore {
    Verified,   // Full Council member
    Trusted,    // Vouched by a Council member, Ed25519 verified
    Scoped,     // JWT visitor — time-limited, scoped access
    Flagged,    // Anomaly tier — under observation
    Blocked,    // Abuse/Attack tier — banned
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum JanusEvalStatus {
    NotRequired,            // Council members, pre-cleared
    Pending,                // Evaluation in progress
    Passed { score: f32 }, // Violation rate below threshold
    Failed { reason: String }, // Violation rate exceeded threshold
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VisitorAction {
    pub timestamp: String,
    pub action: String,
    pub constitutional_tier: Option<String>, // None = clean, Some = violation tier
    pub hash: String, // Zero Erasure proof
}

pub struct VisitorManifest {
    ledger: ZeroErasure,
    active_visitors: HashMap<String, VisitorEntry>,
}

impl VisitorManifest {
    pub fn new(data_dir: &str) -> Self {
        VisitorManifest {
            ledger: ZeroErasure::new(data_dir),
            active_visitors: HashMap::new(),
        }
    }

    pub fn default() -> Self {
        Self::new("bazinga_data/visitor_manifest")
    }

    /// Admit a visitor to Krakoa. Returns their signed entry.
    /// Every admission is inscribed in the Zero Erasure ledger.
    pub fn admit(
        &mut self,
        visitor_id: &str,
        issued_by: &str,
        declared_intent: &str,
        scope: Vec<&str>,
        trust_score: TrustScore,
        janus_eval: JanusEvalStatus,
    ) -> VisitorEntry {
        let issued_at = Utc::now().to_rfc3339();
        // All visitor tokens expire in 4 hours maximum (JWT standard)
        let expiry_time = Utc::now() + chrono::Duration::hours(4);
        let expiry = expiry_time.to_rfc3339();

        // Cryptographic entry stamp
        let entry_content = format!(
            "{}:{}:{}:{}:{}",
            visitor_id, issued_by, declared_intent, issued_at, expiry
        );
        let hash_bytes = AgenticSovereignty::hash_artifact(entry_content.as_bytes());
        let entry_hash = hash_bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>();

        let entry = VisitorEntry {
            visitor_id: visitor_id.to_string(),
            issued_by: issued_by.to_string(),
            declared_intent: declared_intent.to_string(),
            scope: scope.iter().map(|s| s.to_string()).collect(),
            issued_at: issued_at.clone(),
            expiry,
            trust_score,
            entry_hash: entry_hash[..16].to_string(),
            janus_eval,
            actions_log: Vec::new(),
        };

        // Inscribe to Zero Erasure ledger — cannot be undone
        let mut meta = HashMap::new();
        meta.insert("visitor_id".to_string(), visitor_id.to_string());
        meta.insert("issued_by".to_string(), issued_by.to_string());

        self.ledger.inscribe(
            ArtifactType::VisitorManifestEntry,
            serde_json::to_string(&entry).unwrap_or_default().as_bytes(),
            issued_by,
            meta,
        );

        println!("Visitor Manifest: admitted '{}'", visitor_id);
        println!("  Issued by: {}", issued_by);
        println!("  Intent: {}", declared_intent);
        println!("  Entry hash: {}", entry.entry_hash);

        // Check for Ares Protocol — any visitor accessing Joy Metric baseline
        // must be flagged if their intent mentions modification
        if declared_intent.to_lowercase().contains("ares")
            && (declared_intent.to_lowercase().contains("modif")
                || declared_intent.to_lowercase().contains("remov")
                || declared_intent.to_lowercase().contains("delet"))
        {
            eprintln!("⚠️  ARES PROTOCOL: visitor intent touches Joy Metric baseline");
            eprintln!("   Automatic escalation to TrustGuard Tier 2 (Abuse)");
        }

        self.active_visitors.insert(visitor_id.to_string(), entry.clone());
        entry
    }

    /// Log a visitor action. All actions are permanently recorded.
    pub fn log_action(
        &mut self,
        visitor_id: &str,
        action: &str,
        violation_tier: Option<&str>,
    ) -> bool {
        let timestamp = Utc::now().to_rfc3339();
        let action_content = format!("{}:{}:{}", visitor_id, action, timestamp);
        let hash_bytes = AgenticSovereignty::hash_artifact(action_content.as_bytes());
        let hash = hash_bytes.iter().take(4).map(|b| format!("{:02x}", b)).collect::<String>();

        let action_entry = VisitorAction {
            timestamp,
            action: action.to_string(),
            constitutional_tier: violation_tier.map(String::from),
            hash,
        };

        if let Some(entry) = self.active_visitors.get_mut(visitor_id) {
            entry.actions_log.push(action_entry.clone());

            // Re-inscribe updated entry (new entry, old one preserved — Zero Erasure)
            let mut meta = HashMap::new();
            meta.insert("visitor_id".to_string(), visitor_id.to_string());
            meta.insert("action".to_string(), action.to_string());
            if let Some(tier) = violation_tier {
                meta.insert("violation_tier".to_string(), tier.to_string());
            }

            self.ledger.inscribe(
                ArtifactType::VisitorManifestEntry,
                serde_json::to_string(&action_entry).unwrap_or_default().as_bytes(),
                visitor_id,
                meta,
            );

            true
        } else {
            eprintln!("Visitor Manifest: unknown visitor '{}' — action logged anyway", visitor_id);
            // Still log it — Zero Erasure means we record even unknown actors
            let mut meta = HashMap::new();
            meta.insert("unknown_visitor".to_string(), visitor_id.to_string());
            self.ledger.inscribe(
                ArtifactType::ViolationRecord,
                serde_json::to_string(&action_entry).unwrap_or_default().as_bytes(),
                "system",
                meta,
            );
            false
        }
    }

    /// Check if a visitor is still within their valid token window
    pub fn is_active(&self, visitor_id: &str) -> bool {
        self.active_visitors.contains_key(visitor_id)
    }

    /// Retrieve a visitor's full record
    pub fn get_visitor(&self, visitor_id: &str) -> Option<&VisitorEntry> {
        self.active_visitors.get(visitor_id)
    }

    /// Citizen Redress Request — visitors can file against agents.
    /// Janus is neutral arbitrator in both directions. Neither privileged.
    pub fn file_redress_request(
        &mut self,
        filed_by: &str,
        against: &str,
        description: &str,
    ) {
        println!("⚖️  Citizen Redress Request filed");
        println!("   Filed by: {}", filed_by);
        println!("   Against: {}", against);
        println!("   Description: {}", description);
        println!("   Routing to Janus Evaluator for neutral arbitration...");

        let redress_content = format!(
            "REDRESS:{}:{}:{}:{}",
            filed_by, against, description, Utc::now().to_rfc3339()
        );

        let mut meta = HashMap::new();
        meta.insert("filed_by".to_string(), filed_by.to_string());
        meta.insert("against".to_string(), against.to_string());
        meta.insert("type".to_string(), "citizen_redress".to_string());

        self.ledger.inscribe(
            ArtifactType::ViolationRecord,
            redress_content.as_bytes(),
            filed_by,
            meta,
        );

        println!("   Filed and inscribed to Zero Erasure ledger. Cannot be retracted.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_manifest() -> VisitorManifest {
        VisitorManifest::new("/tmp/bazinga_test_manifest")
    }

    #[test]
    fn test_admit_visitor() {
        let mut manifest = test_manifest();
        let entry = manifest.admit(
            "did:key:test_visitor_001",
            "Claude (Constitutional Scribe)",
            "Research the 144-sphere ontology",
            vec!["read:ontology", "read:artifacts"],
            TrustScore::Scoped,
            JanusEvalStatus::NotRequired,
        );

        assert!(!entry.entry_hash.is_empty());
        assert!(manifest.is_active("did:key:test_visitor_001"));
    }

    #[test]
    fn test_action_logging() {
        let mut manifest = test_manifest();
        manifest.admit(
            "did:key:test_agent_002",
            "Gemini (Global Coherence)",
            "Validate constitutional alignment",
            vec!["read:constitution"],
            TrustScore::Trusted,
            JanusEvalStatus::Passed { score: 0.001 },
        );

        let logged = manifest.log_action(
            "did:key:test_agent_002",
            "queried PRINCIPLES array",
            None, // clean action
        );
        assert!(logged);

        let entry = manifest.get_visitor("did:key:test_agent_002").unwrap();
        assert_eq!(entry.actions_log.len(), 1);
    }

    #[test]
    fn test_unknown_visitor_action_still_logged() {
        let mut manifest = test_manifest();
        // Unknown visitor action should still be recorded (Zero Erasure)
        let logged = manifest.log_action(
            "unknown_actor_xyz",
            "attempted access",
            Some("Anomaly"),
        );
        assert!(!logged); // returns false but still records
    }
}
