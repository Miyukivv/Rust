//7 Napicz funkcje, ktora wyswietli rozklad podanej liczby na czynniki pierwsze

fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();

    //Sprawdzamy podzielnosc przez 2
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    let mut i = 3;

    while i*i <= n {
        factors.push(i);
        n /= i;
    }
    i += 2;

    if n > 2 {
        factors.push(n);
    }

    factors
}


fn main(){
    let number = 56;
    let factors = prime_factors(number);
    println!("Rozklad na czynniki pierwsze liczby {}: {:?}", number, factors);
}
