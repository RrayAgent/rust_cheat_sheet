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
    //please remember this because it is annoying to clone otherwise
    pub fn clone_from(&self) -> Self{
        Learner::new(self.name.to_string(), self.age, self.p_code.to_string())
    }
}

pub fn is_adult(age:&Learner)->bool{
    if age.age >= 18{
        true
    }else{
        false
    }
}

impl Default for Learner{
    fn default() -> Self {
        Learner::new(String::from("Omaha"), 17, "C++".to_string())
    }
}
