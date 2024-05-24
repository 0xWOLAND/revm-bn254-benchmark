use bn::{AffineG1, AffineG2, Fq, Fq2, Fr, Group, G1, G2};
use hex;
use rand::Rng;
use sp1_sdk::{utils, ProverClient, SP1Stdin};
const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn encode_g1(g1: G1) -> String {
    let g1 = AffineG1::from_jacobian(g1).unwrap();
    let mut output = [0u8; 64];
    g1.x().to_big_endian(&mut output[..32]).unwrap();
    g1.y().to_big_endian(&mut output[32..]).unwrap();
    hex::encode(&output)
}

fn encode_g2(g2: G2) -> String {
    let g2 = AffineG2::from_jacobian(g2).unwrap();
    let mut output = [0u8; 128];
    g2.x().real().to_big_endian(&mut output[..32]).unwrap();
    g2.x()
        .imaginary()
        .to_big_endian(&mut output[32..64])
        .unwrap();
    g2.y().real().to_big_endian(&mut output[64..96]).unwrap();
    g2.y().imaginary().to_big_endian(&mut output[96..]).unwrap();
    hex::encode(&output)
}

fn encode_fr(fr: Fr) -> String {
    let mut output = [0u8; 32];
    fr.to_big_endian(&mut output).unwrap();
    hex::encode(&output)
}

fn main() {
    utils::setup_logger();

    let mut stdin: SP1Stdin = SP1Stdin::new();
    let mut rng = rand::thread_rng();

    let client = ProverClient::new();
    let (pk, _) = client.setup(ELF);

    let g1_a = G1::random(&mut rng);
    let g1_b = G1::random(&mut rng);
    let g1_c = G1::random(&mut rng);
    let fr_d1 = Fr::random(&mut rng);

    let g2_a = G2::random(&mut rng);
    let g2_b = G2::random(&mut rng);
    let g2_c = G2::random(&mut rng);
    let fr_d2 = Fr::random(&mut rng);

    let g1_sum_a_b = g1_a + g1_b;
    let g1_mul_c_d = g1_c * fr_d1;
    let g2_sum_a_b = g2_a + g2_b;
    let g2_mul_c_d = g2_c * fr_d2;

    stdin.write(&encode_g1(g1_a));
    stdin.write(&encode_g1(g1_b));
    stdin.write(&encode_g1(g1_c));
    stdin.write(&encode_fr(fr_d1));

    stdin.write(&encode_g2(g2_a));
    stdin.write(&encode_g2(g2_b));
    stdin.write(&encode_g2(g2_c));
    stdin.write(&encode_fr(fr_d2));

    let mut proof = client
        .prove_compressed(&pk, stdin)
        .expect("Failed to prove");

    let _g1_sum_a_b = proof.public_values.read::<String>();
    let _g1_mul_c_d = proof.public_values.read::<String>();
    let _g2_sum_a_b = proof.public_values.read::<String>();
    let _g2_mul_c_d = proof.public_values.read::<String>();

    assert_eq!(encode_g1(g1_sum_a_b), _g1_sum_a_b);
    assert_eq!(encode_g1(g1_mul_c_d), _g1_mul_c_d);
    assert_eq!(encode_g2(g2_sum_a_b), _g2_sum_a_b);
    assert_eq!(encode_g2(g2_mul_c_d), _g2_mul_c_d);
}
