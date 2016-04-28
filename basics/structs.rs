struct Point {
	x: u32,
	y: u32
}

fn main() {
	let origin = Point { x: 0, y: 0 };

	println!("The origin point is: {} {}", origin.x, origin.y);

	// We can make these and their properties mutable

	let mut player_position = Point { x: 5, y: 5 };

	player_position.y = 6;

	// We can't do field level mutability

	// struct Point {
	//   mut x : u32,
	//   y: u32
	// }
	// This is not allowed ;-;

	// We can revoke mutability by redeclaring the same variable as non-mutable

	let player_position = player_position;

	// This will cause an error
	// playerPosition.x = 10;

	// We can use &mut pointers as properties which I don't quite understand yet
	// Looks cook though
	// This allows some mutability
}
