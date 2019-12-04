fn is_armstrong(num: u32) -> bool{
    let digits: Vec<_> = num.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let num_of_digits = digits.len();
    let sum: u32 = digits.into_iter().map(|digit| digit.pow(num_of_digits as u32)).sum();
    return sum == num;
}

fn main() {
    let mut input = String::new();
    println!("Type the number you want to check:");
    std::io::stdin().read_line(&mut input).expect("[ERROR] Unable to get user input!!");
    let num = input.trim().parse::<u32>().expect("[ERROR] User input is not an integer!!");
    if is_armstrong(num){
        println!("{} is an Armstrong number!", num);
    }else{
        println!("{} is not an Armstrong number!", num);
    }
}
