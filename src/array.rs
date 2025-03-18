use std::mem;
pub fn run ()
{
    let mut arr : [i32;5]= [1,2,3,4,5];
    println!("{:?}", arr);
    for i in 0..arr.len()
    {
        println!("{}", arr[i]);
    }
    //Array length
    println!("{:?}", arr.len());
    //reassign value :
    arr[2]=23;
    // getting the mem size of array :
    println!("The size of array is bytes is  {}" , mem::size_of_val(&arr));
    //slicing
    let slice : &[i32] = &arr[1..3];
    println!("{:?}",slice);
    arr.reverse();
    println!("{:?}",arr);
    arr.sort();
    println!("{:?}",arr);
}