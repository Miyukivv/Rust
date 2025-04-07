//1 Zadania numer 6 oraz 9 z Zestawu 1 zrob na dwa sposoby - kazde z uzyciem petli while/loop oraz
//  for. 

//6 Napisz program, ktory oblicza silnie dla danej liczby

fn factorial_while(number: i32) ->i32 {
    let mut n=number;
    let mut result = 1;
    while n>1{
        result*= n;
        n-=1;
    } 

    result
}

fn factorial_loop(number: i32) -> i32{
    let mut n=number;
    let mut result=1;
    loop {
        if n<=1{
            return result;
        }
        result*=n;
        n-=1;
    }
}

fn factorial_for(number: i32) -> i32{
    let mut result=1;
    for i in 1..=number{
        result*=i;
    }
    result
}

//9 Napisz program, ktory znajduje wszystkie trojki pitagorejskie o wartosciach nie wiekszych niz
//  dana. Zakladamy, ze 0 < a < b < c.  

fn pythagorean_triplets_while(number: u32){
    println!("Pythagorean triples with values less than or equal {}:", number);

    let mut a = 1;
    
    while a < number {
        let mut b = a+1;
        while b < number {
            let c_kw = a*a + b*b;
            let c = (c_kw as f64).sqrt() as u32;

            if c*c == c_kw && c <= number{
                println!("{}, {}, {}", a,b,c);
            }
            b+=1;
        }
        a+=1;
    }
}

fn pythagorean_triplets_for(number: u32){
    println!("Pythagorean triples with values less than or equal {}:",number);

    for a in 1..number{
        for b in (a+1)..number{
            let c_kw = a*a + b*b;

            let c = (c_kw as f64).sqrt() as u32;

            if c*c == c_kw && c <= number {
                println!("({}, {}, {})", a,b,c);
            }
        }
    }
}

fn pythagorean_triplets_loop(number: u32){
    println!("Pythagorean triples with values less than or equal {}", number);

    let mut a=1;
    loop {
        if a >= number{
            break;
        }
            let mut b = a+1;

            loop {
                if b >= number {
                    break;
                }
                let c_kw = a*a+b*b;
                let c = (c_kw as f64).sqrt() as u32;

                if c*c == c_kw && c <= number{
                    println!("({}, {}, {})",a,b,c);
                }
                b+=1;
            }
            a+=1;
    }

}

fn main (){
    let number = 5;
    println!("Factorial with while: {}", factorial_while(number));
    println!("Factorial with loop: {}", factorial_loop(number));
    println!("Facotiral with for: {}", factorial_for(number));
 
    let number_for_pythagorean_triplets = 20;
    println!("Pythagorean triples with while");
    pythagorean_triplets_while(number_for_pythagorean_triplets);
    println!("\nPythagorean triplets with for");
    pythagorean_triplets_for(number_for_pythagorean_triplets);
    println!("\nPythagorean triplets with loop");
    pythagorean_triplets_loop(number_for_pythagorean_triplets);
}
