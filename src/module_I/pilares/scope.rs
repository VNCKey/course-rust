pub fn scope() {
    let x = 10;
    {
        let y = 20;
        println!("x = {}, y = {}", x, y);
    }
    println!("x = {}", x);
}
