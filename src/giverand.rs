use rand::{thread_rng, Rng};
pub fn apple()->i32
{
    let mut rng = thread_rng();
    let n: u32 = rng.gen_range(0..10);
    let n:i32 = n as i32; //parsing n in i32 as guess is also in i32 formate
    n


}

