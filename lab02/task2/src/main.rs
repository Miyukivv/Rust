/*2 Przy zalozeniu, ze mamy zdefiniowane funkcje:
 * fn f(x: f64) -> f64
 * fn fp(x: f64) -> f64
 * (spelniajace odpowiednie zalozenia; druga jest pochodna pierwszej) napisz funkcje
 *
 * fn met_newt(x0: f64, eps: f64, n: u128) -> f64
 *
 * realizujaca znajdowanie przyblizonego miejsca zerowego metoda Newtona - w czterech wersjach...*/
fn f(x: f64) -> f64{
    x*x-3.0*x+2.0
}

fn fp(x: f64) -> f64{
    2.0*x-3.0
}


//1) Iteracyjna z petla loop (z ewentualnymi break continue return)
fn met_newt_loop(x0: f64, eps: f64, n: u128) -> f64{
    let mut x=x0;
    let mut iter=0;

    loop {
        let fx=f(x);
        let fpx=fp(x);

        if fpx == 0.0 {
            break;
        }
        let next_x=x-fx/fpx;

        if (next_x-x).abs() < eps || iter >= n{
            return next_x;
        }
        x = next_x;
        iter +=1;
    }
    x
}

//2) Iteracyjna z petla while (bez zadnych break, continue, return)
fn met_newt_while(x0: f64, eps: f64, n: u128) -> f64{
    let mut x=x0;
    let mut iter=0;
    let mut next_x=x;
    let mut continue_iterating = true;

    while iter < n && continue_iterating==true{
        let fx=f(x);
        let fpx=fp(x);

        if fpx == 0.0 {
            next_x=x;
        } else {
            next_x=x-fx/fpx;
        }

        if (next_x-x).abs() < eps{
            continue_iterating = false
        }
    
        x=next_x;
        iter+=1;
    }
    next_x
}
//3) Rekurencyjna
fn met_newt_rec(x0: f64, eps: f64, n: u128) -> f64{
    let x=x0;
    if n == 0 {
        return x;
    }
    let fx=f(x);
    let fpx=fp(x);

    if fpx == 0.0{
       return  x;
    }
    let next_x=x-fx/fpx;

    if (next_x-x).abs()<eps{
        return next_x;
    }
    met_newt_rec(next_x,eps,n-1)
}


//4) Iteracyjna z petla for (z ewentualnymi break continue return)
fn met_newt_for(x0: f64, eps: f64, n: u128) -> f64{

    let mut  x=x0;

    for i in 1..n{
        let fx=f(x);
        let fpx=fp(x);

        if fpx == 0.0 {
            return x;
        }
        let next_x=x-fx/fpx;

        if (next_x-x).abs() < eps {
            return next_x;
        }
        x=next_x;
    }
    x
}

fn main (){
    let  x0=4.0;
    let eps = 1.0;
    let n = 4;

    println!("Loop: {}\n", met_newt_loop(x0,eps,n));
    println!("While: {}\n", met_newt_while(x0,eps,n));
    println!("Rec: {}\n", met_newt_rec(x0,eps,n));
    println!("For: {}\n",met_newt_rec(x0,eps,n));
}
