pub fn tuple_array() {
    println!("Welcome to the tuple array");

    let data = ("hello", 232, 85.45, true);
    println!("{:?}", data);

    // let (para, leng, score, point) = data;

    let marks = [64, 56, 78, 78, 45, 45, 4];
    let sum: i32 = marks.iter().sum();
    let avg = sum as f32 / marks.len() as f32;

    println!("avg: {:?}, sum: {:?}", avg, sum);
}
