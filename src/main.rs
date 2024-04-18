use rand::Rng;

fn main() {
    let numbers = sample_number();
    
    let mut count = 0;
    loop {
        count += 1;
        println!("{}번째 시도", count);

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("잘못된 입력입니다.");
        let guess: Vec<u8> = input.trim().chars().map(|c| c.to_digit(10).unwrap() as u8).collect();

        let mut strikes = 0;
        let mut balls = 0;
        for i in 0..3 {
            if numbers[i] == guess[i] {
                strikes += 1;
            } else if numbers.contains(&guess[i]) {
                balls += 1;
            }
        }

        if strikes == 3 {
            println!("정답입니다!");
            break;
        } else {
            println!("{}S {}B", strikes, balls)
        }

        println!("=====================");
    }
}


fn sample_number() -> Vec<u8>{
    let mut numbers: Vec<u8> = Vec::new();
    let mut rng = rand::thread_rng();
    while numbers.len() < 3 {
        let n = rng.gen_range(0..10);
        if numbers.contains(&n) {
            continue;
        } else {
            numbers.push(n);
        }
    };
    numbers
}