//! An implementation of the FRI low-degree test (LDT).

#![no_std]

extern crate alloc;

use p3_commit::Mmcs;
use p3_ldt::{LdtBasedPcs, LDT};

pub use crate::proof::FriProof;
use crate::prover::prove;
use crate::verifier::verify;

mod config;
mod fold_even_odd;
mod proof;
mod prover;
mod verifier;

pub use config::*;
pub use proof::*;

pub struct FriLDT<FC: FriConfig> {
    config: FC,
}

impl<FC: FriConfig> LDT<FC::Val, FC::InputMmcs, FC::Chal> for FriLDT<FC> {
    type Proof = FriProof<FC>;
    type Error = ();

    fn prove(
        &self,
        inputs: &[<FC::InputMmcs as Mmcs<FC::Val>>::ProverData],
        challenger: &mut FC::Chal,
    ) -> Self::Proof {
        prove::<FC>(&self.config, inputs, challenger)
    }

    fn verify(
        &self,
        _input_commits: &[<FC::InputMmcs as Mmcs<FC::Val>>::Commitment],
        proof: &Self::Proof,
        challenger: &mut FC::Chal,
    ) -> Result<(), Self::Error> {
        verify::<FC>(proof, challenger)
    }
}

pub type FRIBasedPCS<FC, DFT> = LdtBasedPcs<
    <FC as FriConfig>::Val,
    <FC as FriConfig>::Challenge,
    DFT,
    <FC as FriConfig>::InputMmcs,
    FriLDT<FC>,
>;
