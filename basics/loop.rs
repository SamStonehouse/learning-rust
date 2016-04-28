fn main() {


	// Infinite loop
	// loop {
	//	println!("Loop forever");
	// }

	let mut x = 5;
	let mut done = false;

	while !done {
		x += x - 3;

		println!("{}", x);

		if x % 5 == 0 {
			done = true;
		}
	}

	// for var in expression {
	// 	code
	// }

	for n in 1..10 {
		println!("{}", n);
	}

	// Enumerate
	for (i,j) in (5..10).enumerate() {
		println!("{}: {}", i, j);
	}

	// enumerate on other things
	let lines = "hello\nworld".lines();

	for (linenumber, line) in lines.enumerate() {
		println!("{}: {}", linenumber, line);
	}

	// Using breaks
	let mut m = 5;

	loop {
		m += m - 3;
		if m % 5 == 0 {
			break;
		}
	}
}
