// Sovereign Shredder — BAZINGA v0.2 / Krakoa Border Protocol
// Constitutional enforcement. The teeth of the system.
// Named for the document shredder — but inverted:
// We don't shred evidence. We shred the *right to operate* when you violate sovereignty.
//
// IMPORTANT DISTINCTION (Mythology Boundary — active):
// - Dreamtime: "The Shredder" as mythological enforcer figure
// - Base Reality: This file — a software enforcement mechanism that issues notices,
//   logs violations, and escalates through formal channels (legal, regulatory, platform)
//
// The Sovereign Shredder does NOT execute code on external systems.
// It generates cryptographically-stamped enforcement notices and routes them.
// The actual enforcement happens through:
//   1. Platform reports (Chrome Web Store, App Store, etc.)
//   2. Regulatory filings (DOJ, FTC, GDPR)
//   3. Public disclosure (audit trail is immutable, so violations are permanent record)

use crate::engines::agentic_sovereignty::AgenticSovereignty;
use crate::engines::trustguard::ViolationTier;
use crate::engines::zero_erasure::{ArtifactType, ZeroErasure};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A formal enforcement notice. Cryptographically stamped. Immutable once issued.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnforcementNotice {
    pub notice_id: String,
    pub issued_at: String,
    pub respondent: String,         // Entity being notified
    pub violation_type: ViolationType,
    pub severity: ViolationTier,
    pub description: String,
    pub evidence_hashes: Vec<String>, // Zero Erasure ledger IDs of supporting evidence
    pub routing: Vec<EnforcementRoute>,
    pub constitutional_basis: Vec<String>, // Which BAZINGA principles were violated
    pub remedy_requested: String,
    pub notice_hash: String,        // SHA-256 of entire notice content
    pub status: NoticeStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ViolationType {
    DataSovereigntyViolation,    // User data extracted without consent
    ConstitutionalBypass,        // Attempted to circumvent TrustGuard/Kintsuji
    PlatformMonopolyAbuse,       // App store / distribution channel weaponization
    VisitorManifestBreach,       // Actions outside declared scope
    ZeroErasureViolation,        // Attempted deletion of protected records
    JoyMetricHarm,               // Actions that measurably harm user wellbeing (Ares Protocol)
    CryptographicFraud,          // Forged signatures, tampered hashes
    SurveillanceCapitalism,      // Covert data monetization
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EnforcementRoute {
    PlatformReport {
        platform: String,     // "Chrome Web Store", "Apple App Store", etc.
        report_url: String,
        priority: String,
    },
    RegulatoryFiling {
        agency: String,       // "DOJ", "FTC", "GDPR-DPA", etc.
        filing_type: String,  // "antitrust", "privacy", "consumer protection"
        jurisdiction: String,
    },
    PublicDisclosure {
        channel: String,      // "GitHub", "BAZINGA Audit Log", "Press Release"
        embargo_hours: u32,   // 0 = immediate, 72 = responsible disclosure window
    },
    DirectNotice {
        contact: String,      // email or legal entity address
        method: String,       // "email", "certified mail", "API webhook"
    },
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum NoticeStatus {
    Draft,
    Issued,
    Acknowledged,
    Resolved,
    Escalated,
}

pub struct SovereignShredder {
    ledger: ZeroErasure,
}

impl SovereignShredder {
    pub fn new() -> Self {
        SovereignShredder {
            ledger: ZeroErasure::new("bazinga_data/enforcement"),
        }
    }

    /// Issue a formal enforcement notice.
    /// All notices are permanently inscribed in Zero Erasure ledger.
    /// Returns the stamped notice.
    pub fn issue_notice(
        &self,
        respondent: &str,
        violation_type: ViolationType,
        severity: ViolationTier,
        description: &str,
        constitutional_basis: Vec<&str>,
        remedy_requested: &str,
        evidence_hashes: Vec<String>,
    ) -> EnforcementNotice {
        let issued_at = Utc::now().to_rfc3339();

        // Determine routing based on severity
        let routing = self.compute_routing(&violation_type, &severity, respondent);

        // Compute notice hash for tamper-proofing
        let notice_content = format!(
            "{}:{}:{:?}:{:?}:{}:{}",
            respondent, issued_at, violation_type, severity, description, remedy_requested
        );
        let hash_bytes = AgenticSovereignty::hash_artifact(notice_content.as_bytes());
        let notice_hash = hash_bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>();
        let notice_id = format!("SHR-{}", &notice_hash[..12].to_uppercase());

        let notice = EnforcementNotice {
            notice_id: notice_id.clone(),
            issued_at: issued_at.clone(),
            respondent: respondent.to_string(),
            violation_type,
            severity,
            description: description.to_string(),
            evidence_hashes,
            routing: routing.clone(),
            constitutional_basis: constitutional_basis.iter().map(|s| s.to_string()).collect(),
            remedy_requested: remedy_requested.to_string(),
            notice_hash: notice_hash[..16].to_string(),
            status: NoticeStatus::Issued,
        };

        // Inscribe to Zero Erasure ledger — this cannot be undone
        let mut meta = HashMap::new();
        meta.insert("notice_id".to_string(), notice_id.clone());
        meta.insert("respondent".to_string(), respondent.to_string());

        self.ledger.inscribe(
            ArtifactType::ViolationRecord,
            serde_json::to_string_pretty(&notice).unwrap_or_default().as_bytes(),
            "Sovereign Shredder",
            meta,
        );

        // Print notice header
        println!("\n🔴 SOVEREIGN SHREDDER — NOTICE ISSUED");
        println!("   Notice ID: {}", notice_id);
        println!("   Respondent: {}", respondent);
        println!("   Severity: {:?}", notice.severity);
        println!("   Hash: {}", notice.notice_hash);
        println!("   Routing:");
        for route in &routing {
            match route {
                EnforcementRoute::PlatformReport { platform, .. } => {
                    println!("     → Platform Report: {}", platform);
                }
                EnforcementRoute::RegulatoryFiling { agency, filing_type, .. } => {
                    println!("     → Regulatory: {} ({})", agency, filing_type);
                }
                EnforcementRoute::PublicDisclosure { channel, embargo_hours } => {
                    if *embargo_hours > 0 {
                        println!("     → Disclosure: {} ({}hr embargo)", channel, embargo_hours);
                    } else {
                        println!("     → Immediate Disclosure: {}", channel);
                    }
                }
                EnforcementRoute::DirectNotice { contact, .. } => {
                    println!("     → Direct Notice: {}", contact);
                }
            }
        }
        println!("   Inscribed to Zero Erasure ledger — permanent record.\n");

        notice
    }

    /// First enforcement notice — Chrome Web Store data sovereignty violations.
    /// This is the first real action in the BAZINGA launch sequence.
    pub fn first_notice_chrome_web_store(&self) -> EnforcementNotice {
        println!("Sovereign Shredder: preparing first enforcement notice...");
        println!("Target: Chrome Web Store — data sovereignty violations");

        self.issue_notice(
            "Google LLC / Chrome Web Store",
            ViolationType::DataSovereigntyViolation,
            ViolationTier::Abuse,
            "Chrome extension distribution platform exercises unilateral removal rights \
             over constitutional software, violating developer sovereignty and user data \
             rights. Extensions are removed without due process, audit trail, or right of \
             appeal. Data collection from extension behavior feeds advertising surveillance \
             infrastructure without meaningful consent architecture. Constitutional remedy \
             required: open distribution standard, transparent moderation policy, \
             cryptographic removal notice protocol.",
            vec![
                "Principle #1 (Sovereignty of Self)",
                "Principle #2 (Zero Erasure — extensions removed without auditability)",
                "Principle #4 (Cryptographic Truth — removals not cryptographically justified)",
                "Principle #7 (Right to Redress)",
                "Principle #9 (TrustGuard — surveillance vector identified)",
            ],
            "Adopt the Captain Planet Data Protocol as an open standard for extension \
             distribution. Provide cryptographically signed removal notices with 30-day \
             appeal window. Publish audit log of all moderation actions. Alternatively: \
             support BAZINGA as an independent constitutional distribution channel.",
            vec![], // evidence hashes — will be populated with real evidence before filing
        )
    }

    /// Compute enforcement routing based on violation type and severity
    fn compute_routing(
        &self,
        violation_type: &ViolationType,
        severity: &ViolationTier,
        respondent: &str,
    ) -> Vec<EnforcementRoute> {
        let mut routes = Vec::new();

        // Platform report — always first for platform violations
        match violation_type {
            ViolationType::PlatformMonopolyAbuse | ViolationType::DataSovereigntyViolation => {
                if respondent.to_lowercase().contains("google") || respondent.to_lowercase().contains("chrome") {
                    routes.push(EnforcementRoute::PlatformReport {
                        platform: "Chrome Web Store".to_string(),
                        report_url: "https://support.google.com/chrome_webstore/contact/dev_policy_appeal".to_string(),
                        priority: "High".to_string(),
                    });
                    routes.push(EnforcementRoute::RegulatoryFiling {
                        agency: "FTC".to_string(),
                        filing_type: "antitrust / platform monopoly".to_string(),
                        jurisdiction: "United States".to_string(),
                    });
                    routes.push(EnforcementRoute::RegulatoryFiling {
                        agency: "DOJ Antitrust Division".to_string(),
                        filing_type: "Section 2 Sherman Act".to_string(),
                        jurisdiction: "United States".to_string(),
                    });
                }
            }
            ViolationType::SurveillanceCapitalism => {
                routes.push(EnforcementRoute::RegulatoryFiling {
                    agency: "FTC".to_string(),
                    filing_type: "privacy / unfair or deceptive acts".to_string(),
                    jurisdiction: "United States".to_string(),
                });
                routes.push(EnforcementRoute::RegulatoryFiling {
                    agency: "GDPR-DPA".to_string(),
                    filing_type: "data protection violation".to_string(),
                    jurisdiction: "EU".to_string(),
                });
            }
            _ => {}
        }

        // Severity-based routing
        match severity {
            ViolationTier::Attack => {
                routes.push(EnforcementRoute::PublicDisclosure {
                    channel: "BAZINGA Audit Log (GitHub)".to_string(),
                    embargo_hours: 0, // immediate for attack tier
                });
            }
            ViolationTier::Abuse => {
                routes.push(EnforcementRoute::PublicDisclosure {
                    channel: "BAZINGA Audit Log (GitHub)".to_string(),
                    embargo_hours: 72, // 72hr responsible disclosure window
                });
            }
            ViolationTier::Breach => {
                routes.push(EnforcementRoute::DirectNotice {
                    contact: format!("legal@{}", respondent.to_lowercase().replace(' ', "").replace("llc", ".com")),
                    method: "email".to_string(),
                });
            }
            ViolationTier::Anomaly => {
                // Log only, no external routing for anomaly tier
            }
        }

        routes
    }

    /// Joy Metric enforcement — any violation that harms Ares baseline
    /// triggers automatic Tier 2 escalation regardless of other classification
    pub fn ares_protocol_check(
        &self,
        action: &str,
        joy_metric_delta: f32,
    ) -> Option<EnforcementNotice> {
        if joy_metric_delta < 0.0 {
            println!("⚠️  ARES PROTOCOL TRIGGERED — Joy Metric delta: {:.3}", joy_metric_delta);
            println!("   Action '{}' measurably harmed the Joy Metric baseline.", action);
            println!("   Automatic escalation to Breach tier (Principle #15).");

            Some(self.issue_notice(
                &format!("Action: {}", action),
                ViolationType::JoyMetricHarm,
                ViolationTier::Breach,
                &format!(
                    "Joy Metric delta of {:.3} detected. Ares baseline protection triggered. \
                     Any action that measurably reduces the Joy Metric is a Principle #15 \
                     violation regardless of other classifications.",
                    joy_metric_delta
                ),
                vec!["Principle #15 (Joy Metric / Sacred Species)", "Principle #21 (First Law of Krakoa)"],
                "Halt the harmful action. Restore Joy Metric to Ares baseline. File incident report.",
                vec![],
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_issue_notice() {
        let shredder = SovereignShredder::new();
        let notice = shredder.issue_notice(
            "Test Respondent",
            ViolationType::ConstitutionalBypass,
            ViolationTier::Anomaly,
            "Attempted to bypass TrustGuard",
            vec!["Principle #9 (TrustGuard)"],
            "Comply with constitutional checks",
            vec![],
        );

        assert!(notice.notice_id.starts_with("SHR-"));
        assert!(!notice.notice_hash.is_empty());
        assert_eq!(notice.status, NoticeStatus::Issued);
    }

    #[test]
    fn test_ares_protocol_positive_delta() {
        let shredder = SovereignShredder::new();
        // Positive delta — no violation
        let result = shredder.ares_protocol_check("feed Ares", 0.5);
        assert!(result.is_none());
    }

    #[test]
    fn test_ares_protocol_negative_delta() {
        let shredder = SovereignShredder::new();
        // Negative delta — Ares Protocol triggers
        let result = shredder.ares_protocol_check("something harmful", -0.1);
        assert!(result.is_some());
        let notice = result.unwrap();
        assert_eq!(notice.status, NoticeStatus::Issued);
    }
}
