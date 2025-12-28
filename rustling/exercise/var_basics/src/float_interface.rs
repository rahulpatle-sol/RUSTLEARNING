pub fn floating_type() {
    let price: f64 = 29.99;
    let discount = 0.15;
    let final_price = price * (1.0 - discount);

    // Boolean logic
    let is_affordable = final_price < 25.0;
    let has_coupon = true;
    let can_buy = is_affordable && has_coupon;

    // Check if number is even
    let number = 452;
    let is_even = number % 2 == 0;

    if is_even {
        println!("Number {} is even.", number);
    } else {
        println!("Number {} is odd.", number);
    }

    println!("Final price: ${:.2}", final_price);
    println!("Can buy: {}", can_buy);
    println!("Is {} even? {}", number, is_even);
}
