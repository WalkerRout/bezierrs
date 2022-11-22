type Point = (f64, f64);

fn factorial(n: u32) -> u32 {
  fn __factorial(n: u32, acc: u32) -> u32 {
    match n {
      0 => acc,
      _ => __factorial(n-1, acc*n)
    }
  }

  let res = __factorial(n, 1);
  res
}

// n choose k
fn binomial_coefficient(n: u32, k: u32) -> u32 {
  (factorial(n)) / (factorial(k) * factorial(n - k))
}

fn bernstein_polynomial(n: u32, k: u32, t: f64) -> f64 {
  //  double res = binomial_coefficient(n, i) * pow(t, i) * pow(1 - t, n - i);
  (binomial_coefficient(n, k) as f64) * f64::powf(t, k.try_into().unwrap()) * f64::powf(1.0-t, (n-k).try_into().unwrap())
}

fn bezier_curve(t: f64, control_points: &Vec<Point>) -> Point {
  let mut p: Point = (0.0, 0.0);
  let n_points = control_points.len();

  for i in 0..n_points {
    let bern_poly = bernstein_polynomial((n_points-1) as u32, i as u32, t);
    p.0 += control_points[i].0 * bern_poly;
    p.1 += control_points[i].1 * bern_poly;
  }

  p
}

// returns a vector of size num_elements+1, as the vector includes the lower and upper bounds a and b
fn linspace(a: f64, b: f64, element_count: u32) -> Vec<f64> {
  let mut spaced_elements: Vec<f64> = Vec::with_capacity((element_count + 1) as usize);
  let step = b / element_count as f64;
  
  spaced_elements.push(a);
  for i in 1..=element_count {
    let index = i as usize;
    spaced_elements.push(spaced_elements[index-1] + step);
  }
  
  spaced_elements
}

fn split_xy(points: Vec<Point>) -> (Vec<f64>, Vec<f64>) {
  points.into_iter().unzip()
}

fn main() {
  let step = 50;
  let spaced = bezier::linspace(0.0, 1.0, step);
  let points: Vec<Point> = vec![(1.0, 1.0), (6.0, 0.0), (8.0, 20.0), (12.0, 2.0)];
  let mut output: Vec<Point> = vec![];

  for i in 0..=step {
    output.push(bezier::bezier_curve(spaced[i as usize], &points));
  }

  let out = bezier::split_xy(output);
  println!("{:?}", out.0);
  println!("{:?}", out.1);
}
