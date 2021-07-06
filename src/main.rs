use std::time::Instant;

// TODO: stack optimieren (max größe, ältestes löschen)
// TODO: brute force
use cargo_bot_simulator::{CbInterpret, FinishState};
fn main() {
    let mut cb = CbInterpret::<5>::new("qdq>qdq1", "y,n,n,n,n", "n,n,n,n,y").unwrap();
    //let mut cb = CbInterpret::<5>::new("qdq>q<qdq1", "y,n,n,n,n", "n,n,n,n,y").unwrap();

    println!("{}", cb.print_crane());
    println!("{}", cb.print_data());
    // println!("{:?}", cb);
    // for i in 0..10 {
    //     cb.step();
    //     println!("{:?}", cb);
    //     for d in cb.data.iter() {
    //         //println!("{:?}", d);
    //     }
    // }

    let now = Instant::now();

    // let mut steps = 0;
    // while cb.step() == StepState::normal {
    //     // println!("{:?}", cb);
    //     steps += 1;
    //     // println!("{}", cb.print_crane());
    //     // println!("{}", cb.print_data());
    // }

    let steps = match cb.run_all() {
        FinishState::Crashed(i) => i,
        FinishState::Finished(i) => i,
        FinishState::Limited(i) => i,
    };

    let took = now.elapsed().as_nanos();

    // println!("{:?}", cb);

    println!("{}", cb.print_crane());
    println!("{}", cb.print_data());

    println!(
        "simulating {} steps took {}ns, that's {:.2}ns per step",
        steps,
        took,
        took as f64 / steps as f64
    );
}
