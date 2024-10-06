use bellman::{
    groth16::{
        create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof, Proof,
    },
    Circuit, ConstraintSystem, SynthesisError,
};
use bls12_381::{Bls12, Scalar};
use rand::thread_rng;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
struct ProofOfActionCircuit {
    private_action: Option<Scalar>,
    public_threshold: Option<Scalar>,
}

impl Circuit<Scalar> for ProofOfActionCircuit {
    fn synthesize<CS: ConstraintSystem<Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let action = cs.alloc_input(
            || "action",
            || self.private_action.ok_or(SynthesisError::AssignmentMissing),
        )?;
        let threshold = cs.alloc_input(
            || "threshold",
            || {
                self.public_threshold
                    .ok_or(SynthesisError::AssignmentMissing)
            },
        )?;

        cs.enforce(
            || "action >= threshold",
            |lc| lc + action - threshold,
            |lc| lc + CS::one(),
            |lc| lc,
        );

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ProofData {
    proof: String,
    public_inputs: Vec<String>,
}

fn generate_proof<C: Circuit<Scalar> + Clone>(circuit: C, public_inputs: &[Scalar]) -> ProofData {
    let rng = &mut thread_rng();

    let params = generate_random_parameters::<Bls12, _, _>(circuit.clone(), rng).unwrap();
    let proof = create_random_proof(circuit, &params, rng).unwrap();

    let proof_str = format!("{:?}", proof);

    ProofData {
        proof: proof_str,
        public_inputs: public_inputs.iter().map(|s| s.to_string()).collect(),
    }
}

pub fn proof_of_action(secret_value: Scalar) {
    let threshold = Scalar::from(10u64);
    let circuit = ProofOfActionCircuit {
        private_action: Some(secret_value),
        public_threshold: Some(threshold),
    };
    let proof_data = generate_proof(circuit, &[threshold]);

    match send_proof(&proof_data) {
        Ok(_) => println!("Proof sent successfully"),
        Err(e) => println!("Failed to send proof: {}", e),
    }
}

fn send_proof(proof_data: &ProofData) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    let proof_data_text = format!("{:?}", proof_data.proof);

    println!("Sending proof data: {}", proof_data_text);

    let res = client
        .post("http://localhost:8080/verify")
        .header("Content-Type", "text/plain")
        .body(proof_data_text)
        .send()?;

    if res.status().is_success() {
        println!("Server response: {}", res.text()?);
        Ok(())
    } else {
        Err(format!("Server error: {}", res.status()).into())
    }
}

fn main() {
    let secret_value = Scalar::from(5u64);
    proof_of_action(secret_value);
}
