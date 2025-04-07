fn main() {
    println!("Hello, world!");
    println!("Author: Jacob");
    println!("Date:   04/06/2025");
    println!("Time:   8:03PM");
    println!("Objective: Update the file to see how fast it runs before and after");
    println!("time1: 1.62s");
    println!("time2: 0.05s");
    println!("time3: 0.06s");

    let compile_time: f32 = 1.62;
    let run_time: f32 = (0.05 + 0.06) / 2.0;

    let time_saved: f32 = compile_time / run_time;
    println!("{}{}", "Speed increase: ", time_saved);
    println!("\"Note this is a static program. These were the results
        of the first \'cargo run\' command that I did.");
}
