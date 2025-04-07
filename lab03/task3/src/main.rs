/*
Stworz generator liczb pseudolosowych, ktorego ziarno przechowywane bedzie na zewnatrz i podawane w pierwszym parametrze, a w parametrze drugim i trzecim podawany bedzie przedzial losowanych liczb. 
fn rand(seed: &mut ..., min_rand: ...., max_rand: ...) -> ....
*/

fn rand( seed: &mut usize, min_rand: usize, max_rand: usize) -> usize{
  *seed = (*seed * 134775813 + 1 ) % 4_294_967_295;
  min_rand+(*seed % (max_rand - min_rand + 1))
}

fn main(){
    let mut seed = 11;
    let min_rand = 11;
    let max_rand = 30;

    println!("{}", rand(&mut seed, min_rand, max_rand));
    println!("{}", rand(&mut seed, min_rand, max_rand));
    println!("{}", rand(&mut seed, min_rand, max_rand));
}
