use std::time::Instant;

// TODO: stack optimieren (max größe, ältestes löschen)
// TODO: brute force
use cargo_bot_simulator::{CbInterpret, FinishState, StepState};
fn main() {
    // let mut cb = CbInterpret::<5>::new("q.a>q1", "yy,n,n,n,n", "y,n,n,n,y").unwrap();
    // let mut cb = CbInterpret::<4>::new("q.a>q1", "n,rrr,bbb,ggg,n", "rrr,bbb,ggg,n").unwrap();
    let mut cb = CbInterpret::<4>::new("q.a>q1", "n,rrb,n,rbb", "b,rr,bb,r").unwrap();
    //let mut cb = CbInterpret::<5>::new("qdq>q<qdq1", "y,n,n,n,n", "n,n,n,n,y").unwrap();

    println!("{:0b}", cb.data[0]);
    println!("{}", cb.print_crane());
    println!("{}", cb.print_data());
    // for i in 0..10 {
    //     cb.step();
    //     println!("{:?}", cb);
    //     for d in cb.data.iter() {
    //         //println!("{:?}", d);
    //     }
    // }

    let now = Instant::now();

    // let mut steps = 0;
    // while cb.step() == StepState::Normal {
    //     println!("{:?}", cb);
    //     steps += 1;
    //     println!("{}", cb.print_crane());
    //     println!("{}", cb.print_data());
    //     if steps == 10{
    //         return;
    //     }
    // }

    // let steps = match cb.run_all() {
    //     FinishState::Crashed(i) => {println!("Crashed");i},
    //     FinishState::Finished(i) => {println!("Finished");i},
    //     FinishState::Limited(i) => {println!("Limited");i},
    // };

    let steps = 0;
    cb.brute_force();

    println!("{:?}", cb);

    let took = now.elapsed().as_nanos();

    // println!("{:?}", cb);

    println!("{}", cb.print_crane());
    println!("{}", cb.print_data());

    println!("{}", cb.print_inst());

    println!(
        "simulating {} steps took {}ns, that's {:.2}ns per step",
        steps,
        took,
        took as f64 / steps as f64
    );
}
