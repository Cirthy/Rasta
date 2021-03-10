fn	xor(rop: &mut bool, a: bool, b: bool)
{
	*rop = a ^ b;
}


fn	and(rop: &mut bool, a: bool, b: bool)
{
	*rop = a & b;
}


fn	scal(rop: &mut bool, a: &[bool], b: &[bool])
{
	*rop = false;
	for (i, j) in izip!(a, b){
		rop ^= *a & *b;
	}
}


fn	mat_prod()
{
	
}


fn	vec_xor(rop: &mut [bool], a: &[bool], b: &[bool])
{
	for (i, j, k) in izip!(rop, a, b){
		*i = *j ^ *k;
	}
}