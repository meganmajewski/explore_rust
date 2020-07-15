pub fn if_statement(){
    let temp = 25;

    if temp > 30 {
        println!("greater than 30");
    } else if temp < 10 {
        println!("less than 10")
    } else {
        println!("else")
    }

    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("today is {}", day);
}
pub fn while_and_loop() {
    let mut x = 1;

    while x < 1000 { 
        x *= 2;
        if x == 64 { //if x = 64 skip the rest of this function
            continue;
        }
        println!("x = {}", x);
    }

    //loop is basically while true
    let mut y = 1;
    loop { 
    
        y *= 2;
        println!("y = {}", y);

        if y == 1<<10 { // 2 ^ 10;
            break;
        }
    }
}
pub fn for_loop() {
    //init x as a range of values from 1 to 10 inclusive
    for x in 1..11 {
        println!("x = {}",x);
    }
    
    for (position, y) in (20..51).enumerate() {
        println!("{}: {}",position, y);
    }
}

pub fn match_statement(){
    let country_code = 444;
    let country = match country_code { // if the country code matches something below return that value
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=999 => "unknown", //inclusive
        _ => "bad code" // catch all 
    };

    println!("the country is {}", country);
}