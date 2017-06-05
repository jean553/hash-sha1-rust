use std::io;

fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("cannot read input");

    // remove the last characters from the input
    let size = input.len();
    input.remove(size - 1);

    let mut message = String::new();
    for character in input.chars() {
        let val: u8 = character as u8;
        let binary = format!("{:b}", val);
        let fill = "0".repeat(8 - binary.len());

        message = format!("{}{}{}", message, fill, binary);
    }

    message = format!("{}1", message);

    while message.len() % 512 != 448
    {
        message = format!("{}0", message);
    }

    println!("{}", message);
}
