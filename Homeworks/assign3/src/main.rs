
fn guess_game(guess: i32, secret: i32) -> i32{
    if guess == secret{
        0
    }
    else if guess < secret{
        -1
    }
    else{
        1 
    }
}

fn main() {
    let mut guess_num: i32 = 30;
    let answer: i32 = 12;
    let mut counter: i32 = 0;
    let mut amount: i32 = 0;

    loop{
        amount +=1;
        if guess_game(guess_num, answer) == 0{
            println!("0");
            break;
        }
        else if guess_game(guess_num, answer) == -1{
            println!("-1");
            guess_num+= 1;
        }
        else{
            guess_num+= -1;
            println!("1");
        }
    }
    println!("You're total guesses was {}", amount);
}
