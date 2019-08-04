use rand::Rng;


fn main() {
    let x : [f32; 6] = [3.0,4.0,7.0,8.0,9.0,13.0];
    let y : [f32; 6] = [5.0,8.0,9.0,11.0,13.0,15.0];
    let i : f32 = rand::thread_rng().gen_range(1.0f32, 15.0f32);
    let c : f32 = rand::thread_rng().gen_range(1.0f32, 15.0f32);

    println!("Cost is {}", cost_function(&x, &y, i, c));
}

fn cost_function(x: &[f32], y: &[f32], i: f32, c: f32) -> f32{

    let mut total_error = 0.0f32;
    for n in 0..x.len(){
        total_error += y[n] - (i * x[n] + c);
    }

    total_error / x.len() as f32
}

fn update_weights(x: &[f32], y: &[f32], i: f32, c: f32, learning_rate: i32) -> (i32, i32){

    let i_deriv = 0;
    let c_deriv = 0;

    for n in 0..x.len(){
        i_deriv += -2*x[n] * (y[n] - (i*x[n] + c));

        # -2(y - (mx + b))
        bias_deriv += -2*(sales[i] - (weight*radio[i] + bias))
    }

    (1 , 2)
}