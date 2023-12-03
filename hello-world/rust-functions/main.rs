fn main(){

    println!("{}", iseven(5));

}
//all rust funcs are private by default unless states "pub"
pub fn iseven(num: u8) -> bool {
    let a: u8 = num % 2;
    a == 0 // return bool (no semicolon for return statement)
    
}

