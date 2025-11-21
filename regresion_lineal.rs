use std::f64;

fn main() {
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];

    let mut w = 0.0;
    let mut b = 0.0;

    let learning_rate = 0.01;
    let epochs = 1000;

    let m = x.len() as f64;

    for epoch in 1..=epochs {
        let mut dw = 0.0;
        let mut db = 0.0;

        for i in 0..x.len() {
            let y_pred = w * x[i] + b;
            let error = y_pred - y[i];
            dw += error * x[i];
            db += error;
        }

        dw = (2.0 / m) * dw;
        db = (2.0 / m) * db;

        w -= learning_rate * dw;
        b -= learning_rate * db;

        if epoch % 200 == 0 {
            let mse: f64 = x
                .iter()
                .enumerate()
                .map(|(i, &xi)| {
                    let pred = w * xi + b;
                    let err = pred - y[i];
                    err * err
                })
                .sum::<f64>()
                / m;

            println!("Epoch {}, MSE: {:.4}, w: {:.4}, b: {:.4}", epoch, mse, w, b);
        }
    }

    println!("Modelo final: w = {:.4}, b = {:.4}", w, b);

    let x_new = 7.0;
    let y_pred_new = w * x_new + b;
    println!("Para x = {}, y_pred ≈ {:.4}", x_new, y_pred_new);
}
