fn is_even(n:i32) -> bool {
    if n%2 == 0{
        true
    }
    else{
        false
    }
}

fn main() {
    let num_array: [i32;10] = [14, 23, 67, 54, 120, 88, 3, 99, 44, 2];
    let mut counter: i32 = 0;
    let mut piv: i32 = 0;
    let mut sum: i32 = 0;
    let mut comp: i32 = 0;
        
    for num in num_array.iter(){
        
        if is_even(*num) == true{
            println!("This number is even");
        }
        else{
            println!("This number is odd");
        }
        if num%3==0 && num%5 == 0{
            println!("FizzBuzz");
        }
        else if num%3 == 0{
            println!("Fizz");
        }
        else if num%5 == 0{
            println!("Buzz");
        }
        else{
            println!("{}",num);

        }
        
    }
    for num in num_array.iter(){
        sum = sum + num;
        if comp < *num{
            comp = *num;
        }

    }
    println!("The sum of the array is {}",sum);
    println!("The largest num of the array is {} ", comp);
}
