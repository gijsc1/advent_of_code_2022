use std::ops::{Mul, MulAssign};
use num_traits::Pow;
use uint::construct_uint;
//This ended up not working, because even a 2000 bit unsigned integer was still not big enough, and assigning 4000 bits crashed the build system.
construct_uint! {
	pub struct U1024(16);
}

// pub fn number_testing(){
// 	let base:U1024 = 2.into();
// 	let exp:U1024 = 200.into();
// 	let x:U1024 = (base.pow(exp)).into();
// 	let y:U1024 = (base.pow(exp)).into();
// 	let res = x+y;
// 	// println!("control group: {} + {} = {}",2^4,2^4,(2^4)+(2^4));
// 	println!("test res {x} + {y} = {res}");
// }

// impl Pow<u16> for U1024{
// 	type Output = U1024;
//
// 	///only does squaring, but that was all I needed.
// 	fn pow(self, rhs: u16) -> Self::Output {
// 		self.mul(self)
// 	}
// }