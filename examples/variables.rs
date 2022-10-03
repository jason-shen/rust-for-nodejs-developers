fn main() {
	// explicit
	let foo:&str = "foo";

	// type inferred
	let bar = "foo";

	// constant
	const QUX:&str = "Qux";

    println!("{} {} {}", foo, bar, QUX);
}
