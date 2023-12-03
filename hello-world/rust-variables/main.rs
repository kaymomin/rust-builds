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

    //On array, slices and the concept of ownership and borrowing
    let anotherarr = [0,1,3,6];
    let slice = &anotherarr[0 .. 3];

    borrowing_slice(anotherarr, slice);

    //string
    let str: &str = "hey you there";
    //or
    let mut styy: String = String::from("hey you there");
    let strslice = &styy[.. 5];
    println!("method 1: {}",str);
    println!("method 2: {}",styy);
    println!("method 3: {}, len: {}",strslice, strslice.len());

    styy.push('8');
    styy.push_str(" hey");
    styy = styy.replace("there", "here");
    println!("{}", styy);


}

fn borrowing_slice(anotherarr: [u8; 4], slice: &[u8]){
    println!("{:?}", anotherarr);
    println!("{:?}", slice);
    //know len before
    println!("len: {}", slice.len());
    println!("{} {}", slice[0], slice[2]);

}