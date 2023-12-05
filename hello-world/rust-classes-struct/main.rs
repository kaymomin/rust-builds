fn main(){
  let gname = String::from("Angry Bird");
  let G1 = Game {name: gname, version: 3};
  G1.game_info();
  println!("Is multiplayer: {} / Is Onchain: {}", G1.ismultiplayer(), G1.isonchain());
}

struct Game {
    name: String,
    version: u8
}

//methods in struct

impl Game {
    fn game_info (&self) {
       println!("{}", self.name);
    }
  }

//to extend a trait
impl ingame for Game {
  fn ismultiplayer(&self) -> bool{
    false
  }

}

//interfaces
//can extend or override a trait
trait ingame {
 fn ismultiplayer(&self) -> bool;
 fn isonchain(&self) -> bool {
  true
 }

}


