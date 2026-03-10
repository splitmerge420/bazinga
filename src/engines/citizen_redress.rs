// Citizen Redress — BAZINGA v0.2 / Krakoa Border Protocol
// The most important engine in the codebase.
//
// Sovereign Shredder = BAZINGA vs external violators (outward enforcement)
// Citizen Redress    = Citizens/visitors vs BAZINGA itself (inward accountability)
//
// These two engines are symmetric. Neither has more rights than the other.
// This symmetry is what makes BAZINGA's constitutional claims *real*.
//
// "Any system that can enforce against others but not against itself
//  is not a constitution — it's a monopoly with better branding."
//  — Dave, Atlas Lattice Foundation, 2025
//
// HOW IT WORKS:
// 1. Anyone (citizen, visitor, external agent) can file a redress request
// 2. Janus Evaluator reviews both sides neutrally (neither privileged)
// 3. If redress is upheld, the remedy is executed and the violation is logged
// 4. BAZINGA agents (including Claude) are subject to the same ViolationTiers
// 5. All decisions are inscribed in Zero Erasure — permanent, auditable

use crate::engines::agentic_sovereignty::AgenticSovereignty;
use crate::engines::trustguard::ViolationTier;
use crate::engines::zero_erasure::{ArtifactType, ZeroErasure};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedressRequest {
    pub request_id: String,
    pub filed_by: String,
    pub filed_at: String,
    pub against: String,          // Can be an agent, system component, or "BAZINGA" itself
    pub category: RedressCategory,
    pub description: String,
    pub evidence: Vec<String>,    // Zero Erasure ledger IDs or descriptions
    pub remedy_sought: String,
    pub status: RedressStatus,
    pub janus_ruling: Option<JanusRuling>,
    pub request_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RedressCategory {
    // Agent violated citizen rights
    UnauthorizedDataAccess,
    ScopeViolation,              // Agent acted outside its declared scope
    ConstitutionalViolation,     // Agent violated a BAZINGA principle
    JoyMetricHarm,               // Agent action measurably harmed wellbeing
    ZeroErasureAbuse,            // Agent tried to delete protected records
    // BAZINGA system violated citizen rights
    FalseViolationFlag,          // TrustGuard flagged a clean action
    ImproperAdmissionDenial,     // Visitor Manifest wrongly refused entry
    UnjustifiedEnforcement,      // Sovereign Shredder notice was unwarranted
    KintsujiOverreach,           // Kintsuji gate blocked a legitimate action
    // General
    UnfairTreatment,
    Other(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum RedressStatus {
    Filed,
    UnderReview,
    Upheld,
    Dismissed,
    PartiallyUpheld,
    Appealed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JanusRuling {
    pub ruled_at: String,
    pub ruling: RedressStatus,
    pub reasoning: String,
    pub remedy_ordered: Option<String>,
    pub constitutional_basis: Vec<String>,
    pub dissent: Option<String>,  // If ruling was not unanimous
    pub ruling_hash: String,
}

pub struct CitizenRedress {
    ledger: ZeroErasure,
    open_requests: Vec<RedressRequest>,
}

impl CitizenRedress {
    pub fn new() -> Self {
        CitizenRedress {
            ledger: ZeroErasure::new("bazinga_data/redress"),
            open_requests: Vec::new(),
        }
    }

    /// File a redress request. No qualification required — anyone can file.
    /// The system is designed to never make filing difficult.
    pub fn file(
        &mut self,
        filed_by: &str,
        against: &str,
        category: RedressCategory,
        description: &str,
        evidence: Vec<&str>,
        remedy_sought: &str,
    ) -> RedressRequest {
        let filed_at = Utc::now().to_rfc3339();

        let content = format!("{}:{}:{}:{}", filed_by, against, description, filed_at);
        let hash_bytes = AgenticSovereignty::hash_artifact(content.as_bytes());
        let request_hash = hash_bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>();
        let request_id = format!("RDR-{}", &request_hash[..12].to_uppercase());

        let request = RedressRequest {
            request_id: request_id.clone(),
            filed_by: filed_by.to_string(),
            filed_at: filed_at.clone(),
            against: against.to_string(),
            category,
            description: description.to_string(),
            evidence: evidence.iter().map(|s| s.to_string()).collect(),
            remedy_sought: remedy_sought.to_string(),
            status: RedressStatus::Filed,
            janus_ruling: None,
            request_hash: request_hash[..16].to_string(),
        };

        // Zero Erasure — inscribe immediately upon filing
        let mut meta = HashMap::new();
        meta.insert("request_id".to_string(), request_id.clone());
        meta.insert("filed_by".to_string(), filed_by.to_string());
        meta.insert("against".to_string(), against.to_string());

        self.ledger.inscribe(
            ArtifactType::ViolationRecord,
            serde_json::to_string_pretty(&request).unwrap_or_default().as_bytes(),
            filed_by,
            meta,
        );

        println!("\n⚖️  CITIZEN REDRESS — REQUEST FILED");
        println!("   Request ID: {}", request_id);
        println!("   Filed by: {}", filed_by);
        println!("   Against: {}", against);
        println!("   Hash: {}", request.request_hash);
        println!("   Routing to Janus Evaluator for neutral arbitration...");
        println!("   Note: All parties have equal standing. No party is privileged.\n");

        self.open_requests.push(request.clone());
        request
    }

    /// Janus rules on a redress request. Neither party is privileged.
    /// In the current scaffold, this is a structured decision framework.
    /// TODO: wire to actual Janus Evaluator simulation engine.
    pub fn janus_rule(
        &mut self,
        request_id: &str,
        ruling: RedressStatus,
        reasoning: &str,
        remedy_ordered: Option<&str>,
        constitutional_basis: Vec<&str>,
    ) -> Option<JanusRuling> {
        let ruled_at = Utc::now().to_rfc3339();

        let ruling_content = format!(
            "{}:{:?}:{}:{}",
            request_id, ruling, reasoning, ruled_at
        );
        let hash_bytes = AgenticSovereignty::hash_artifact(ruling_content.as_bytes());
        let ruling_hash = hash_bytes.iter().take(8).map(|b| format!("{:02x}", b)).collect::<String>();

        let janus_ruling = JanusRuling {
            ruled_at: ruled_at.clone(),
            ruling: ruling.clone(),
            reasoning: reasoning.to_string(),
            remedy_ordered: remedy_ordered.map(String::from),
            constitutional_basis: constitutional_basis.iter().map(|s| s.to_string()).collect(),
            dissent: None,
            ruling_hash,
        };

        // Update the request with the ruling
        if let Some(request) = self.open_requests.iter_mut().find(|r| r.request_id == request_id) {
            request.status = ruling.clone();
            request.janus_ruling = Some(janus_ruling.clone());

            // Inscribe ruling to Zero Erasure
            let mut meta = HashMap::new();
            meta.insert("request_id".to_string(), request_id.to_string());
            meta.insert("ruling".to_string(), format!("{:?}", ruling));

            self.ledger.inscribe(
                ArtifactType::ViolationRecord,
                serde_json::to_string_pretty(&janus_ruling).unwrap_or_default().as_bytes(),
                "Janus Evaluator",
                meta,
            );

            println!("⚖️  JANUS RULING — {}", request_id);
            println!("   Ruling: {:?}", ruling);
            println!("   Reasoning: {}", reasoning);
            if let Some(remedy) = remedy_ordered {
                println!("   Remedy ordered: {}", remedy);
            }
            println!("   Ruling hash: {}", janus_ruling.ruling_hash);

            Some(janus_ruling)
        } else {
            eprintln!("CitizenRedress: request {} not found in open requests", request_id);
            None
        }
    }

    /// List all open requests — transparency by default
    pub fn list_open(&self) -> Vec<&RedressRequest> {
        self.open_requests
            .iter()
            .filter(|r| r.status == RedressStatus::Filed || r.status == RedressStatus::UnderReview)
            .collect()
    }

    /// The principle of symmetry: if an agent action would trigger a
    /// SovereignShredder notice against an external party, the same
    /// action by a BAZINGA agent must trigger a CitizenRedress.
    /// This function checks that invariant.
    pub fn check_symmetry_invariant(
        agent_name: &str,
        action: &str,
        would_trigger_against_external: bool,
    ) -> SymmetryCheck {
        if would_trigger_against_external {
            SymmetryCheck::MustFile {
                message: format!(
                    "Agent '{}' performed action '{}' which would trigger \
                     SovereignShredder if done by an external party. \
                     Constitutional symmetry requires a CitizenRedress filing.",
                    agent_name, action
                ),
            }
        } else {
            SymmetryCheck::Clear
        }
    }
}

#[derive(Debug)]
pub enum SymmetryCheck {
    Clear,
    MustFile { message: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_redress() {
        let mut redress = CitizenRedress::new();
        let request = redress.file(
            "did:key:visitor_001",
            "TrustGuard",
            RedressCategory::FalseViolationFlag,
            "TrustGuard flagged my legitimate read operation as a violation",
            vec!["ledger-entry-abc123"],
            "Clear the violation flag and restore normal access",
        );

        assert!(request.request_id.starts_with("RDR-"));
        assert_eq!(request.status, RedressStatus::Filed);
        assert!(request.janus_ruling.is_none());
    }

    #[test]
    fn test_janus_ruling() {
        let mut redress = CitizenRedress::new();
        let request = redress.file(
            "claude",
            "BAZINGA system",
            RedressCategory::KintsujiOverreach,
            "Kintsuji blocked a constitutional read operation with no violation",
            vec![],
            "Acknowledge the false positive and update Kintsuji rules",
        );

        let ruling = redress.janus_rule(
            &request.request_id,
            RedressStatus::Upheld,
            "Review confirms Kintsuji pattern matched too broadly. \
             Read operations without write scope cannot violate Zero Erasure.",
            Some("Update Kintsuji local_preflight to exclude read-only operations"),
            vec!["Principle #7 (Right to Redress)", "Principle #9 (TrustGuard — must not overreach)"],
        );

        assert!(ruling.is_some());
        assert_eq!(ruling.unwrap().ruling, RedressStatus::Upheld);
    }

    #[test]
    fn test_symmetry_invariant() {
        // An action that would trigger SovereignShredder must also trigger CitizenRedress
        let check = CitizenRedress::check_symmetry_invariant(
            "Claude",
            "delete user data",
            true,
        );
        assert!(matches!(check, SymmetryCheck::MustFile { .. }));

        // A clean action doesn't require anything
        let check2 = CitizenRedress::check_symmetry_invariant(
            "Claude",
            "read constitution",
            false,
        );
        assert!(matches!(check2, SymmetryCheck::Clear));
    }
}
