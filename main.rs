use rayon::prelude::*;

fn main() {
    let mut number = vec![1, 2, 4, 21];
    //Do calculations over elements of this list in parallel.
    let _sum = increment_all_par_fun_mut(&mut number);
        //let _sum = increment_all_par_fun_mut(&mut number);
        println!("{:?}",number);

    //Do calculations over files in the paths listed
}

/*
fn increment_all(counts: &mut [u32]) {
    for c in counts.iter_mut() {
        *c += 1;
        println!("in for loop c is {}", c);
    }
}

fn increment_all_par_fun_shared(counts: &mut [u32]) {
    counts.par_iter_mut().for_each(   |mut c|  {cals_shared(&mut c)}  ); 
        
        println!("in for loop c is {:?}", counts);
    
}

fn cals_shared(c:&mut u32)->u32{
  *c+=2;
  let mut b:u32=32;
  *c=b+(*c);
  b
}

*/


fn increment_all_par_fun_mut(counts: &mut [u32]) {
    counts.par_iter_mut().for_each(    |mut c| cals_mut(&mut c) ); 
        
        println!("in for loop c is {:?}", counts);
    
}
fn cals_mut(c:&mut u32){
  *c+=2;
  let mut b:u32=32;
  *c=b+(*c)
  
}



//fn load_images(paths: &[PathBuf]) -> Vec<Images> {
//    paths.iter().map(|path| Images::load(path)).collect()
//}
