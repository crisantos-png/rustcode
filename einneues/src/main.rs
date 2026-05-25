fn main(){
	/*const X: i32 = 24 * 365;
	const X_HOURS_IN_SECONDS: i32 = 60 * 60 * X;

	println!("{X} hours is {X_HOURS_IN_SECONDS} seconds");*/

	/*let x = 5;
	println!("The first value of x is {x}");
	let x = x + 2;

	{
		let x = x * 2;
		println!("The value of x in the inner scope is {x}");
	}
	println!("The final value of x is {x}");*/

	/*
	The integer variable type in rust can be unsingned int (u) or a signed int (i) which depend on whether it can only be positive (u) or it can be both positive and negaitve (i)
	The signed int variable can be of different forms: i8, i16, i32, i64, i128, isize. representing how many bits that variant uses: each can store from -(2^(n-1)) to 2^(n-1) -1
	The unsigned int can also have u8, u16, u32, u64, u128, usize; and also representing the bits used by the variant in memory: each can store 0 to 2^(n-1)
	Those represent 8-bit, 16-bit, 32-bit, 64-bit, 128-bit and arch reespectively
	arch used by isize and usize depends on the architecture of the computer you use, which can be a 32 bit architecture or a 64 bit architecture
	*/
	let quotient = 56.7 / 19.2;
	println!("{quotient}");
}
