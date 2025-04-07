/*4 Napisz funkcję swap_arr(arr: ..., i: usize, j: usize) która zamieni wartości dwóch podanych elementów pewnej tablicy. 
*/


fn swap_in_array(array: &mut Vec<i32>, i: usize, j: usize){
    let tmp = array[i];
    array[i] = array[j];
    array[j] = tmp;
}

fn main (){
    let mut array: Vec<i32> =vec![3,5,1,7,4];

    println!("{:?}", array); 
        swap_in_array(&mut array, 3,1);
    println!("po zmianie: {:?}", array);
 }

