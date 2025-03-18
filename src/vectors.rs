use std::mem;
pub fn run ()
{
    let mut vectors : Vec<i32>= vec![1,2,3,4,5];
    println!("{:?}", vectors);
    for i in 0..vectors.len()
    {
        println!("{}", vectors[i]);
    }
    //vector say length
    println!("{:?}", vectors.len());
    //reassign value :
    vectors[2]=23;
    // getting the mem size of vector  :
    println!("The size of vector in bytes is  {}" , mem::size_of_val(&vectors));
    //slicing
    let slice : &[i32] = &vectors[1..3];
    println!("{:?}",slice);
    vectors.reverse();
    println!("{:?}",vectors);
    vectors.sort();
    println!("{:?}",vectors);
}