fn main(){
    let  n = 2;
    //if
    if n > 0 {
        println!("> than 0");
    }
    else if n < 0 {
        println!("< than 0");
    }
    else {
        println!("is 0");
    }

    //for
    for i in 0..5{
        println!("{}", i);
    }

    //while
    let mut k = 0;
    while k < 4 {
        println!("{}", k);
        k += 1; 
        if k == 3 {
        println!("exit");
        break
        }
    }

    //switch/match
    k = 6;
    match k {
        0 => println!("0"),
        1 | 2 => println!("1 or 2"),
        4..=7 => println!("4-7"),
        _ => println!("default")

    }

}