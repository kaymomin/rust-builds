fn main(){
    //integers
    //signed (i8, i16, i32, i64, i128)
    let _signed: i8 = -2;
    //unsigned (u8, u16, u32, u64, u128)
    let _unsigned: u8 = 6;

    //char
    let _string1 = "hey";
    
    //bool
    let is_true = true;
    let check = is_true && false || false && false;

    //float
    let float = 44.67;

    println!("first var is {}, second is {}, third one is a {}, forth is just a {}, and last one's a {}", _signed, _unsigned, _string1, check, float);

    //array 
    let arr: [u8; 4] = [1,3,4,45];
    let arrtest: [u8; 6] = [26; 6];

    println!("index: {}, length: {}", arr[0], arr.len());
    println!("second array content: {:?}, length: {}", arrtest, arrtest.len());

    //tuple
    let tup: (u8, f32, bool) = (26, 19.98, true);
    let tup2 = ("me", 2);
    println!("first: {}, second: {}, third: {}", tup.0, tup.1, tup.2);
    println!("{:?}", tup2);

    //represent each var in a tuple as a, b, or c
    let (a, b, c) = tup;
    println!("first: {}, second: {}, third: {}", a, b, c);
}