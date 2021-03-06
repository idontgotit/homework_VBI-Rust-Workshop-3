// Bai 1
fn change_value(input:u32, output: &mut u32) -> u32{
    if input ==1 {
        *output =3;
    }
    else {
        *output = 4;
    }

    *output
}


fn main() {
	let x = change_value(10,&mut 20);
	println!("{:?}", x );

}


// Bai 2
fn vector_is_prime(num: u64, p: &[u64]) -> bool {
    for &i in p {
        if num > i && num % i != 0 {
            return false;
        }
    }

    return true
}

fn main() {
    let mut count: u32 = 1;
    let mut num: u64 = 1;
    let mut primes: Vec<u64> = Vec::new();
    primes.push(1);
    // let mut test = primes;
    while count < 10 {
        num += 2;
        if vector_is_prime(num, &primes) {
        	println!("{:?}", &num);
            count += 1;
            primes.push(num);
        }
    }
    println!("{:?}", primes);
}



// Bai 3
fn main() {
    let mut values = vec![10, 11, 12];
    let v = &values;
    let mut max = 0;
    for n in v {
        max = std::cmp::max(max, *n);
    }

    println!("max is {}", max);
    println!("Converting to percentages of maximum value...");
    for n in &mut values {
        *n = 100 * (*n) / max;
    }
    println!("values: {:#?}", values);
}




// Bai 4
fn main(){
    let mut a = vec![1,2,3,4,5];
    // let i = 0;
    loop {
        let (a, c) = test(&mut a);
        println!("{:?}", a);
        if c >=5 {break;}
    }
}

pub fn test(a: &mut Vec<u8>) -> (Vec<u8>, i32) {
    let mut b:Vec<u8> = Vec::new();
    let mut c:usize = 0;
    loop {
        if a.len() == 0 { break; }
        if a.len() == c { break; }
        let d = a.pop().unwrap();
        c = c+1;
        b.push(d);
    }
    (b, c as i32)
}
