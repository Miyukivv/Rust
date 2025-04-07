/*1 Napisz dwuargumentowa funkcje, ktora zamieni wartosci podanych w argumentach zmiennych (dla
 *   ustalenia uwagi typu i32)
 */

fn swap( a: &mut i32,b: &mut i32){
    let mut tmp=*a;
    *a=*b;
    *b=tmp;
}


fn main (){
    let mut a = 4;
    let mut b = 7;

    println!("a: {}, b: {}", a,b);
    swap(&mut a,&mut b);
    println!("a: {}, b: {}",a,b);
}
