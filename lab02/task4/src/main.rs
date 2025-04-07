//4 Napisz funkcje, ktora dla danego calkowitego dodatniego n zwraca numer iteracji, w ktorej
//  osiagamy jedynke w problemie Collatza (np. dla n = 12 wynikiem jest 9)


fn collatz(mut n: u32) -> u32  {
    let mut i = 0;
    while n != 1 {
        if n % 2 == 0{
            n=n/2;
        } else {
            n=3*n+1;
         }
        i+=1;
    }
    i
}

fn main (){
    let n = 12;

    println!("Number of iteration {} is: {}",n, collatz(n));
}
