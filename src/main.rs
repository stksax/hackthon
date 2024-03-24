use std::{hash::{Hash, Hasher}, ops::{Add, Mul}, result};
use halo2_proofs::arithmetic::Field;
use rand::Rng;
use pasta_curves::{group::{cofactor::CofactorCurveAffine, ff::PrimeField, Curve, Group}, pallas};
use std::collections::hash_map::DefaultHasher;
mod key_generate;
use key_generate::*;
mod tool;
use tool::*;

pub fn pre_compute(
    pri : pallas::Scalar,
    input_r : pallas::Scalar,
    message : u128,
) -> (pallas::Affine, pallas::Scalar){
    let affine_generator = pallas::Affine::generator();
    let r = pallas::Affine::mul(affine_generator, input_r).to_affine();
    let mut hasher = DefaultHasher::new();
    hasher.write_u128(message);
    let hash_value = hasher.finish() as u128;
    let temp = pallas::Scalar::mul(&pri, &pallas::Scalar::from_u128(hash_value));
    let s = pallas::Scalar::add(&input_r, &temp);
    (r, s)
}
struct Eddsa {
    response : pallas::Scalar,
    commitment : pallas::Affine,
    pub_key :  pallas::Affine,
    e : pallas::Scalar,
}

#[allow(non_snake_case)]
impl Eddsa {
    fn eddsa(&self) -> bool {
        let affine_generator = pallas::Affine::generator();
        let p1 = pallas::Affine::mul( self.pub_key, &self.e).to_affine();
        let p2 = pallas::Affine::add(p1, self.commitment).to_affine();
        let response = pallas::Affine::mul(affine_generator, self.response).to_affine();
        let result = pallas::Affine::eq(&p2,&response);
        result
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
fn test() {
    //there are 5 player join teh key generation
    let player1 = generate_random_u128_in_range(1, std::u64::MAX as u128);
    let player2 = generate_random_u128_in_range(1, std::u64::MAX as u128);
    let player3 = generate_random_u128_in_range(1, std::u64::MAX as u128);
    let player4 = generate_random_u128_in_range(1, std::u64::MAX as u128);
    let player5 = generate_random_u128_in_range(1, std::u64::MAX as u128);

    let input1 = Input{
        key_share : player1,
        rand_num : 379278,
        output_max : 5,
        output_min : 3,
    };
    let result1 = input1.output_key_share();

    let input2 = Input{
        key_share : player2,
        rand_num : 4812738974,
        output_max : 5,
        output_min : 3,
    };
    let result2 = input2.output_key_share();

    let input3 = Input{
        key_share : player3,
        rand_num : 43217,
        output_max : 5,
        output_min : 3,
    };
    let result3 = input3.output_key_share();

    let input4 = Input{
        key_share : player4,
        rand_num : 12343432,
        output_max : 5,
        output_min : 3,
    };
    let result4 = input4.output_key_share();

    let input5 = Input{
        key_share : player5,
        rand_num : 1234546,
        output_max : 5,
        output_min : 3,
    };
    let result5 = input5.output_key_share();

    let mut user_vec = Vec::new();
    user_vec.extend(result1);
    user_vec.extend(result2);
    user_vec.extend(result3);
    user_vec.extend(result4);
    user_vec.extend(result5);

    let user1 = CollectOutputKeyShare{
        key_share : user_vec.clone(),
        member : 5,
        self_num : 1,
    };
    let (user1_prikey_share_a, user1_pubket_share) = user1.collect();
    let calculate_user1_prikey_share = CalculatePriKey {
        self_coefficient : 1,
        coefficient : [2,3],
        pri_key : user1_prikey_share_a,
    };
    let user1_prikey_share = calculate_user1_prikey_share.calculate();
    
    let user2 = CollectOutputKeyShare{
        key_share : user_vec.clone(),
        member : 5,
        self_num : 2,
    };
    let (user2_prikey_share_a, user2_pubket_share) = user2.collect();
    let calculate_user2_prikey_share = CalculatePriKey {
        self_coefficient : 2,
        coefficient : [1,3],
        pri_key : user2_prikey_share_a,
    };
    let user2_prikey_share = calculate_user2_prikey_share.calculate();

    let user3 = CollectOutputKeyShare{
        key_share : user_vec.clone(),
        member : 5,
        self_num : 3,
    };
    let (user3_prikey_share_a, user3_pubket_share) = user3.collect();
    let calculate_user3_prikey_share = CalculatePriKey {
        self_coefficient : 3,
        coefficient : [1,2],
        pri_key : user3_prikey_share_a,
    };
    let user3_prikey_share = calculate_user3_prikey_share.calculate();

    let pub_key_calaulate = CalculatePubKey {
        degree : 3,
        coefficient : [1,2,3].to_vec(),
        pub_key : [user1_pubket_share, user2_pubket_share, user3_pubket_share].to_vec(),
    };
    //they make the public key
    let pub_key = pub_key_calaulate.calculate();
    //message is the thing they want to vote
    let message = generate_random_u128_in_range(1, std::u64::MAX as u128);

    let (r1, s1) = pre_compute(
        user1_prikey_share, 
        pallas::Scalar::random(rand::rngs::OsRng),
        message,
    );

    let (r2, s2) = pre_compute(
        user2_prikey_share, 
        pallas::Scalar::random(rand::rngs::OsRng),
        message,
    );

    let (r3, s3) = pre_compute(
        user3_prikey_share, 
        pallas::Scalar::random(rand::rngs::OsRng),
        message,
    );
    
    let mut commitment = pallas::Affine::add(r1, &r2).to_affine();
    commitment = pallas::Affine::add(commitment, &r3).to_affine();

    let mut response = pallas::Scalar::add(&s1, &s2);
    response = pallas::Scalar::add(&response, &s3);
    let mut hasher = DefaultHasher::new();
    hasher.write_u128(message);
    let challange = hasher.finish() as u128;
    //this is just for make sure the user1_prikey_share add together is as our expect 
    let check1 = player1 + player2 + player3 + player4 + player5;
    let check2 = user1_prikey_share + user2_prikey_share + user3_prikey_share;
    let check3 = pallas::Scalar::from_u128(check1);
    let pri_key_equal = pallas::Scalar::eq(&check2, &check3);
    assert_eq!(pri_key_equal,true);

    let circuit = Eddsa{
        response : response,
        pub_key : pub_key,
        commitment : commitment,
        e : pallas::Scalar::from_u128(challange),
    };
    let result = circuit.eddsa();
    assert_eq!(result,true);
}
}

