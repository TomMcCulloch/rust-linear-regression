use rand::Rng;


fn main() {
    let x = [3,4,7,8,9,13];
    let y = [5,8,9,11,13,15];
    let i : f32 = rand::thread_rng().gen_range(1.0f32, 15.0f32);
    let c : f32 = rand::thread_rng().gen_range(1.0f32, 15.0f32);

    cost_function(&x, &y, i, c);
    print!("1");
}

fn cost_function(x: &[i32], y: &[i32], i: f32, c: f32) -> f32{


    
    1.0f32
}