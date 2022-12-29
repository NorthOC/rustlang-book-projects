fn main(){

    let mut fibnums: [i128; 2] = [0, 1];

    for _item in 1..=100 {

        println!("Fibonacci: {}", fibnums[0]);

        let sum = fibnums[0] + fibnums[1];
        fibnums[0] = fibnums[1];
        fibnums[1] = sum;

    }

}