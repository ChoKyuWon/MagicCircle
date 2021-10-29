use std::io;
use std::vec;

fn verify(iter:&Vec<u64>) -> bool {
    for i in 0..iter.len() - 1{
        if !is_square(iter[i] + iter[i+1]){
            return false;
        }
    }
    if !is_square(iter[0] + iter[iter.len() - 1]){
        return false;
    } 
    true
}

fn is_square(x:u64) ->bool{
    match x&0xf {
        0 | 1 | 4 | 9 => (), 
        _ => return false
    };
    let tmp = ((x as f64).sqrt() + 0.5) as u64;
    x == tmp * tmp
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("STDIN Error!");
    let input = buffer.trim_end().parse::<u64>().expect("Only integer input is allowed!");
    let mut array: Vec<Vec<Vec<u64>>> = Default::default();
    array.push(vec![vec![1]]);
    for l in 0..input - 1{
        let global_vec = &(array[l as usize]);
        let mut new_vec:Vec<Vec<u64>> = vec![];
        for local_vec in global_vec{
            let mut freelist:Vec<u64> = Default::default();
            for i in 1..input + 1{
                if !local_vec.contains(&i){
                    freelist.push(i);
                }
            }
            for i in freelist{
                let tail = local_vec.last().unwrap();
                if is_square(tail + i){
                    let mut tmp = local_vec.clone();
                    tmp.push(i);
                    new_vec.push(tmp);
                }
            }
        }
        array.push(new_vec);
    }
    let mut result : Vec<&Vec<u64>> = vec![];
    match array.last(){
        Some(q) => for vectors in q{
            if is_square(vectors.last().unwrap() + 1){
                result.push(vectors);
            }
        },
        None => println!("None"),
    }
    for res in result{
        println!("{:?}", res)
    }
}
