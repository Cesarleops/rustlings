fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {number}");

    // THIS IS A SHADOWING EXAMPLE, IT ALLOWS REUSING THE PREVIOUS VALUE, CHANGING THE TYPE
        let number = 3;
    println!("Number plus two is: {}", number + 2);
}
