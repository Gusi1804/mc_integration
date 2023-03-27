use std::f32::consts::PI;
use std::io;
use rand::prelude::*;
use std::time::{Instant};
use std::thread;
use std::sync::mpsc;
use std::fs::OpenOptions;
use std::io::{Write};
use strum_macros::EnumString;
use std::process;

#[derive(Copy, Clone)]
#[derive(PartialEq, EnumString)]
enum Func {
    Normal,
    Quadratic,
    Sine,
    SqrtSine,
    Cosine,
    LN,
    MIT,
    Polynomial
}

fn main() {
    println!("Would you like to perform the autorun routine (pre-programmed values)? (y/n) ");

    // Ask for input to see if the user wants to use custom parameters or to run the autoroutine
    let mut input = String::new(); // create new String where the input will be saved

    io::stdin() // save terminal input to the input String object
        .read_line(&mut input)
        .expect("Failed to read line");


    // Ask for input to see if the user wants to run 'efficient' mode (if min > 0, only sample in that area)
    /*
    println!("Would you like to run the 'efficient' mode? (y/n) ");
    
    let mut eff_in = String::new(); // create new String where the input will be saved
    let mut efficient_mode: bool;

    io::stdin() // save terminal input to the input String object
        .read_line(&mut eff_in)
        .expect("Failed to read line");

    if eff_in.trim() == "y".to_string() {
        efficient_mode = true;
    } else if eff_in.trim() == "n".to_string() {
        efficient_mode = false;
    } else {
        println!("Invalid input! Please try again.");
        process::abort();
    }
     */
    let efficient_mode = true;

    // run the mode selected by the user
    if input.trim() == "y".to_string() {
        auto(efficient_mode);
    } else if input.trim() == "n".to_string() {
        manual(efficient_mode);
    } else {
        println!("Invalid input! Please try again.");
        process::abort();
    }
}

// autorutine; used in order to avoid typing all values manually in the terminal
fn auto(efficient: bool) {
    /*
    integrate(Func::Polynomial, 0, 1000, 10, -3.0, 3.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 100, -3.0, 3.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 1000, -3.0, 3.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 10000, -3.0, 3.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 100000, -3.0, 3.0, 1.0, 2.0, true, efficient);
     */

    integrate(Func::Normal, 0, 1000, 100000, 0.0, 0.5, 0.0, 0.0, true, !efficient);
    integrate(Func::Normal, 0, 1000, 100000, 0.0, 0.5, 0.0, 0.0, true, efficient);
}

// manual routine; used to ask for user input
fn manual(efficient: bool) {
    // list all available functions
    println!("Hi! Please select a function to analyze:");
    println!("n: e^(-x^2)");
    println!("q: x^2");
    println!("s: sin(x)");
    println!("sqs: sqrt(sin(x))");
    println!("c: cos(x)");
    println!("l: ln(x)");
    println!("m: MIT Integration Bee ()");
    println!("p: Polynomial");

    // ask user for function selection
    let mut f_input = String::new();
    io::stdin() // save terminal input to the input String object
        .read_line(&mut f_input)
        .expect("Failed to read line");

    let func: Func;

    // define default values of A and n for the polynomial function (they are both 0 because they are not relevant for the other functions)
    let mut A: f64 = 0.0;
    let mut n_p: f64 = 0.0;
    
    // convert user selection to `Func` Enum
    if f_input.trim() == "n".to_string() {
        func = Func::Normal;
    } else if f_input.trim() == "q".to_string() {
        func = Func::Quadratic;
    } else if f_input.trim() == "s".to_string() {
        func = Func::Sine;
    } else if f_input.trim() == "sqs".to_string() {
        func = Func::SqrtSine;
    } else if f_input.trim() == "c".to_string() {
        func = Func::Cosine;
    } else if f_input.trim() == "l".to_string() {
        func = Func::LN;
    } else if f_input.trim() == "m".to_string() {
        func = Func::MIT;
    } else if f_input.trim() == "p".to_string() {
        func = Func::Polynomial;

        // for the
        let mut A_in = String::new();
        io::stdin() // save terminal input to the input String object
            .read_line(&mut A_in)
            .expect("Failed to read line"); // this similar to a 'throws' function; if a wrong input is provided, the program will terminate and send this message

        A = A_in.trim().parse() // convert the String input to an f64
            .expect("Not a number!");

        let mut n_p_in = String::new();
        io::stdin() // save terminal input to the input String object
            .read_line(&mut n_p_in)
            .expect("Failed to read line"); // this similar to a 'throws' function; if a wrong input is provided, the program will terminate and send this message

        n_p = n_p_in.trim().parse() // convert the String input to an f64
            .expect("Not a number!");
    } else {
        println!("Invalid input for the function! Please try again.");
        process::abort();
    }

    let f_desc: String = desc(func);

    println!("");
    println!("Selected function: {f_desc}");
    println!("");

    println!("Please enter the number of trapezoids to generate."); // prompt to request user input

    let mut input = String::new(); // create new String where the input will be saved

    io::stdin() // save terminal input to the input String object
        .read_line(&mut input)
        .expect("Failed to read line"); // this similar to a 'throws' function; if a wrong input is provided, the program will terminate and send this message

    let trap: i32 = input.trim().parse() // convert the String input to an i32
        .expect("Not a number!"); // this similar to a 'throws' function; if the input was not an integer, the program will terminate and send this message

    println!("Please enter the number of points to generate for the MC integration.");
    let mut points_input = String::new();
    io::stdin()
        .read_line(&mut points_input)
        .expect("Failed to read line");
    let points_tot: i32 = points_input.trim().parse()
        .expect("Not a number!");

    println!("Please enter the number of repetitions for the MC and trapezoid integration (n).");
    let mut n_in = String::new();
    io::stdin()
        .read_line(&mut n_in)
        .expect("Failed to read line");
    let n: i32 = n_in.trim().parse()
        .expect("Not a number!");

    println!("Please enter the lower bound of the integral (a).");
    let mut a_in = String::new();
    io::stdin()
        .read_line(&mut a_in)
        .expect("Failed to read line");
    let a: f64 = a_in.trim().replace("pi","3.1415926535897932384626433832795028841971693993751058209749445923").parse()
        .expect("Not a number!");

    println!("Please enter the upper bound of the integral (b).");
    let mut b_in = String::new();
    io::stdin()
        .read_line(&mut b_in)
        .expect("Failed to read line");
    let b: f64 = b_in.trim().replace("pi","3.1415926535897932384626433832795028841971693993751058209749445923").parse()
        .expect("Not a number!");

    println!("");

    integrate(func, trap, points_tot, n, a, b, A, n_p, false, efficient);
}

