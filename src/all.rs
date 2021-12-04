mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let mains = [
        day1::main,
        day2::main,
        day3::main,
        day4::main
    ];
    
    for (day, main) in mains.iter().enumerate() {
        println!(
            "------------------------------------ DAY {} ------------------------------------",
            day + 1
        );
        main();
        println!();
    }
}