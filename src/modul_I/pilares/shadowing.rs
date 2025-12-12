pub fn variables_shadowing() {
    let x = 5;
    let mut y = 10;

    let x = 15;
    let mut y = 20;

    println!("x = {}", x);
    println!("y = {}", y);
}
