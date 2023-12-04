fn main(){
  let gname = String::from("Angry Bird");
  let G1 = Game {name: gname, version: 3};
  G1.game_info();
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
