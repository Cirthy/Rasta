pub const	SECURITY_LEVEL: usize = 8;
pub const	NB_ROUNDS: usize = 4;
pub const	BLOCK_SIZE: usize = 10;


mod	rasta_operations;


// TODO: set parameters depending on security level
// TODO: make the project a library
// TODO: handle genericBits
// TODO: make Rasta an implementation


use itertools::izip;
use	rasta_operations::stream;
use rand::Rng;

use	rasta_operations::sbox;


fn	main()
{
	let	message_size = 100;

	let	mut key = vec![Default::default(); BLOCK_SIZE];

	let	message = random_vector(message_size);
	let mut ct = vec![Default::default(); message_size];
	let mut decryption = vec![Default::default(); message_size];

	encrypt(&mut ct, &message, &key);
	decrypt(&mut decryption, &ct, &key);

	assert_eq!(message, decryption);

}


fn	encrypt(ct: &mut [bool], message: &[bool], key: &[bool])
{
	for (c, m) in izip!(ct, message){
		*c = *m ^ stream(key);
	}
}


fn	decrypt(message: &mut [bool], ct: &[bool], key: &[bool])
{
	for (m, c) in izip!(message, ct){
		*m = *c ^ stream(key);
	}
}


fn	random_vector(size: usize) -> Vec<bool>
{
	let mut rng = rand::thread_rng();
	let mut res = vec![true; size];
	rng.fill(&mut res[..]);

	res
}