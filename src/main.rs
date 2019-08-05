use rand::Rng;


fn main() {
    let x : [f32; 6] = [3.0,4.0,7.0,8.0,9.0,13.0];
    let y : [f32; 6] = [5.0,8.0,9.0,11.0,13.0,15.0];
    let i : f32 = rand::thread_rng().gen_range(1.0f32, 15.0f32);
    let c : f32 = rand::thread_rng().gen_range(1.0f32, 15.0f32);

    train(&x, &y, i, c, 0.1, 10);
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

    for n in 0..x.len(){
        i_deriv += -2.0*x[n] * (y[n] - (i*x[n] + c));

        c_deriv += -2.0*(y[n] - (i*x[n] + c));
    }
    println!("  - InputW: {}, InputB: {}, i_deriv: {}, c_deriv: {}", i, c, i_deriv, c_deriv);

    (i - (i_deriv / (x.len() as f32) * learning_rate), c - (c_deriv / (x.len() as f32) * learning_rate))
}

fn train(x: &[f32], y: &[f32], i: f32, c: f32, learning_rate: f32, iters: i32) -> (f32, f32, Vec<f32>) {

    let mut history : Vec<f32> = vec![];
    let mut weight = i;
    let mut bias = c;

    let mut vals = (weight, bias);

    for i in 1..iters {
        vals = update_weights(x, y, weight, bias, learning_rate);
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