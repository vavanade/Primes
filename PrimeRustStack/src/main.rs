// The implementation is a bit wasteful since it only uses the first half of the array but there is
// no U500000. However, it is stack-allocated so it could be faster than more dynamic solutions.
//
// Based on the PrimeSievePY.


extern crate typenum;
use bit_array::BitArray;
use typenum::U1000000;
use std::convert::TryInto;
use typenum::Unsigned;
use std::time::{Duration, Instant};


type N = U1000000;


fn get_bit(bit_array: &BitArray<u32, N>, index: usize) -> bool{
    if index % 2 == 0 {
        return false;
    } else {
        return bit_array.get((index/2).try_into().unwrap()).expect("Failed to read bit.");
    }
}


fn clear_bit(bit_array: &mut BitArray<u32, N>, index: usize){
    bit_array.set((index/2).try_into().unwrap(), false);
}


fn run_sieve(bit_array: &mut BitArray<u32, N>){
    let sqrt_size = ((bit_array.len()) as f64).sqrt() as usize;
 

    let mut factor = 3;
    while factor < sqrt_size {
        for num in factor..bit_array.len() {
            if get_bit(bit_array, num) {
                factor = num;
                break;
            }
        }

        for num in (factor*3..bit_array.len()).step_by(factor*2) {
            clear_bit(bit_array, num);
        }

        factor += 2;
    }
}


fn main() {
    
    //let n = N::to_u32();

    let mut passes = 0;

    let limit = Duration::new(10, 0); // 10 s
    let start = Instant::now();
    while start.elapsed() < limit {
        let mut ba = BitArray::<u32, N>::from_elem(true);
        run_sieve(&mut ba);
        passes += 1;
    }
    let stop = Instant::now();
    let actual_duration = stop - start;

    println!("Passes: {}, Time: {}, Avg: {}, Limit: {}",
             passes,
             actual_duration.as_secs_f32(),
             actual_duration.as_secs_f32() / passes as f32,
             ba.len()
    );
    //for num in 3..n {
    //    let num = num as usize;
    //    println!("{}: {}", num, get_bit(&ba, num));
    //}

}
