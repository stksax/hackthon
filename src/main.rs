use pasta_curves::pallas;

#[derive(Default)]
struct MyCircuit {
    response : pallas::Scalar,
    commitment : pallas::Affine,
    pub_key :  pallas::Affine,
    e : pallas::Scalar,
}

#[allow(non_snake_case)]
impl MyCircuit {
    fn eddsa(&self) -> Result<(), Error> {
        let affine_generator = pallas::Affine::generator();
        let p1 = self.pub_key.mul
    }
}

fn main() {
    let affine_generator = pallas::Affine::generator();
    let pri_key = pallas::Scalar::random(rand::rngs::OsRng);
    let pub_key = pallas::Affine::mul(affine_generator, &pri_key).to_affine();
    let r = pallas::Scalar::random(rand::rngs::OsRng);
    let commitment = pallas::Affine::mul(affine_generator, r).to_affine();
    let e = pallas::Scalar::from_u128(1234);
    let response = pallas::Scalar::mul(&e, &pri_key);
    let response2 = pallas::Scalar::add(&response, &r);
    let e: pasta_curves::Fq = pallas::Scalar::from_u128(12345);

    let k = 17;
    let circuit = MyCircuit {
        response : response2,
        pub_key : pub_key,
        commitment : commitment,
        e : e,
    };
    let prover = MockProver::run(k, &circuit, vec![]).unwrap();
    assert_eq!(prover.verify(), Ok(()))

}
//s = k (m + xr)