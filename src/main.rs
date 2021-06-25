use std::time::Instant;

// TODO: abbruchbedingung testen, stack optimieren (max größe, ältestes löschen)
// TODO: brute force
use cargo_bot_simulator::CbInterpret;
fn main() {
    let mut cb = CbInterpret::new("qdq>qdq1", "y,n,n,n,n", "n,n,n,n,y").unwrap();

    println!("{:?}", cb);
    // for i in 0..10 {
    //     cb.step();
    //     println!("{:?}", cb);
    //     for d in cb.data.iter() {
    //         //println!("{:?}", d);
    //     }
    // }


    let now = Instant::now();

    let mut steps = 0;
    while cb.step() {
        // println!("{:?}", cb);
        steps += 1;
        // println!("{}", cb.print_crane());
        // println!("{}", cb.print_data());
    }

    let took = now.elapsed().as_micros(); 

    println!("{:?}", cb);

    println!("{}", cb.print_crane());
    println!("{}", cb.print_data());

    println!("simulating {} steps took {}µs", steps, took);
}