fn integrate(func: Func, trap: i32, points_tot: i32, n: i32, a: f64, b: f64, A: f64, n_p: f64, auto: bool, efficient: bool) {
    let f_desc: String = desc(func);

    let mut mc_res: Vec<f64> = Vec::new();
    let mut trap_res: Vec<f64> = Vec::new();

    let (tx, rx) = mpsc::channel::<f64>();
    let (tx_trap, rx_trap) = mpsc::channel::<f64>();

    let mut handles = vec![];

    let now = Instant::now(); // save the current 'instant'; used to calculate the runtime of the program
    
    // Integration
    let mut min = 0.0;
    let max:f64;

    /*
    if a < 0.0 && b > 0.0 && func == Func::Normal {
        max = 1.0;
    } else if func == Func::Sine {
        if b - a >= 2.0 * f64::from(PI) {
            min = -1.0;
            max = 1.0;
        } else {
            let f_a = f(a, Func::Sine, A, n_p);
            let f_b = f(b, Func::Sine, A, n_p);

            if f_a >= 0.0 && f_b >= 0.0 {
                min = 0.0;
                max = max_of_f(a, b, func, A, n_p);
            } else if f_a <= 0.0 && f_b <= 0.0 {
                max = 0.0;
                min = min_of_f(a, b, func, A, n_p);
            } else {
                min = min_of_f(a, b, func, A, n_p);
                max = max_of_f(a, b, func, A, n_p);
            }
        }
    }  else if func == Func::Cosine {
        if b - a >= 2.0 * f64::from(PI) {
            min = -1.0;
            max = 1.0;
        } else {
            let f_a = f(a, Func::Cosine, A, n_p);
            let f_b = f(b, Func::Cosine, A, n_p);

            if f_a >= 0.0 && f_b >= 0.0 {
                min = 0.0;
                max = max_of_f(a, b, func, A, n_p);
            } else if f_a <= 0.0 && f_b <= 0.0 {
                max = 0.0;
                min = min_of_f(a, b, func, A, n_p);
            } else {
                min = min_of_f(a, b, func, A, n_p);
                max = max_of_f(a, b, func, A, n_p);
            }
        }
    } else if func == Func::SqrtSine {
        if b - a < f64::from(PI) {
            let pi_mult_min = (a / (2.0 * f64::from(PI))).floor() * 2.0 * f64::from(PI);
            let pi_mult_max = pi_mult_min + f64::from(PI);

            if a >= pi_mult_min && b <= pi_mult_max {
                min = min_of_f(a, b, func, A, n_p);
                max = max_of_f(a, b, func, A, n_p);
            } else {
                println!("Invalid input for the function! Please try again.");
                process::abort();
            }
        } else {
            println!("Invalid input for the function! Please try again.");
            process::abort();
        }
    } else if func == Func::MIT || func == Func::Polynomial {
        min = min_of_f(a, b, func, A, n_p);
        if min > 0.0 {
            min = 0.0;
        }
        max = max_of_f(a, b, func, A, n_p);
        //println!("max: {max}");
    } else if f(a, func, A, n_p) > f(b, func, A, n_p) {
        if func == Func::LN {
            if a > 0.0 && b > 0.0 {
                min = min_of_f(a, b, func, A, n_p);
            } else {
                println!("Invalid input for the function! Please try again.");
                process::abort();
            }
        }
        max = f(a, func, A, n_p);
    } else {
        max = f(b, func, A, n_p)
    }
     */
    max = max_of_f(a, b, func, A, n_p);
    min = min_of_f(a, b, func, A, n_p);

    let mut offset: f64 = 0.0;

    if min > 0.0 && efficient {
        offset = min;
    } else if min > 0.0 && !efficient {
        min = 0.0;
    }

    let mut i = 1;

    let threads = 10; // number of threads to generate
    let cycles_per_thread = n / threads; // this is the number of values that each thread needs to generate

    while i <= threads { // repeat this code to generate each thread
        let tx_temp = tx.clone(); // create a copy of tx (this is needed because the original object is 'owned' by the main thread and can't be used within 'secondary' threads)
        let tx_temp_trap = tx_trap.clone();
    
        let handle = thread::spawn(move || { // create a thread and save its handle which will be used later
            let mut i_loc = 1; // index to store the current area count (of this thread)

            while i_loc <= cycles_per_thread { // do for each area needed to be generated by this thread
                let area_mc = generate_area_mc(points_tot, min, max, a, b, func, A, n_p); // calculate an area value with MC method
                let area_traps = calc_area_trap(trap, a, b, func, A, n_p); // calculate an area value with trapezoids method

                tx_temp.send(area_mc).unwrap(); // send MC calculated aread to the tx_temp Sender<f64> object
                tx_temp_trap.send(area_traps).unwrap(); // send trapezoid calculated aread to the tx_temp Sender<f64> object

                i_loc += 1; // add 1 to the area counter of this thread
            }
            if !auto {
                println!("Finished thread {i} of {threads}"); // print when the thread finishes
            }
        });

        handles.push(handle); // push handle to the handles vector, which will be used later to wait for all thread to be done before executing the remaining code of the main thread
        i += 1; // add 1 to the threads counter
    }

    for handle in handles { // perform this code for each of the handles in the handles vector
        handle.join().unwrap(); // this code is what I referenced earlier; basically, the main thread will not continued with its execution untill all the threads are done
    }

    println!("");

    for received in rx { // for each value that the rx Receiver<f64> gets from the tx Sender<f64> object, store it to the received constant (an f64)
        mc_res.push(received); // save the received value to the mc_res Vec<f64>

        if mc_res.len() as i32 == n { // once the area count is equal to the desired number of areass to be generated...
            if points_tot > 0 {
                println!("–––––––––––––––––––––");

                let length = mc_res.len() as i32; // number of generated areas
                let area = mean(&mc_res[..]).unwrap() + offset * (b - a); // average of the generated area values stored in 'mc_res', the Vec<64> object
                let stdev = std_deviation(&mc_res[..]).unwrap(); // standard deviation of the generated area values stored in 'mc_res', the Vec<64> object
                let se = stdev / f64::from(length).sqrt(); // calculate the standard error of the mean value by dividing the std. dev. by the sqrt. of the number of areas (converted to a float)

                let t = now.elapsed(); // elapsed runtime
                let extra: String;
                let extra_latex: String;

                if func == Func::Polynomial {
                    extra = format!(", A: {A}, n_p: {n_p}");
                    extra_latex = format!(" {A} {n_p}");
                } else {
                    extra = "".to_string();
                    extra_latex = "".to_string();
                }

                println!("MC Integration");
                println!("Mean: {}, SE: {}, STDEV: {}, f: {}, a: {}, b: {}, n: {}, points: {}, t: {:?}{}",
                    area,
                    se,
                    stdev,
                    f_desc,
                    a,
                    b,
                    length,
                    points_tot,
                    t,
                    extra
                ); // print result to the console

                let file_name = "mc_results.txt"; // name of the file where the results will be stored.
                let mut file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .append(true)
                    .open(file_name)
                    .expect("Error reading file!");
                writeln!(file, "Mean: {}, SE: {}, STDEV: {}, f: {}, a: {}, b: {}, n: {}, points: {}, t: {:?}{}",
                    area,
                    se,
                    stdev,
                    f_desc,
                    a,
                    b,
                    length,
                    points_tot,
                    t,
                    extra
                ) // write results to the file
                    .expect("Error writing to file"); // throw an error if there was a problem writing to the file 

                let num_desc: i32;
                match func {
                    Func::Normal => num_desc = 1,
                    Func::Quadratic => num_desc = 7,
                    Func::Sine => num_desc = 3,
                    Func::SqrtSine => num_desc = 5,
                    Func::Cosine => num_desc = 4,
                    Func::LN => num_desc = 6,
                    Func::MIT => num_desc = 8,
                    Func::Polynomial => num_desc = 2
                }

                let t_secs = t.as_secs_f64();

                // LaTeX Results
                let file_name = "mc_res_latex.txt"; // name of the file where the results will be stored.
                let mut file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .append(true)
                    .open(file_name)
                    .expect("Error reading file!");
                writeln!(file, "{} {} {} {} {} {:?} {} {} {}{}",
                    area,
                    stdev,
                    se,
                    n,
                    points_tot,
                    t_secs,
                    num_desc,
                    a,
                    b,
                    extra_latex
                ) // write results to the file
                    .expect("Error writing to file"); // throw an error if there was a problem writing to the file 
            }
            break; // break from the loop, so that the main thread can start the processing the results from the areas and volumes
        }
    }

    for received in rx_trap { // for each value that the rx Receiver<f64> gets from the tx Sender<f64> object, store it to the received constant (an f64)
        trap_res.push(received); // save the received value to the trap_res Vec<f64>

        if trap_res.len() as i32 == n { // once the area count is equal to the desired number of areass to be generated...
            if trap > 0 {
                println!("–––––––––––––––––––––");

                let length = trap_res.len() as i32; // number of generated areas
                let area = mean(&trap_res[..]).unwrap(); // average of the generated area values stored in 'trap_res', the Vec<64> object
                let stdev = std_deviation(&trap_res[..]).unwrap(); // standard deviation of the generated area values stored in 'mc_res', the Vec<64> object
                let se = stdev / f64::from(length).sqrt(); // calculate the standard error of the mean value by dividing the std. dev. by the sqrt. of the number of areas (converted to a float)

                let t = now.elapsed(); // elapsed runtime
                let extra: String;

                if func == Func::Polynomial {
                    extra = format!(", A: {A}, n_p: {n_p}");
                } else {
                    extra = "".to_string();
                }
                
                println!("Trapezoid Integration");
                println!("Mean: {}, SE: {}, STDEV: {}, f: {}, a: {}, b: {}, n: {}, traps: {}, t: {:?}{}",
                    area,
                    se,
                    stdev,
                    f_desc,
                    a,
                    b,
                    length,
                    trap,
                    t,
                    extra
                ); // print result to the console

                let file_name = "trapezoid_results.txt"; // name of the file where the results will be stored.
                let mut file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .append(true)
                    .open(file_name)
                    .expect("Error reading file!");
                writeln!(file, "Mean: {}, SE: {}, STDEV: {}, f: {}, a: {}, b: {}, n: {}, traps: {}, t: {:?}{}",
                    area,
                    se,
                    stdev,
                    f_desc,
                    a,
                    b,
                    length,
                    trap,
                    t,
                    extra
                ) // write results to the file
                    .expect("Error writing to file"); // throw an error if there was a problem writing to the file 
            }
            break; // break from the loop, so that the main thread can start the processing the results from the areas and volumes
        }
    }

    println!("–––––––––––––––––––––");

    if !auto {
        println!("Press any key to terminate the program and close the window.");

        let mut wait_i = String::new();
        io::stdin() // save terminal input to the input String object
            .read_line(&mut wait_i)
            .expect("Failed to read line");
    }
}

