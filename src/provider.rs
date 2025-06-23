//! Cryptographic providers for signing and verification

use bytes::Bytes;
use malachitebft_core_types::{
    SignedExtension, SignedProposal, SignedProposalPart,
    SignedVote, SigningProvider, SigningScheme,
};
pub use malachitebft_signing_ed25519::{PrivateKey, PublicKey, Signature};
use rand::{CryptoRng, RngCore};

use crate::context::{BaseProposal, BaseProposalPart, BaseVote, MalachiteContext};

/// Ed25519 signing provider for Malachite consensus
#[derive(Debug, Clone)]
pub struct Ed25519Provider {
    private_key: PrivateKey,
}

impl Ed25519Provider {
    /// Create a new provider with a private key
    pub fn new(private_key: PrivateKey) -> Self {
        Self { private_key }
    }

    /// Create a new provider with a default/test key
    pub fn new_test() -> Self {
        let private_key = PrivateKey::generate(&mut rand::thread_rng());
        Self::new(private_key)
    }

    /// Get the public key
    pub fn public_key(&self) -> PublicKey {
        self.private_key.public_key()
    }

    /// Get the private key reference
    pub fn private_key(&self) -> &PrivateKey {
        &self.private_key
    }

    /// Sign raw bytes
    pub fn sign(&self, data: &[u8]) -> Signature {
        self.private_key.sign(data)
    }

    /// Verify a signature
    pub fn verify(&self, data: &[u8], signature: &Signature, public_key: &PublicKey) -> bool {
        public_key.verify(data, signature).is_ok()
    }
}

impl Default for Ed25519Provider {
    fn default() -> Self {
        Self::new_test()
    }
}

impl SigningScheme for Ed25519Provider {
    type PrivateKey = PrivateKey;
    type PublicKey = PublicKey;
    type Signature = Signature;
    type DecodingError = std::io::Error;

    fn decode_signature(bytes: &[u8]) -> Result<Self::Signature, Self::DecodingError> {
        if bytes.len() != 64 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid signature length",
            ));
        }
        let mut sig_bytes = [0u8; 64];
        sig_bytes.copy_from_slice(bytes);
        Ok(Signature::from_bytes(sig_bytes))
    }

    fn encode_signature(signature: &Self::Signature) -> Vec<u8> {
        signature.to_bytes().to_vec()
    }
}

// Implement the to_sign_bytes trait for our types
pub trait ToSignBytes {
    fn to_sign_bytes(&self) -> Vec<u8>;
}

impl ToSignBytes for BaseVote {
    fn to_sign_bytes(&self) -> Vec<u8> {
        // For now, use a simple serialization
        // In production, this should match the canonical serialization format
        bincode::serialize(self).unwrap_or_default()
    }
}

impl ToSignBytes for BaseProposal {
    fn to_sign_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap_or_default()
    }
}

impl ToSignBytes for BaseProposalPart {
    fn to_sign_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap_or_default()
    }
}

impl SigningProvider<MalachiteContext> for Ed25519Provider {
    fn sign_vote(&self, vote: BaseVote) -> SignedVote<MalachiteContext> {
        let signature = self.sign(&vote.to_sign_bytes());
        SignedVote::new(vote, signature)
    }

    fn verify_signed_vote(
        &self,
        vote: &BaseVote,
        signature: &Signature,
        public_key: &PublicKey,
    ) -> bool {
        public_key.verify(&vote.to_sign_bytes(), signature).is_ok()
    }

    fn sign_proposal(&self, proposal: BaseProposal) -> SignedProposal<MalachiteContext> {
        let signature = self.sign(&proposal.to_sign_bytes());
        SignedProposal::new(proposal, signature)
    }

    fn verify_signed_proposal(
        &self,
        proposal: &BaseProposal,
        signature: &Signature,
        public_key: &PublicKey,
    ) -> bool {
        public_key.verify(&proposal.to_sign_bytes(), signature).is_ok()
    }

    fn sign_proposal_part(&self, proposal_part: BaseProposalPart) -> SignedProposalPart<MalachiteContext> {
        let signature = self.sign(&proposal_part.to_sign_bytes());
        SignedProposalPart::new(proposal_part, signature)
    }

    fn verify_signed_proposal_part(
        &self,
        proposal_part: &BaseProposalPart,
        signature: &Signature,
        public_key: &PublicKey,
    ) -> bool {
        public_key.verify(&proposal_part.to_sign_bytes(), signature).is_ok()
    }

    fn sign_vote_extension(&self, extension: Bytes) -> SignedExtension<MalachiteContext> {
        let signature = self.sign(extension.as_ref());
        malachitebft_core_types::SignedMessage::new(extension, signature)
    }

    fn verify_signed_vote_extension(
        &self,
        extension: &Bytes,
        signature: &Signature,
        public_key: &PublicKey,
    ) -> bool {
        public_key.verify(extension.as_ref(), signature).is_ok()
    }
}