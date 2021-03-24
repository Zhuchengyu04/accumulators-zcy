// A very basic example.
use accumulator::Accumulator;
use accumulator::group::Rsa2048;
use accumulator::VectorCommitment;
use accumulator::hash::hash_to_prime;
use rug::Integer;
fn main(){
  // let acc = Accumulator::<Rsa2048, &'static str>::empty();
  let vc = VectorCommitment::<Rsa2048>::empty();
  let (mut vc, mut proof) = VectorCommitment::update(vc,&[Integer::from(1), Integer::from(2)],&[(true,Integer::from(1)),(true, Integer::from(2))]).unwrap();
  assert!(VectorCommitment::verify(&vc,&[(true,Integer::from(1))],&proof));

  // Accumulate "dog" and "cat". The `add_with_proof` method returns the new accumulator state
  // and a proof that you accumulated "dog" and "cat".
  // let (acc, proof) = acc.add_with_proof(&["dog", "cat"]);

  // A network participant who sees (acc, proof, and ["dog", "cat"]) can verify that the update
  // was formed correctly ...
  // assert!(acc.verify_membership_batch(&["dog", "cat"], &proof));

  // ... and trying to verify something that has not been accumulated will fail.
  // assert!(!acc.verify_membership(&"cow", &proof));
}