fn f(x: f64, func: Func, A: f64, n_p: f64) -> f64 {
    let res: f64;

    let e = 2.71828182845904523536028747135266250f64;
    match func {
        Func::Normal => res = e.powf(-1.0 * x.powf(2.0)),
        Func::Quadratic => res = x * x,
        Func::Sine => res = x.sin(),
        Func::SqrtSine => res = (x.sin()).sqrt(),
        Func::Cosine => res = x.cos(),
        Func::LN => res = x.ln(),
        Func::MIT => res = (x.tan().powf(1.0/3.0)) / (x.sin() + x.cos()).powf(2.0),
        Func::Polynomial => res = A * x.powf(n_p)
    }
    
    //return e^(-x^2.0);
    return res;
}

fn generate_area_mc(points_tot: i32, min: f64, max:f64, a: f64, b: f64, func: Func, A: f64, n_p: f64) -> f64 {
    let mut rng = rand::thread_rng(); // the thread random number generator

    let mut i = 1; // index to count the number of generated points
        let mut points_in: f64 = 0.0; // variable to count the number of points that are within the function
        // let mut points_in_neg: f64 = 0.0;
        while i <= points_tot { // repeat until all points have been generated
            let x: f64 = rng.gen_range(a..=b); // generate x coordinate
            let y: f64 = rng.gen_range(min..=max); // generate y coordinate

            let f_x = f(x, func, A, n_p);

            if y > 0.0 && y <= f_x { // test if the point is within the function
                points_in += 1.0; // if so, add 1 to the counter of the points within the function
            } else if y < 0.0 && y >= f_x {
                points_in += -1.0;
            }
            i += 1; // add 1 to the counter of the generated points
        }

        let area: f64 = points_in / f64::from(points_tot) * (b - a) * (max - min); // calculate the area value
        return area; // return the generated area value
}

