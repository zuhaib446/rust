
#[derive(Debug)]
struct Students {
    name : String,
    age : i8
}

fn main() {
     let std1 = Students{
       name : "std 1".to_string(),
       age  : 21,
       };
    
       let std2 = Students{
        name : "std 2".to_string(),
        age  : 22,
        };
     
       std_struct_fn(std1 , std2);
    
}


fn std_struct_fn (user1:Students , user2:Students) {
  println!("{:#?} {:#?}"  , user1 , user2);
}