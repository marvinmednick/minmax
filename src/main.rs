use minmax::{MinMax,MinMax::Value, MinMax::NA,MinMax::Max};

fn main() {
    let v1 : MinMax<i32> = NA;
    let v2 : MinMax<i32> = Max;
    let v3 = Value(10.0);
    let v4 : MinMax<i32> = Value(5);
    let v5 = v4 + Value(10);

    println!("Hello, minmax world! {} {} {} {} {}",v1,v2,v3,v4,v5);
}
