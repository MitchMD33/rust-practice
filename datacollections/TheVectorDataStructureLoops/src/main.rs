//Vector Data and Loops

struct Test {
    score: i32
}
fn main() {
    //let mut my_numbers = vec![1,2,3];
    //
    // let mut my_numbers = Vec::new();
    // my_numbers.push(1);
    // my_numbers.push(2);
    // my_numbers.push(3);
    // my_numbers.pop();
    // my_numbers.len();
    //
    // let two = my_numbers[2];
    //
    // for num in my_numbers {
    //     println!("{:?}", num);
    //}

    let my_scores = vec![
        Test{score: 90},
        Test{score: 88},
        Test{score: 77},
        Test{score: 93},
    ];

    for test in my_scores {
        println!("score is {:?}", test.score);
    }
}
