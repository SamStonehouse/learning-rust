fn main() {
	let v1 = vec![1,2,3];
	let v2 = vec![3,4,5];

	let (v1, v2, answer) = foo(v1,v2);

	println!("{}", v1[0]);
	println!("{}", v2[0]);
	println!("{}", answer);

	let v3 = vec![5,6,7];
	let v4 = vec![6,7,8];

	borrowing_foo(&v3, &v4);

	// We can still use v3 and v4
	println!("{} {}", v3[0], v4[0]);

	// References are immutable
	foo_wrong(&v3);

	// But we can make them mutable
	let mut x = 8;
	{
		let mut y = &mut x;
		*y += 2;
	}
	println!("{}", x);
}

// We can manually pass back ownership by returning and re-assigning
fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
	(v1, v2, 42)
}

fn borrowing_foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
	let v = *v1;
	println!("{} {}", v[0], v2[0]);
	42
}

fn foo_wrong(v: &Vec<i32>) {
	// We can't do this as it's immutable
	//v.push(5);
	println!("{}", v[0]);
}
