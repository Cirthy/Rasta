use crate::BLOCK_SIZE;
use crate::NB_ROUNDS;


pub fn	stream(key: &[bool], ) -> bool
{
	false
}


fn	round(key: &[bool])
{

}


fn	affine_layer()	// y = Ax + c
{
	// generate A and c
	
}


pub fn	sbox(rop: &mut[bool], x: &[bool])
{
	for (i, r) in rop.iter_mut().enumerate(){
		*r = x[i] ^ x[(i + 2) % BLOCK_SIZE] ^ (x[(i + 1) % BLOCK_SIZE] & x[(i + 2) % BLOCK_SIZE]);
	}
}
