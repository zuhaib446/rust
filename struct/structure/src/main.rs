
// #[derive(Debug)]
// struct Students {
//     name : String,
//     age : i8
// }

// fn main() {
//      let std1 = Students{
//        name : "std 1".to_string(),
//        age  : 21,
//        };
    
//        let std2 = Students{
//         name : "std 2".to_string(),
//         age  : 22,
//         };
     
//        std_struct_fn(std1 , std2);
    
// }


// fn std_struct_fn (user1:Students , user2:Students) {
//   println!("{:#?} {:#?}"  , user1 , user2);
// }
 #[derive(Debug)]
struct Team {
country : String ,
score : u16,
}

impl Team {
    fn high(self , other:Team) -> Team{
    //team1.score> team2.score
      if self.score > other.score{
          self
      }else {
        other
      }
    }

    fn print(&self){
        println!("{}",&self.score);
      
    }
}

fn main(){
    let team1 = Team {
        country :  "pakistan".to_string(),
        score:435,
    };
    let team2 =  Team {
        country :  "sir lanka".to_string(),
        score:271,
    };
   team1.print();
   team2.print();
   let today_high = team1.high(team2);

   println!("in main : {:#?}" , today_high);
}
