fn main() {
	print_a_number(28);
	print_a_number(sum(14,15));
}

fn print_a_number(num: u64) {
	println!("{}", num);
}

fn sum(num1: u64, num2: u64) -> u64 {
	return num1+num2;
}
