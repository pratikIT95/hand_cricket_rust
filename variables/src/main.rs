fn main() {
    let mut x = 5;
    println!("The value of x is {}",x);
    x=6;
    println!("The value of x is {}",x);

    let tup = (1.0,23,'c');
    println!("x.0 = {}, x.1 = {}, x.2 ={}",tup.0,tup.1,tup.2);

    let arr : [i32;5] = [1,2,3,4,5];

    let mut index = 0;

    while index < arr.len(){
        println!("arr[{}] = {}",index,arr[index]);
        index += 1;
    }

    for element in arr.iter() {
        println!("{}",element);
    }
    cricket_game();
}

fn cricket_game(){
    println!("Cricket game!");
}
