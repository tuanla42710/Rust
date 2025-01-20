/*
use std::{collections::HashMap, future::pending};


fn main(){
    println!("hello, world");
    let mut fruit_prices = HashMap::new();
    fruit_prices.insert("apple", 1.2);
    fruit_prices.insert("banana", 0.5);
    fruit_prices.insert("cherry", 2.5);

    println!("price of apple ${}", fruit_prices["apple"]);
    
    // safe way to access values : using the get index method
    match fruit_prices.get("apple") {
        Some(price) => println!("price of apple is exist : ${}", price),
        _ => println!("price of apple does not exist")
        
    }
    for (fruit, price) in &fruit_prices {
        println!("price of {} is ${}", fruit, price);
    }

    // iterate over key
    for price in fruit_prices.values() {
        println!("price {}", price);
    }

    for fruit in fruit_prices.keys(){
        println!("fruit {}", fruit);
    }
}   
 */