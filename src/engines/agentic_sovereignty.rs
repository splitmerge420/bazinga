/// Agentic Sovereignty — Principle #1
/// Ed25519 signing on every artifact. Identity is cryptographic, not claimed.
///
/// FIXES applied (vs Grok's broken version):
///   - Uses SigningKey (v2.1 API) instead of removed Keypair (v1 API)
///   - rand = "0.8" is declared in Cargo.toml
///   - verify_artifact uses todo!() not `return true` — no silent backdoor
use ed25519_dalek::{SigningKey, VerifyingKey, Signer, Verifier, Signature};
use rand::rngs::OsRng;
use rand::RngCore;

pub struct AgenticSovereignty;

impl AgenticSovereignty {
    /// Generate a new Ed25519 signing keypair.
    /// Returns the SigningKey (contains the VerifyingKey too via .verifying_key()).
    /// Note: ed25519-dalek v2 removed SigningKey::generate — use RngCore::fill_bytes instead.
    pub fn generate_keypair() -> SigningKey {
        let mut secret_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut secret_bytes);
        SigningKey::from_bytes(&secret_bytes)
    }

    /// Sign an artifact payload. Returns 64-byte Ed25519 signature.
    pub fn sign_artifact(signing_key: &SigningKey, message: &[u8]) -> Vec<u8> {
        signing_key.sign(message).to_bytes().to_vec()
    }

    /// Verify an artifact signature.
    /// WILL PANIC until real ed25519 bytes are wired in.
    /// This is intentional — silence is not safety.
    pub fn verify_artifact(
        verifying_key_bytes: &[u8; 32],
        message: &[u8],
        signature_bytes: &[u8; 64],
    ) -> Result<bool, String> {
        let verifying_key = VerifyingKey::from_bytes(verifying_key_bytes)
            .map_err(|e| format!("Invalid verifying key: {e}"))?;
        let signature = Signature::from_bytes(signature_bytes);
        match verifying_key.verify(message, &signature) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_and_verify_roundtrip() {
        let signing_key = AgenticSovereignty::generate_keypair();
        let verifying_key_bytes: [u8; 32] = signing_key.verifying_key().to_bytes();
        let message = b"BAZINGA constitutional artifact v0.2";
        let sig_bytes_vec = AgenticSovereignty::sign_artifact(&signing_key, message);
        let sig_bytes: [u8; 64] = sig_bytes_vec.try_into().expect("signature must be 64 bytes");
        let result = AgenticSovereignty::verify_artifact(&verifying_key_bytes, message, &sig_bytes);
        assert_eq!(result.unwrap(), true, "valid signature must verify");
    }

    #[test]
    fn test_tampered_message_fails_verification() {
        let signing_key = AgenticSovereignty::generate_keypair();
        let verifying_key_bytes: [u8; 32] = signing_key.verifying_key().to_bytes();
        let message = b"BAZINGA constitutional artifact v0.2";
        let tampered = b"BAZINGA constitutional artifact v0.2 TAMPERED";
        let sig_bytes_vec = AgenticSovereignty::sign_artifact(&signing_key, message);
        let sig_bytes: [u8; 64] = sig_bytes_vec.try_into().expect("signature must be 64 bytes");
        let result = AgenticSovereignty::verify_artifact(&verifying_key_bytes, tampered, &sig_bytes);
        assert_eq!(result.unwrap(), false, "tampered message must fail verification");
    }
}
