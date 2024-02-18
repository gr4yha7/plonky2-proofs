use anyhow::Result;
use plonky2::field::types::Field;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};

// Define parameters for this circuit
const D: usize = 2;
type C = PoseidonGoldilocksConfig;
type F = <C as GenericConfig<D>>::F;

// TODO build CLI app, accept JSON file

fn main() -> Result<()> {
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    // Build circuit for ploynomial x³ + 2xyz^2 − yz + 1 = x * x * x + 2 * x * y * z * z — y * z + 1
    let x = builder.add_virtual_target();
    let y = builder.add_virtual_target();
    let z = builder.add_virtual_target();

    let a =  builder.cube(x);
    let b =  builder.square(z);
    let c = builder.mul_many(vec![x, y, b]);
    let d = builder.mul_const(F::from_canonical_u32(2), c);
    let e = builder.mul(y, z);
    let f = builder.mul_const(F::NEG_ONE, e); // builder.neg(d)
    let g = builder.add_many(vec![a, d, f]);
    let h = builder.add_const(g, F::from_canonical_u32(1));
    

    // Public inputs are the initial values (provided below) and the result (which is generated).
    builder.register_public_input(x);
    builder.register_public_input(y);
    builder.register_public_input(z);
    builder.register_public_input(h);

    // Build the circuit
    let circuit_data = builder.build::<C>();

    // Now compute the witnesses and generate a proof
    let mut pw = PartialWitness::new();
    pw.set_target(x, F::from_canonical_u32(1));
    pw.set_target(y, F::from_canonical_u32(1));
    pw.set_target(z, F::from_canonical_u32(0));

    let proof = circuit_data.prove(pw).unwrap();

    println!(
        "x³ + 2xyz^2 − yz + 1 where x = {}, y = {}, z = {} is {}",
        proof.public_inputs[0],
        proof.public_inputs[1],
        proof.public_inputs[2],
        proof.public_inputs[3]
    );
    // Verify the proof
    circuit_data.verify(proof)
}
