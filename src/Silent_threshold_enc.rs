use crate::{to_array_32, DataType, EncryptionScheme, EncryptionSchemeInputs};
use ark_ec::pairing::Pairing;
use ark_poly::univariate::DensePolynomial;
use ark_std::{UniformRand, Zero};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use silent_threshold_encryption::{
    decryption::decrypt,
    encryption::encrypt1,
    kzg::KZG10,
    setup::{AggregateKey, LagrangePowers, PublicKey, SecretKey},
};

// change benchmark.rs to get input from the user and also change nech_decrypt to use it as decrypt function benchmarking
pub struct STE;
impl STE{
  pub fn get_inputs( data_type: DataType, n: usize) -> EncryptionSchemeInputs {
    EncryptionSchemeInputs::SilentThreshold(data_type, n)
}
}

type E = ark_bls12_381::Bls12_381;
type G2 = <E as Pairing>::G2;
type Fr = <E as Pairing>::ScalarField;
type UniPoly381 = DensePolynomial<<E as Pairing>::ScalarField>;

impl EncryptionScheme for STE {
  fn scheme_name(&self) -> String {
      String::from("SilentThreshold")
  }

  fn get_inputs(
      &self,
      // batch_size: usize,
      data_type: DataType,
      n: usize,
  ) -> EncryptionSchemeInputs {
      EncryptionSchemeInputs::SilentThreshold( data_type, n)
  }

// in my input the n and dataype changes 
fn encrypt(&self, encryption_scheme_inputs: EncryptionSchemeInputs) {
  match encryption_scheme_inputs {
    EncryptionSchemeInputs::SilentThreshold( data, n) => {
        let mut rng = ark_std::test_rng();
        
  // let n = 8;
  let t = n/2;
  let tau = Fr::rand(&mut rng);
  let params = KZG10::<E, UniPoly381>::setup(n, tau.clone()).unwrap();

  let mut sk: Vec<SecretKey<E>> = Vec::new();
  let mut pk: Vec<PublicKey<E>> = Vec::new();

  sk.push(SecretKey::<E>::new(&mut rng));
  sk[0].nullify();
  pk.push(sk[0].get_pk(0, &params, n));

  for i in 1..n {
      sk.push(SecretKey::<E>::new(&mut rng));
      pk.push(sk[i].get_pk(i, &params, n))
  }

  let ak = AggregateKey::<E>::new(pk, &params);
  let msg = &data.to_vec();
  let message_bytes = to_array_32(msg.to_vec());
  let ct = encrypt1::<E>(&ak, t, &params, message_bytes);

  // c.bench_function("encrypt", |b| {
  //     b.iter(|| encrypt1::<E>(&ak, t, &params, msg))
  // });
}

_ => unimplemented!("SilentThreshold : Invalid Inputs."),
  }}

fn decrypt(&self, encryption_scheme_inputs: EncryptionSchemeInputs) {
  match encryption_scheme_inputs {
    EncryptionSchemeInputs::SilentThreshold( data, n) => {
        let mut rng = ark_std::test_rng();
        
  // let n = 8;
  let t = n/2;
  let tau = Fr::rand(&mut rng);
  let params = KZG10::<E, UniPoly381>::setup(n, tau.clone()).unwrap();

  let mut sk: Vec<SecretKey<E>> = Vec::new();
  let mut pk: Vec<PublicKey<E>> = Vec::new();

 // create the dummy party's keys
 sk.push(SecretKey::<E>::new(&mut rng));
 sk[0].nullify();
 pk.push(sk[0].get_pk(0, &params, n));

 for i in 1..n {
     sk.push(SecretKey::<E>::new(&mut rng));
     pk.push(sk[i].get_pk(i, &params, n))
 }

  let ak = AggregateKey::<E>::new(pk, &params);
  let msg = &data.to_vec();
  let message_bytes = to_array_32(msg.to_vec());
  let ct = encrypt1::<E>(&ak, t, &params, message_bytes);
     // compute partial decryptions
      let mut partial_decryptions: Vec<G2> = Vec::new();
      for i in 0..t + 1 {
          partial_decryptions.push(sk[i].partial_decryption(&ct));
      }
      for _ in t + 1..n {
          partial_decryptions.push(G2::zero());
      }

      // compute the decryption key
      let mut selector: Vec<bool> = Vec::new();
      for _ in 0..t + 1 {
          selector.push(true);
      }
      for _ in t + 1..n {
          selector.push(false);
      }
let msg = decrypt(&ct,&partial_decryptions, &selector, &ak, &params);
  }


}}}








