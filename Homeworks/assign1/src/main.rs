fn Fahrenheit_to_celsius(y:f32) -> f32{
    let x: f32 = 32.0;
    let d: f32 = 5.0/9.0;
    (y-x)*(d)
    

    
}

fn Celsius_to_fahrenheit (k:f32) -> f32{
    let x: f32 = 32.0;
    let d: f32 = 9.0/5.0;
    
    (k*d) + (x)
}
    //const: f32 = 32.0;
fn main() {
    let mut F_temp: f32 = 50.0;
    let mut C_temp: f32 = 17.0;
    let mut count: i32 = 0;
    
    let F_cell: fn(f32) -> f32 = Fahrenheit_to_celsius;
    loop{
    let result_C = F_cell(F_temp);
    println!("The degrees in celsius is: {}",result_C);
        F_temp = F_temp-1.0;
        count+=1;
            if count == 6{
             break;
    }
    }
    count = 0;
    
    let C_farr: fn(f32) -> f32 = Celsius_to_fahrenheit;
    loop{
    let result_F = C_farr(C_temp);
    println!("The degrees in farenheit is: {}",result_F);
        C_temp = C_temp-1.0;
        count+=1;
            if count == 6{
             break;
    }
    }

}