fn calc_area_trap(traps: i32, a: f64, b: f64, func: Func, A: f64, n_p: f64) -> f64 {
    let range = b - a;

    let delta_x = range / f64::from(traps);

    let mut i = 1;

    let mut area = 0.0;

    while i <= traps {
        let f1 = f(a + delta_x * f64::from(i - 1), func, A, n_p);
        let f2 = f(a + delta_x * f64::from(i), func, A, n_p);
        let mut trap = (delta_x * (f1 + f2) / 2.0).abs();
        
        if f1 < 0.0 && f2 < 0.0 {
            trap *= -1.0;
        }

        area += trap;
        i += 1;
    }

    return area;
}

fn max_of_f(a: f64, b: f64, func: Func, A: f64, n_p: f64) -> f64 {
    let n = 10000;
    let mut i = 1;

    let range = b - a;
    let delta_x = range / f64::from(n);

    let mut max = -100000000000.0;

    while i <= n {
        let f_i = f(a + delta_x * f64::from(i - 1), func, A, n_p);

        if f_i > max {
            max = f_i;
        }

        i += 1;
    }

    return max * 1.05;
}

fn min_of_f(a: f64, b: f64, func: Func, A: f64, n_p: f64) -> f64 {
    let n = 10000;
    let mut i = 1;

    let range = b - a;
    let delta_x = range / f64::from(n);

    let mut min = 100000000000.0;

    while i <= n {
        let f_i = f(a + delta_x * f64::from(i - 1), func, A, n_p);

        if f_i < min {
            min = f_i;
        }

        i += 1;
    }

    if min <= 0.0 {
        return min * 1.05;
    } else {
        return min * 0.95;
    }
}

