// similar to switch case but easier
fn main() {
    let val = 5;
    match val {
        1 => println!("one"),
        10 => println!("ten"),
        100 => println!("one hundred"),
        _ =>
	        {
            println!("something else");
        }
    }
	let flag = true;
	let val = match flag {
		true => true,
		false => false,
	};
	println!("The value of {flag} is {val}");
}