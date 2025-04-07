//6 Napisz funkcje, ktora odpowiada na pytanie, czy jej argument jes liczba doskonala

fn perfect_number(number: i32) -> bool {
    let mut sum = 0;

    for i in 1..=number/2 {
        if number % i == 0 {
            sum+=i;
        }
    }
    sum == number
}


fn main (){
    let number = 6;

    if perfect_number(number){
        println!("{} jest l. doskonala", number);
    } else {
        println!("{} nie jest l. doskonala", number);
    }
}
