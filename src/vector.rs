/*
Macro is a powerful tool in Rust , this allow user write code that generates other code 
at compile time

Array has some constrains:
- Fixed size array is stored in stack, so it can not increase size during runtime 
=> Vector is stored in heap that can solve this problem

Vector is a growable list type, this can easily increase or shrink size at run time.
(adding or removing element)
Vector also supports random access, push , pop, and okter list-like operation
std::vec::Vec<T> T can be any type

*/

/*
    Initialize a vector :
    - let mut v1 : Vec<i32> = Vec::new(); (use push to add value to vector)
    - let mut v2 = vec![1,2,3,4,5] ()
    - let array = [1,2,3]
        + let vect = array.to_vec();
    - let vec = Vector::from(array)
    - let vec3 = Vector::from([10;5]); (same as initialize array and conver arry to vector)
 */

/*
   Data of vector is stored in both stack and heap
   - Metadata include capacity, size, pointer to first value of vector
   - Data stored in heap 
   When create empty vector , it isn't allocated memory, but when user push value ,
   heap will reallocate memory for this , in genarall, new memory is larger than 1 , it can be 4 
   => waste memory
   => best practice: estimate capacity and use with_capacity to initialize capacity for vector

    let expected_capacity = 100;
    let mut v = Vec::with_capacity(expected_element);
    for i in 0..expected_capacity {
        v.push(i);
    }
   
*/

/*
    Vector in rust are move-by-default and do not implement the Copy trait
    This is because vector own heap-allocated memory and copying them requires a deep copy of the 
    underlying data (Clone trait)
        let v = vec![1,2,3,4,5];
        let v1 = v;
        println!("{}",v1) => error
    If user access to index larger than size => this causes panic at runtime
    => A safe way to access vector by indexing is using
        - get (return Option<&T> )
        - get_mut (return Option<&mut T>)
    let mut vec = vec![1,2,3,3,5];  
    let val_ref = vec.get(1);
    if let Some(val) = val_ref {
        println!("value {}", val);
    }
    let val_mut_ref = vec.get_mut(2);
    if let Some(val) = val_mut_ref {
        *val *= 10;
    }
    println!("{:?}", vec);
    let v2 = &mut vec[..3];
    let value = v2.get_mut(1);
    if let Some(val) = value {
        *val += 10; 
    }
    println!("check v2 {:?}", v2);
    println!(" check vector {:?}", vec);
*/

/* 
    slicing vector :
    let vec = vec![1,2,3,4,5];

    let slice = &vec[1..4];

*/

/*
    drain : 
    - remove a range of element from a vector withour deallocating the entire vector
    - move a subset of element from one vector to another
    - removing elements one at a time from a vector
    - removing specific element such as all element greater than 5
    let mut v = vec![1,2,3,4,5];
    let _ = v.drain(0..2);
    for i in v {
        println!("{}",i);
    }
*/

/*
    function retain allow filter value meet condition
    let mut nums = vec![1,2,3,4,5];
    nums.retain(|x| *x > 0);
    nums.retain(retain_positive);
    nums.retain_mut(|x| if *x > 0 {
                                     *x += 10;
                                     true
                                    } else {false});
    println!("{:?}", nums);

*/

/*
    slice :
    split_at : allow split vector into 2 sub vector (in fact , it just returns pointers of 2 first element)
    splice allow substitute some element of vector by others
    let mut list_name = vec![String::from("julia"), String::from("john"), String::from("darwin")];
    list_name.retain_mut(|x| if x.len() <= 6 {
                                            let add_str = format!(" : {}", x.len());
                                            x.push_str(&add_str);
                                            true
                                            } else {false});
    println!("{:?}", list_name); 
    let mut primary_reading = vec![22.5,23.0,0.0,0.0,23.2,24.0];
    let backup = vec![22.7,22.6,23.0];
    let faulty_reading : Vec<_> = primary_reading.splice(2..4, backup).collect();
    println!("{:?}", primary_reading);
    println!("{:?}",faulty_reading);
    let mut v1 = vec![1,2,3,4,5];
    let mut v2 = vec![6,7,8];
    v1.append(&mut v2);
    println!("{:?}", v1); // co the dung extent 
 */

 use std::collections::VecDeque;



 fn retain_positive(x : &i32)->bool {
     *x > 0
 }
 
 /* 
 fn main(){
     let mut vd = VecDeque::with_capacity(6);
     vd.push_back(2);
     vd.push_back(12);
     vd.push_front(1);
 
     let (front, back ) = vd.as_slices();
 
     println!("vd : {:?}", vd);
     println!("back {:?}", front);
     println!("front {:?}", back);
 }
*/