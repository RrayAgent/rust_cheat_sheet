
use std::{io::{self, Write}, ops::Add};
//use is how to import scripts not local to projects

#[allow(dead_code)]//macros are used to extent or restrict code for more power/control most are for testing perposes
//this macro forces the compiler to not warn that code isn't used
pub fn print(){
    println!("print with automatic return");

    print!("print with out automatic return")
}

#[allow(dead_code)]//have to be re-applied to all fuctions so the compiler knows what it applies to
pub fn noticing_print_diff(){
    let str = "print with automatic return";
    let str2 = "print with out automatic return";
    //variables that act like constants in it's scope 
    //(area in code that when exited all memory is cleared demarkaed by {})

    let mut buf = String::new();
    let mut buf2 = String::new();
    //creates mutable (or variable) value storage, this one of a heap string, a string that can have variable size
    println!("{}",str);
    //printing vars
    io::stdout().flush().unwrap();
    //clears the line for use in in things like user input
    io::stdin().read_line(&mut buf).unwrap();
    //gets user input
    print!("{str2}");
    //printing vars
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf2).unwrap();
}

#[allow(dead_code)]
pub fn vars(){
    let first = 1;
    let sec:i32;
    let (_u, mut _t, _h)=(12,"r", 12.6);
    _t="y";
    sec=6;
    //referencing is done through '&' (prevents re-assinging a variable (ownership safty) mostly when being moved between functions)
    let r = &sec;
    //varieables may be explicetly or implicatly typed, 
    //if you do the latter keep in mind that first value you assign it will give it a constant type
    
    println!("{first},{sec}");

    let i = sec;
    println!("{r}, {i}");
    //proper intro to scopes with shadowing
    {
        //all vars created in this section are trashed when section ends
        //if you want to have good garbage collection in your code this probably the least annoying way
        let first = 4;
        //first's value is re-assigned for this section only
        println!("shadowed : {first}");
    }
    
    println!("original : {first}");

    let first = 8;

    println!("reassigned in main scope : {first}");

    //but the best way to change a var without increasing the amount of living vars 
    //(as long as it is the same type) is to make it mutable
    //like if let mut g:&str cannot have an i32 assigned to it
    let mut third = 7;
    println!("ori: {}", third);
    third = 529;
    //you can do any number of opperations with [var _= val] to change the vars value directly
    third *= 2;
    println!("changed: {}", third);
    // you can also parse in a plethora of ways
    {
        let mut k = third as f32;
        k/=0.1;
        println!("{k}")
    }
    {
        let k = third.to_string();
        
        println!("{:#?}",k);

        let y = k.parse::<i32>().unwrap();
        //String to ::<type>()
        println!("{:#?}",y);
        //:#? = raw var
    }
    //arrays have fixed lenght so it is on the stack
    {
        //must be initialized imeadiatly
        let k:[i32;6]=[1,2,3,4,5,6];
        println!("{:#?}",k);
    }
    //most of these are stack vars now lets see some heap vars

    let h_one = Box::new(7000000);
    let r = h_one.add(7);
    println!("{r}");

    //the better version of arrays are vectors
    let mut vec_one:Vec<i32> = Vec::new();
    vec_one.push(17);
    vec_one.push(260);
    println!("{:#?}", vec_one)
}

#[allow(dead_code)]
pub fn loops_funcs_and_ownership(){
    //this will also introduse conditions because they are kinda self explanitory

    //rust has 3 types of loops:
    //  loop = infinate loop until a condition is met
    //  for = looks like a python for
    //  while = just like every other while loop

    //we'll start with one specific to rust loop
    let mut b_loop:u32 = 0;
    loop{
        if b_loop>=70{
            break;
        }
        b_loop+=7;
    }
    println!("{b_loop}");

    
    {    
        //you can also name loops to stop specific ones if nested
        let mut o_loop:u32=0;

        'outer: loop{
            b_loop = 0;
            //inner loop will be started multiple times
            'inner: loop{

                if b_loop >= 12{
                    println!("inner loop breaking");
                    break 'inner
                }

                b_loop+=6
            }

            if o_loop>=9 && b_loop>=12{
                println!("outer loop breaking");
                break 'outer
            }
            
            o_loop+=3
        }
    }
    //for
    {    
        //there are two main ways iteration: over a range and over vec/list/slice/array values
        let mut r = Vec::new();
        //over a range
        for i in 0..=10{
            r.insert(i, i)
        }
        println!("{:#?}", r);
        //over a vec 
        //(into_iter will change i to usize instead of reference usize because of ownership *which we'll get into later*)
        for i in r.iter(){
            println!("{i}")
        }
    }
    //while
    {
        let mut i:u32 = 0;
        while i<12{
            print!("{i}");
            if i%2==0 && i!=0{
                break
            }
            i+=1;
        }
    }
}