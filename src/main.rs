/* Challenge -> Given a list of integers, use a vector and return the mean (average),
median (when sorted, the value in the middle position), and mode (the value that
occurs most often; a hash map will be helpful here) of the list. */
use core::f32;
use rand::prelude::*;
use std::collections::HashMap;

//Average function ->
fn average(nums: &[i32]) -> f32 {
    /* Now with a for loop iterate the vector count the total numbers, sum them all
    and divide them to get the average */
    let mut counter = 0;
    for x in nums {
        counter = counter + x;
    }
    counter as f32 / nums.len() as f32
}
//Let's make the function to calculate the median value!
fn median(nums: &mut [i32]) -> i32 {
    //I'll take a reference of the vector and order it and get the mean value
    nums.sort();
    //Let's print the vector in order, just to check.
    println!("The vector in order is {:?}", nums);
    //Now let's calculate de median.
    let median = nums.len() / 2;
    //And return it.
    nums[median]
}
//Now the mode value calculate function.
fn mode(nums: &[i32]) -> i32 {
    //First a hasmap, to add a key to each value.
    let mut occurrences = HashMap::new();
    /* A for loop whith the entry and or_insert method to add a key to each value if
    it doesn't have one or add 1 if the value has a key. */
    for &value in nums {
        *occurrences.entry(value).or_insert(0) += 1;
    }
    //Just to see how it works this hashmap and the loop let's print ocurrences.
    println!("Ocurrences HashMap looks like {:?}", occurrences);
    occurrences
        //First let's iterate the hashmap
        .into_iter()
        //Then obtain the maximun value according to their keys (amazed with that)
        .max_by_key(|&(_, count)| count)
        //Then with map, i let the count out. Bye count!
        .map(|(val, _)| val)
        //And, in case of an error, I'll let the user know it.
        .expect("Imposible to calculate the mode of zero numbers")
}
fn main() {
    //I call the method to bring to the scope a random number generator
    let mut rng = thread_rng();
    //I made a rand distribution from the range from 1 to 200, including them.
    let distr = rand::distributions::Uniform::new_inclusive(1, 200);
    //The I create a Vector of thirdteen zeros
    let mut nums = vec![0i32; 13];
    /*Here with a for loop I fill de vector with, modifying in every loop one of the
    zeros in the vector with a sample of the distribution range that I created*/
    for x in &mut nums {
        *x = rng.sample(distr);
    }
    //I made an especific vector, not a random one, to be able to calculate the mode.
    let mode_num = vec![1, 34, 56, 90, 54, 60, 34, 3, 2, 12, 67, 34, 42];
    //Let's print out the vector values
    println!("First let's see our random vector {:?}", nums);

    //Let's print the average value!
    println!("The average is {}", average(&nums));

    //Let's print the mean value!
    println!("The mean value is {:?}", median(&mut nums));

    //Let's print the mode value!
    println!("The mode value is {}", mode(&mode_num));
}
