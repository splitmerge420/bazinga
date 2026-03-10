// Agentic Sovereignty — BAZINGA v0.2
// Ed25519 signing on every artifact. Principle #1.
// FIXED: Uses ed25519-dalek v2.1 API (SigningKey, not deprecated Keypair)
// FIXED: rand declared as dependency in Cargo.toml
// FIXED: verify_artifact uses todo!() — will panic loudly if called before real impl

use ed25519_dalek::{SigningKey, Signature, Signer};
use rand::rngs::OsRng;
use sha2::{Sha256, Digest};

pub struct AgenticSovereignty;

impl AgenticSovereignty {
    /// Generate a new Ed25519 signing keypair (v2.1 API)
    pub fn generate_keypair() -> SigningKey {
        SigningKey::generate(&mut OsRng)
    }

    /// Sign an artifact — returns 64-byte Ed25519 signature
    pub fn sign_artifact(signing_key: &SigningKey, message: &[u8]) -> Vec<u8> {
        let signature: Signature = signing_key.sign(message);
        signature.to_bytes().to_vec()
    }

    /// Get verifying (public) key bytes from a signing key
    pub fn verifying_key_bytes(signing_key: &SigningKey) -> Vec<u8> {
        signing_key.verifying_key().to_bytes().to_vec()
    }

    /// Verify an artifact signature.
    /// STUB: Returns todo!() panic — must be implemented before Ring 1 goes live.
    /// DO NOT remove the todo!() and replace with `true`. That is a backdoor.
    pub fn verify_artifact(
        _public_key_bytes: &[u8; 32],
        _message: &[u8],
        _signature_bytes: &[u8; 64],
    ) -> bool {
        // TODO: Wire real verification before any Ring 1 deployment
        // Implementation will be:
        //   let verifying_key = VerifyingKey::from_bytes(public_key_bytes)?;
        //   let signature = Signature::from_bytes(signature_bytes);
        //   verifying_key.verify(message, &signature).is_ok()
        todo!("verify_artifact: real ed25519 verification not yet implemented. DO NOT SHIP WITHOUT THIS.")
    }

    /// Hash a message with SHA-256 for artifact provenance
    pub fn hash_artifact(data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }

    /// Generate a sovereign sphere ID from a name + timestamp
    pub fn generate_sphere_id(name: &str) -> String {
        let ts = chrono::Utc::now().timestamp();
        format!("did:key:{}_{}", name.to_lowercase().replace(' ', "_"), ts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_roundtrip() {
        let signing_key = AgenticSovereignty::generate_keypair();
        let message = b"BAZINGA constitutional artifact v0.2";
        let sig_bytes = AgenticSovereignty::sign_artifact(&signing_key, message);
        assert_eq!(sig_bytes.len(), 64, "Ed25519 signature must be 64 bytes");
    }

    #[test]
    fn test_hash_deterministic() {
        let data = b"Ares is the Joy Metric baseline";
        let h1 = AgenticSovereignty::hash_artifact(data);
        let h2 = AgenticSovereignty::hash_artifact(data);
        assert_eq!(h1, h2, "SHA-256 must be deterministic");
        assert_eq!(h1.len(), 32, "SHA-256 must produce 32 bytes");
    }

    #[test]
    fn test_sphere_id_format() {
        let id = AgenticSovereignty::generate_sphere_id("ares senior dog austin");
        assert!(id.starts_with("did:key:"), "Sphere ID must start with did:key:");
    }
}
