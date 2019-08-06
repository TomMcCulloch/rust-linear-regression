use rand::Rng;


fn main() {
    let x : [f32; 10] = [3.0,4.0,7.0,8.0,9.0,13.0,13.5,14.1,18.9,19.2];
    let y : [f32; 10] = [5.0,8.0,9.0,11.0,13.0,15.0,15.2,17.2,21.0,22.1];
    let i : f32 = rand::thread_rng().gen_range(1.0f32, 15.0f32);
    let c : f32 = rand::thread_rng().gen_range(1.0f32, 15.0f32);

    train(&x, &y, i, c, 0.001, 10);
}

fn cost_function(x: &[f32], y: &[f32], i: f32, c: f32) -> f32{

    let mut total_error = 0.0f32;
    for n in 0..x.len(){
        total_error += y[n] - (i * x[n] + c);
    }

    total_error / x.len() as f32
}

fn update_weights(x: &[f32], y: &[f32], i: f32, c: f32, learning_rate: f32) -> (f32, f32){

    let mut i_deriv : f32 = 0.0;
    let mut c_deriv : f32 = 0.0;
    println!("  - InputW: {}, InputB: {}, i_deriv: {}, c_deriv: {}", i, c, i_deriv, c_deriv);

    for n in 0..x.len(){
        i_deriv += -2.0*x[n] * (y[n] - (i*x[n] + c));

        c_deriv += -2.0*(y[n] - (i*x[n] + c));
    }
    println!("  - InputW: {}, InputB: {}, i_deriv: {}, c_deriv: {}", i, c, i_deriv, c_deriv);

    i_deriv = i -(i_deriv / (x.len() as f32)) * learning_rate;
    c_deriv = c - (c_deriv / (x.len() as f32)) * learning_rate;
    println!("returning i: {}, c: {}", i_deriv, c_deriv);

    (i_deriv,c_deriv)
}

fn train(x: &[f32], y: &[f32], i: f32, c: f32, learning_rate: f32, iters: i32) -> (f32, f32, Vec<f32>) {

    let mut history : Vec<f32> = vec![];
    let mut weight = i;
    let mut bias = c;

    for i in 1..iters {
        let vals = update_weights(x, y, weight, bias, learning_rate);
        weight = vals.0;
        bias = vals.1;
        let cost: f32 = cost_function(x, y, weight, bias);
        history.push(cost);

        if i % 1 == 0 {
            println!("Iteration: {}, Weight: {}, Bias: {}, Cost: {}", i, weight, bias, cost);
        }
    }

    (weight,bias,history)
}