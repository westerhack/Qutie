#![allow(dead_code, unused)]

/*
Car = class(){
   __init = func(self, maker, wheels, doors){
      self.maker = maker;
      self.wheels = wheels;
      self.doors = doors;
   }
   __text = func(self){
      "I'm a " + self.maker + " with " + self.wheels + " wheels and " + self.doors + " doors."
   }
}
*/

mod parsing;
mod objects;
mod traits;

fn main() {

   let inp = "
14 15;
#1 = { 3 + 5 '}' /* foo } */ }
";
   let mut stream = parsing::stream::Stream::new(inp);
   println!("{:?}\n", parsing::parser::parse(&mut stream));
}









