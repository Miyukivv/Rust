fn swap(a: &mut i32, b: &mut i32){
    let mut tmp=*a;
    *a=*b;
    *b=tmp;
}

fn swap_abc(a: &mut i32, b: &mut i32, c: &mut i32){
    if a > b {
        swap(a,b);
    } 
    if a > c {
        swap (a,c);
    }

    if b > c {
        swap(b,c);
    }
}

fn main (){
    let mut a = 74;
    let mut b = 23;
    let mut c = 14;

    println!("a: {}, b: {}, c: {}",  a,  b, c);
    swap_abc(&mut a, &mut b, &mut c);
    println!("a: {}, b: {}, c: {}", &mut a, &mut b, &mut c);
}
