//! Traits for working with zero-knowledge proofs.

use rand_core::RngCore;

/// Trait for a zero-knowledge proof about some statement.
pub trait Proof: Sized {
    /// The parameters necessary to create a proof.
    type ProvingKey;

    /// The parameters necessary to verify a proof.
    type VerifyingKey;

    /// The input commonly known to both the prover and verifier.
    type Instance;

    /// The private input to the prover.
    type Witness;

    /// Errors that may occur while creating or verifying proofs.
    type Error;

    /// Creates a proof that the given instance and witness satisfy the statement.
    fn create<R: RngCore>(
        pk: &Self::ProvingKey,
        instance: &Self::Instance,
        witness: &Self::Witness,
        rng: R,
    ) -> Result<Self, Self::Error>;

    /// Verifies that this proof satisfies the statement for the given instance.
    fn verify(&self, vk: &Self::VerifyingKey, instance: &Self::Instance)
        -> Result<(), Self::Error>;
}
