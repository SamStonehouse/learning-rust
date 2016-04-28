use std::cell::Cell;

fn main() {
	let mut x = 5;
	let y = &mut x;
	println!("{}", y);

	// Field level mutability

	struct Point {
		x: u32,
		y: u32
	}

	let mut a = Point { x: 10, y: 10 };

	a.y = 15;

	let b = Point { x: 10, y: 10 };

	println!("{}", b.x);

	// b isn't mutable so we can't change it's properties
	// b.y = 15

	// Can emulate field level immutability with Cell<T>

	struct Poont {
		x: u32,
		y: Cell<u32>
	}

	let c = Poont { x: 11, y: Cell::new(13) };

	c.y.set(15);

	println!("{} {:?}", c.x, c.y);
}
