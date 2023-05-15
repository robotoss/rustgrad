use std::fmt;
use std::ops::Add;

use plotly::common::Title;
use plotly::{Plot, Scatter};

fn main() {
    let xs: Vec<f64> = arange(-5., 5., 0.25);
    let ys: Vec<f64> = xs.clone().into_iter().map(|x| f(x)).collect();
    println!("ys {:?}", ys);
    // print_parabola(xs, ys);

    let h: f64 = 0.000001;
    let x: f64 = 2. / 3.;
    let deviation = (f(x + h) - f(x)) / h;
    println!("deviation {}", deviation);

    // let get more complex
    // inputs
    let mut a: f64 = 2.0;
    let b: f64 = -3.0;
    let c: f64 = 10.0;
    let d = a * b + c;
    println!("d {}", d);

    let h: f64 = 0.0001;
    let d1 = a * b + c;
    a = a + h;
    let d2 = a * b + c;
    println!("d1 {}", d1);
    println!("d2 {}", d2);
    println!("slope {}", (d2 - d1) / h);

    let a = Value { data: 2.0 };
    let b = Value { data: -3.0 };
    println!("{}", a);
    let x = a + b;
    println!("{}", x);
}

fn f(x: f64) -> f64 {
    3.0 * f64::powi(x, 2) - 4.0 * x + 5.0
}

fn arange(start: f64, stop: f64, step: f64) -> Vec<f64> {
    let mut array: Vec<f64> = Vec::new();
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

struct Value {
    pub data: f64,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "Value(data={})", self.data,)
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            data: self.data + other.data,
        }
    }
}
