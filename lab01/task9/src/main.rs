//9 Napisz program, ktory znajduje wszystkie trojki pitagorejskie o wartosciach nie wiekszych niz
//  dana, zakladamy ze 0<a<b<c



fn main (){
    let value = 25;

    println!("Trojki pitagorejskie o wartosciach mniejszych lub rownych {}:", value);

    for a in 1..value{
        for b in (a+1)..value{
            let c_kw=a*a+b*b;
            let c = (c_kw as f64).sqrt() as u32;

            if c*c == c_kw && c <= value {
                println!("({}, {}, {})", a, b, c);
            }
        }
    }

}
