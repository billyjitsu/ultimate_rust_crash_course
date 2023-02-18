const STARTING_MISSLES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    
    let mut missiles: i32 = STARTING_MISSLES;
    let ready: i32 = READY_AMOUNT;
    let fired: i32 = 2;
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
}
