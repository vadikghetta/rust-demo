
// fn - function


fn main() {
    println!("hello");
    let a = 32;
   let res = demo(a);
    println!("hello {res}");
    let mut counter = 0;
    // infinite loop
    // loop {
        //     if counter == 10 {
            //         break;
            //     }
            //     counter += 1;
            //     println!("{counter}");
            // }
            let arr = [10, 20, 30 , 40];
            while counter == 10 {
                counter += 1;
            }
            
            println!("{counter}");
    for el  in arr {
        println!("{el}")
    }
}
// statement что то делают но ничего не возвращают
// expression обрабатываем какое то значение и возвращаем его
fn demo (a : i32) -> bool {
    println!("hello demo {a}");
    if a > 10 {
        true
    }else { 
        false
    }
    // a * 2
}