// function to calculate the mean of a 'slice' (a part) of a Vec<f64>, returns an 'optional' f64
fn mean(data: &[f64]) -> Option<f64> {
    let sum = data.iter().sum::<f64>() as f64; // sum the values of the 'slice'
    let count = data.len(); // count the values of the 'slice'

    match count {
        positive if positive > 0 => Some(sum / count as f64), // only return the mean if the 'slice' has a length greater than 0
        _ => None,
    }
}

// function to calculate the standard deviation of a 'slice' (a part) of a Vec<f64>, returns an 'optional' f64
fn std_deviation(data: &[f64]) -> Option<f64> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => { // return value if the length of the 'slice' is greater than 0
            let variance = data.iter().map(|value| { // calculate the variance of the 'slice', by calculating the square of the difference of each value from the mean, then adding all the resultant values, and dividing the result by the amount of numbers within the 'slice'
                let diff = data_mean - (*value as f64);

                diff * diff
            }).sum::<f64>() / (count as f64 - 1.0) as f64;

            Some(variance.sqrt()) // return the sqrt of the variance
        },
        _ => None
    }
}

fn desc(func: Func) -> String {
    let f_desc: String;

    match func {
        Func::Normal => f_desc = "e^(-(x^2))".to_string(),
        Func::Quadratic => f_desc = "x^2".to_string(),
        Func::Sine => f_desc = "sin(x)".to_string(),
        Func::SqrtSine => f_desc = "sqrt(sin(x))".to_string(),
        Func::Cosine => f_desc = "cos(x)".to_string(),
        Func::LN => f_desc = "ln(x)".to_string(),
        Func::MIT => f_desc = "MIT Integration Bee".to_string(),
        Func::Polynomial => f_desc = "Ax^n".to_string(),
    }

    return f_desc;
}