fn main() {
	let v = vec![1, 2, 3];

	let v2 = v;

	println!("{}", v2[0]);

	//println!("{}", v[0]) will not compile

	// Copy types
	let a = false;

	let b = change_truth(a);

	// We can still use a because b is a complete copy of a
	println!("{}", a);
	println!("{}", b);
}

fn change_truth(x: bool) -> bool {
	!x
}
