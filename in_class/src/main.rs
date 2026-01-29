fn RGB (n:char) -> (uB, uB, uB){
/*if  c == 'R'{
    return (255,0,0); }
else if c == 'G'{
    return (0,255,0); }
else if c == 'B'{
    return (0,0,255); }
else{
    return (0,0,0);}
} */
    match c {
        'R' => (255,0,0);
        'G' => (0,255,0);
        'B' => (0,0,255);
    _ => (0,0,0);
}
}


fn main() {
/*let res = get RGB('R');
println! ("{:?}",res);
    */
let idx in 0..letters.len(){
    let res = get_RGB(letters[idx]);
    println!("{:?}",res);
}
}