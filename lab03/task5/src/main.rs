/*5
Stwórz funkcję
rand_perm(arr: ..., seed: ...)
permutującą w miejscu w sposób losowy wartości tablicy przekazanej w argumencie.
Uwaga: Ta funkcja ma korzystać z dwóch poprzednich. 
*/

fn  rand(seed: &mut usize, min_rand: usize, max_rand: usize) -> usize{
    *seed = (*seed * 134775813 + 1 ) % 4294967296;
    min_rand +  *seed % (max_rand - min_rand + 1)
}

fn swap_in_array(array: &mut Vec<usize>, i: usize, j: usize){
    let tmp = array[i];
    array[i]=array[j];
    array[j]=tmp;
}

fn rand_perm(array: &mut Vec<usize>, seed: &mut usize){
    let size = array.len();

    for i in (1..size).rev(){
        let j = rand(seed,0,i);
        swap_in_array( array,i,j);
    }
}


fn main (){
    let mut array: Vec<usize> = vec![3,4,7,2,53,123,75];
    let mut seed = 12345;

    println!("Przed permutacja: {:?}", array);
    rand_perm(&mut array,&mut seed);
    println!("Po permutacji: {:?}", array);
}
