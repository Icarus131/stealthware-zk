mod zk_proof;
use bls12_381::Scalar;
use zk_proof::proof_of_action;

fn main() {
    let secret_value = Scalar::from(42u64);
    proof_of_action(secret_value);
}
