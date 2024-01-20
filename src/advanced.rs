#[derive(Debug)]
pub struct Learner{
    pub name:String,
    age:u32,
    pub p_code:String
}
//you must define a structs vars with a hard type
//use the impl key word before the name of your defined struct to create or assine functions to the struct
impl Learner{
    pub fn new(n:String, a:u32,pc:String)->Self{
        Self{
            name:n,
            age:a,
            p_code:pc
        }
    }
    /*if you use an instance of a struct in an implamented func if it isn't a reference
     ownership will be transfered to the function*/
    pub fn structs_classes(&self){
        let n = &self.name;
        println!("hello {n}")
    }
    
}

pub fn is_adult(age:&Learner)->bool{
    if age.age >= 18{
        true
    }else{
        false
    }
}

//how to implement the trait
impl Default for Learner{
    fn default() -> Self {
        Learner::new(String::from("Omaha"), 17, "C++".to_string())
    }
}

//how to define a trait, which can be impl in to any struct
pub trait Cop {
    //please remember this because it is annoying to clone and make traits otherwise
    fn clone_from(learner:&Learner) -> Self;
}

impl Cop for Learner{
    fn clone_from(learner:&Learner) -> Self{
        Learner::new(learner.name.to_string(), learner.age, learner.p_code.to_string())
    }
}

//now for the final big thing to learn importing
//to import a library go to the .toml file write, "[package_name] = '[version # or * for most resent]"'
extern crate rand;
use rand::Rng;
#[allow(dead_code)]
pub fn using_libs()->Vec<i32>{
    let mut y = rand::thread_rng();
    let mut t: Vec<i32>=Vec::new();
    for _i in 0..90{
        t.push(y.gen::<i32>())
    }
    return t
}

//treading
use std::{thread, time::Duration};
#[allow(dead_code)]
pub fn running_parralle(o:i32){
    let mut hand = vec![];
    for i in 0..8{
        
        let handl = thread::spawn(move|| {
            for j in 0..(o){
                println!("{}",j/(i+1));
                thread::sleep(Duration::from_secs(1));
            };
        });
        hand.push(handl)
        
    }

    for h in hand{
        h.join().expect("dead")
    }
}