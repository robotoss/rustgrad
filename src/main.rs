use plotly::common::Title;
use plotly::{Plot, Scatter};

fn main() {
    let xs: Vec<f32> = arange(-5., 5., 0.25);
    let ys: Vec<f32> = xs.clone().into_iter().map(|x| f(x)).collect();
    println!("ys {:?}", ys);
    print_parabola(xs, ys);

    //  print!("{:?}", figure.save("./figures/lineplot_basic.png", None));
}

fn f(x: f32) -> f32 {
    3.0 * f32::powi(x, 2) - 4.0 * x + 5.0
}

fn arange(start: f32, stop: f32, step: f32) -> Vec<f32> {
    let mut array: Vec<f32> = Vec::new();
    let mut last_number = start;
    if start > stop {
        while stop < last_number {
            array.push(last_number);
            last_number = last_number - step;
        }
    } else {
        while stop > last_number {
            array.push(last_number);
            last_number = last_number + step;
        }
    }

    println!("array {:?}", array);
    array
}

fn print_parabola(xs: Vec<f32>, ys: Vec<f32>) {
    // Create data for the line chart

    // Create a scatter plot with the data
    let scatter = Scatter::new(xs, ys).name("Line Chart");

    // Create a plot and add the scatter plot
    let mut plot = Plot::new();
    plot.add_trace(scatter);

    // Customize the plot layout
    plot.set_layout(plotly::Layout::new().title(Title::from("My Line Chart")));

    // Show the plot in a browser window
    plot.show();
}
