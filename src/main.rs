use std::io;
use rand::prelude::*;
use std::time::{Instant};
use std::thread;
use std::sync::mpsc;
use std::fs::OpenOptions;
use std::io::{Write};
use strum_macros::EnumString;
use std::process;
use std::f64::consts::E;

// Func `enum` to store all the functions that can be integrated
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

// Point `struct` to store the generate points for Figure 1 of the report
#[derive(Copy, Clone)]
struct Point {
    value: i32, // 1, -1 or 0; according to the in(x_rand, y_rand) function [see Equation 3 in the report]
    x: f64, // x-coordinate of the point
    y: f64 // y-coordinate of the point
}

// main function
fn main() {
    println!("Would you like to perform the autorun routine (pre-programmed values)? (y/n) ");

    // Ask for input to see if the user wants to use custom parameters or to run the autoroutine
    let mut input = String::new(); // create new String where the input will be saved

    io::stdin() // save terminal input to the input String object
        .read_line(&mut input)
        .expect("Failed to read line");

    let efficient_mode = false; // if true, then the MC simulation will only sample points from y_min to y_max, even if y_min > 0

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
    // NOTE: all the commented lines were the ones used to generate the values in the report
    /*
    integrate(Func::Polynomial, 0, 1000, 10, -3.0, 3.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 100, -3.0, 3.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 1000, -3.0, 3.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 10000, -3.0, 3.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 100000, -3.0, 3.0, 1.0, 2.0, true, efficient);
     */

    //integrate(Func::Normal, 0, 1000, 100000, 0.0, 0.5, 0.0, 0.0, true, !efficient);
    //integrate(Func::Normal, 0, 1000, 100000, 0.0, 0.5, 0.0, 0.0, true, efficient);
    /*
    integrate(Func::Sine, 0, 1000, 10, -3.1415926535897932384626433832795028841971693993751058209749445923, 0.0, 1.0, 2.0, true, efficient);
    integrate(Func::Sine, 0, 1000, 100, -3.1415926535897932384626433832795028841971693993751058209749445923, 0.0, 1.0, 2.0, true, efficient);
    integrate(Func::Sine, 0, 1000, 1000, -3.1415926535897932384626433832795028841971693993751058209749445923, 0.0, 1.0, 2.0, true, efficient);
    integrate(Func::Sine, 0, 1000, 10000, -3.1415926535897932384626433832795028841971693993751058209749445923, 0.0, 1.0, 2.0, true, efficient);
    integrate(Func::Sine, 0, 1000, 100000, -3.1415926535897932384626433832795028841971693993751058209749445923, 0.0, 1.0, 2.0, true, efficient);
    integrate(Func::Sine, 0, 1000, 1000000, -3.1415926535897932384626433832795028841971693993751058209749445923, 0.0, 1.0, 2.0, true, efficient);
    integrate(Func::Sine, 0, 1000, 10000000, -3.1415926535897932384626433832795028841971693993751058209749445923, 0.0, 1.0, 2.0, true, efficient);
    */
    /*
    integrate(Func::Polynomial, 0, 1000, 10, -3.0, 3.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 100, -3.0, 3.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 1000, -3.0, 3.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 10000, -3.0, 3.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 100000, -3.0, 3.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 1000000, -3.0, 3.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 10000000, -3.0, 3.0, 1.0, 2.0, true, efficient);
    */
    /*
    integrate(Func::Sine, 0, 1000, 10, -6.283185307179586, -3.1415926535897932384626433832795028841971693993751058209749445923, 1.0, 2.0, true, efficient);
    integrate(Func::Sine, 0, 1000, 100, -6.283185307179586, -3.1415926535897932384626433832795028841971693993751058209749445923, 1.0, 2.0, true, efficient);
    integrate(Func::Sine, 0, 1000, 1000, -6.283185307179586, -3.1415926535897932384626433832795028841971693993751058209749445923, 1.0, 2.0, true, efficient);
    integrate(Func::Sine, 0, 1000, 10000, -6.283185307179586, -3.1415926535897932384626433832795028841971693993751058209749445923, 1.0, 2.0, true, efficient);
    integrate(Func::Sine, 0, 1000, 100000, -6.283185307179586, -3.1415926535897932384626433832795028841971693993751058209749445923, 1.0, 2.0, true, efficient);
    integrate(Func::Sine, 0, 1000, 1000000, -6.283185307179586, -3.1415926535897932384626433832795028841971693993751058209749445923, 1.0, 2.0, true, efficient);
    integrate(Func::Sine, 0, 1000, 10000000, -6.283185307179586, -3.1415926535897932384626433832795028841971693993751058209749445923, 1.0, 2.0, true, efficient);
     */
    /*
    integrate(Func::Cosine, 0, 1000, 10, 0.0, 1.5707963267948966192313216916397514420985846996875529104874722961, 1.0, 2.0, true, efficient);
    integrate(Func::Cosine, 0, 1000, 100, 0.0, 1.5707963267948966192313216916397514420985846996875529104874722961, 1.0, 2.0, true, efficient);
    integrate(Func::Cosine, 0, 1000, 1000, 0.0, 1.5707963267948966192313216916397514420985846996875529104874722961, 1.0, 2.0, true, efficient);
    integrate(Func::Cosine, 0, 1000, 10000, 0.0, 1.5707963267948966192313216916397514420985846996875529104874722961, 1.0, 2.0, true, efficient);
    integrate(Func::Cosine, 0, 1000, 100000, 0.0, 1.5707963267948966192313216916397514420985846996875529104874722961, 1.0, 2.0, true, efficient);
    integrate(Func::Cosine, 0, 1000, 1000000, 0.0, 1.5707963267948966192313216916397514420985846996875529104874722961, 1.0, 2.0, true, efficient);
    integrate(Func::Cosine, 0, 1000, 10000000, 0.0, 1.5707963267948966192313216916397514420985846996875529104874722961, 1.0, 2.0, true, efficient);
     */
    /*
    integrate(Func::LN, 0, 1000, 10, 1.0, 2.7182818284590452353602874713526624977572470936999595749669676277, 1.0, 2.0, true, efficient);
    integrate(Func::LN, 0, 1000, 100, 1.0, 2.7182818284590452353602874713526624977572470936999595749669676277, 1.0, 2.0, true, efficient);
    integrate(Func::LN, 0, 1000, 1000, 1.0, 2.7182818284590452353602874713526624977572470936999595749669676277, 1.0, 2.0, true, efficient);
    integrate(Func::LN, 0, 1000, 10000, 1.0, 2.7182818284590452353602874713526624977572470936999595749669676277, 1.0, 2.0, true, efficient);
    integrate(Func::LN, 0, 1000, 100000, 1.0, 2.7182818284590452353602874713526624977572470936999595749669676277, 1.0, 2.0, true, efficient);
    integrate(Func::LN, 0, 1000, 1000000, 1.0, 2.7182818284590452353602874713526624977572470936999595749669676277, 1.0, 2.0, true, efficient);
    integrate(Func::LN, 0, 1000, 10000000, 1.0, 2.7182818284590452353602874713526624977572470936999595749669676277, 1.0, 2.0, true, efficient);
     */
    /*
    integrate(Func::Normal, 0, 1000, 10, -2.0, 2.0, 1.0, 2.0, true, efficient);
    integrate(Func::Normal, 0, 1000, 100, -2.0, 2.0, 1.0, 2.0, true, efficient);
    integrate(Func::Normal, 0, 1000, 1000, -2.0, 2.0, 1.0, 2.0, true, efficient);
    integrate(Func::Normal, 0, 1000, 10000, -2.0, 2.0, 1.0, 2.0, true, efficient);
    integrate(Func::Normal, 0, 1000, 100000, -2.0, 2.0, 1.0, 2.0, true, efficient);
    integrate(Func::Normal, 0, 1000, 1000000, -2.0, 2.0, 1.0, 2.0, true, efficient);
    integrate(Func::Normal, 0, 1000, 10000000, -2.0, 2.0, 1.0, 2.0, true, efficient);
     */
    /*
    integrate(Func::SqrtSine, 0, 1000, 10, 12.566370614359173, 15.707963267948966, 1.0, 2.0, true, efficient);
    integrate(Func::SqrtSine, 0, 1000, 100, 12.566370614359173, 15.707963267948966, 1.0, 2.0, true, efficient);
    integrate(Func::SqrtSine, 0, 1000, 1000, 12.566370614359173, 15.707963267948966, 1.0, 2.0, true, efficient);
    integrate(Func::SqrtSine, 0, 1000, 10000, 12.566370614359173, 15.707963267948966, 1.0, 2.0, true, efficient);
    integrate(Func::SqrtSine, 0, 1000, 100000, 12.566370614359173, 15.707963267948966, 1.0, 2.0, true, efficient);
    integrate(Func::SqrtSine, 0, 1000, 1000000, 12.566370614359173, 15.707963267948966, 1.0, 2.0, true, efficient);
    integrate(Func::SqrtSine, 0, 1000, 10000000, 12.566370614359173, 15.707963267948966, 1.0, 2.0, true, efficient);
     */
    /*
    integrate(Func::Polynomial, 0, 1000, 100000000, -3.0, 3.0, 1.0, 2.0, true, efficient);
    integrate(Func::Sine, 0, 1000, 100000000, -6.283185307179586, -3.1415926535897932384626433832795028841971693993751058209749445923, 1.0, 2.0, true, efficient);
    integrate(Func::Cosine, 0, 1000, 100000000, 0.0, 1.5707963267948966192313216916397514420985846996875529104874722961, 1.0, 2.0, true, efficient);
    integrate(Func::LN, 0, 1000, 100000000, 1.0, 2.7182818284590452353602874713526624977572470936999595749669676277, 1.0, 2.0, true, efficient);
    integrate(Func::Normal, 0, 1000, 100000000, -2.0, 2.0, 1.0, 2.0, true, efficient);
    integrate(Func::SqrtSine, 0, 1000, 100000000, 12.566370614359173, 15.707963267948966, 1.0, 2.0, true, efficient);
     */
    /*
    integrate(Func::Polynomial, 0, 1000, 30, -3.0, 3.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 300, -3.0, 3.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 3000, -3.0, 3.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 30000, -3.0, 3.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 300000, -3.0, 3.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 3000000, -3.0, 3.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 30000000, -3.0, 3.0, 1.0, 2.0, true, efficient);
     */
    /*
    integrate(Func::Sine, 0, 1000, 30, -6.283185307179586, -3.1415926535897932384626433832795028841971693993751058209749445923, 1.0, 2.0, true, efficient);
    integrate(Func::Sine, 0, 1000, 300, -6.283185307179586, -3.1415926535897932384626433832795028841971693993751058209749445923, 1.0, 2.0, true, efficient);
    integrate(Func::Sine, 0, 1000, 3000, -6.283185307179586, -3.1415926535897932384626433832795028841971693993751058209749445923, 1.0, 2.0, true, efficient);
    integrate(Func::Sine, 0, 1000, 30000, -6.283185307179586, -3.1415926535897932384626433832795028841971693993751058209749445923, 1.0, 2.0, true, efficient);
    integrate(Func::Sine, 0, 1000, 300000, -6.283185307179586, -3.1415926535897932384626433832795028841971693993751058209749445923, 1.0, 2.0, true, efficient);
    integrate(Func::Sine, 0, 1000, 3000000, -6.283185307179586, -3.1415926535897932384626433832795028841971693993751058209749445923, 1.0, 2.0, true, efficient);
    integrate(Func::Sine, 0, 1000, 30000000, -6.283185307179586, -3.1415926535897932384626433832795028841971693993751058209749445923, 1.0, 2.0, true, efficient);

    integrate(Func::Cosine, 0, 1000, 30, 0.0, 1.5707963267948966192313216916397514420985846996875529104874722961, 1.0, 2.0, true, efficient);
    integrate(Func::Cosine, 0, 1000, 300, 0.0, 1.5707963267948966192313216916397514420985846996875529104874722961, 1.0, 2.0, true, efficient);
    integrate(Func::Cosine, 0, 1000, 3000, 0.0, 1.5707963267948966192313216916397514420985846996875529104874722961, 1.0, 2.0, true, efficient);
    integrate(Func::Cosine, 0, 1000, 30000, 0.0, 1.5707963267948966192313216916397514420985846996875529104874722961, 1.0, 2.0, true, efficient);
    integrate(Func::Cosine, 0, 1000, 300000, 0.0, 1.5707963267948966192313216916397514420985846996875529104874722961, 1.0, 2.0, true, efficient);
    integrate(Func::Cosine, 0, 1000, 3000000, 0.0, 1.5707963267948966192313216916397514420985846996875529104874722961, 1.0, 2.0, true, efficient);
    integrate(Func::Cosine, 0, 1000, 30000000, 0.0, 1.5707963267948966192313216916397514420985846996875529104874722961, 1.0, 2.0, true, efficient);

    integrate(Func::LN, 0, 1000, 30, 1.0, 2.7182818284590452353602874713526624977572470936999595749669676277, 1.0, 2.0, true, efficient);
    integrate(Func::LN, 0, 1000, 300, 1.0, 2.7182818284590452353602874713526624977572470936999595749669676277, 1.0, 2.0, true, efficient);
    integrate(Func::LN, 0, 1000, 3000, 1.0, 2.7182818284590452353602874713526624977572470936999595749669676277, 1.0, 2.0, true, efficient);
    integrate(Func::LN, 0, 1000, 30000, 1.0, 2.7182818284590452353602874713526624977572470936999595749669676277, 1.0, 2.0, true, efficient);
    integrate(Func::LN, 0, 1000, 300000, 1.0, 2.7182818284590452353602874713526624977572470936999595749669676277, 1.0, 2.0, true, efficient);
    integrate(Func::LN, 0, 1000, 3000000, 1.0, 2.7182818284590452353602874713526624977572470936999595749669676277, 1.0, 2.0, true, efficient);
    integrate(Func::LN, 0, 1000, 30000000, 1.0, 2.7182818284590452353602874713526624977572470936999595749669676277, 1.0, 2.0, true, efficient);

    integrate(Func::Normal, 0, 1000, 30, -2.0, 2.0, 1.0, 2.0, true, efficient);
    integrate(Func::Normal, 0, 1000, 300, -2.0, 2.0, 1.0, 2.0, true, efficient);
    integrate(Func::Normal, 0, 1000, 3000, -2.0, 2.0, 1.0, 2.0, true, efficient);
    integrate(Func::Normal, 0, 1000, 30000, -2.0, 2.0, 1.0, 2.0, true, efficient);
    integrate(Func::Normal, 0, 1000, 300000, -2.0, 2.0, 1.0, 2.0, true, efficient);
    integrate(Func::Normal, 0, 1000, 3000000, -2.0, 2.0, 1.0, 2.0, true, efficient);
    integrate(Func::Normal, 0, 1000, 30000000, -2.0, 2.0, 1.0, 2.0, true, efficient);

    integrate(Func::SqrtSine, 0, 1000, 30, 12.566370614359173, 15.707963267948966, 1.0, 2.0, true, efficient);
    integrate(Func::SqrtSine, 0, 1000, 300, 12.566370614359173, 15.707963267948966, 1.0, 2.0, true, efficient);
    integrate(Func::SqrtSine, 0, 1000, 3000, 12.566370614359173, 15.707963267948966, 1.0, 2.0, true, efficient);
    integrate(Func::SqrtSine, 0, 1000, 30000, 12.566370614359173, 15.707963267948966, 1.0, 2.0, true, efficient);
    integrate(Func::SqrtSine, 0, 1000, 300000, 12.566370614359173, 15.707963267948966, 1.0, 2.0, true, efficient);
    integrate(Func::SqrtSine, 0, 1000, 3000000, 12.566370614359173, 15.707963267948966, 1.0, 2.0, true, efficient);
    integrate(Func::SqrtSine, 0, 1000, 30000000, 12.566370614359173, 15.707963267948966, 1.0, 2.0, true, efficient);
    */
    //integrate(Func::Sine, 0, 1000, 10, 0.0, 6.283185307179586, 1.0, 2.0, true, efficient);
    
    integrate(Func::Polynomial, 0, 1000, 10, 3.0, 6.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 100, 3.0, 6.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 1000, 3.0, 6.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 10000, 3.0, 6.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 100000, 3.0, 6.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 30, 3.0, 6.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 300, 3.0, 6.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 3000, 3.0, 6.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 30000, 3.0, 6.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 300000, 3.0, 6.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 1000000, 3.0, 6.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 3000000, 3.0, 6.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 10000000, 3.0, 6.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 30000000, 3.0, 6.0, 1.0, 2.0, true, efficient);
    integrate(Func::Polynomial, 0, 1000, 100000000, 3.0, 6.0, 1.0, 2.0, true, efficient);
    //integrate(Func::Polynomial, 0, 1000, 100000000, -3.0, 3.0, 1.0, 2.0, true, efficient);
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
    let mut f_input = String::new(); // save terminal input to the f_input String object
    io::stdin() // save terminal input to the input String object
        .read_line(&mut f_input)
        .expect("Failed to read line"); // this similar to a 'throws' function; if a wrong input is provided, the program will terminate and send this message

    let func: Func; // constant that store the selected function

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

        // ask for input of the A and 
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
        process::abort(); // abort program if the selected function is not valid
    }

    let f_desc: String = desc(func); // String description of the function

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

    // NOTE: the following lines will not be explained in detail because they do essentially the same as the previous lines (the ones to input trapezoid number)
    // choose p_tot
    println!("Please enter the number of points to generate for the MC integration.");
    let mut points_input = String::new();
    io::stdin()
        .read_line(&mut points_input)
        .expect("Failed to read line");
    let points_tot: i32 = points_input.trim().parse()
        .expect("Not a number!");

    // choose N
    println!("Please enter the number of repetitions for the MC and trapezoid integration (N).");
    let mut n_in = String::new();
    io::stdin()
        .read_line(&mut n_in)
        .expect("Failed to read line");
    let n: i32 = n_in.trim().parse()
        .expect("Not a number!");

    // choose a
    println!("Please enter the lower bound of the integral (a).");
    let mut a_in = String::new();
    io::stdin()
        .read_line(&mut a_in)
        .expect("Failed to read line");
    let a: f64 = a_in.trim().replace("pi","3.1415926535897932384626433832795028841971693993751058209749445923").parse()
        .expect("Not a number!");

    // choose b
    println!("Please enter the upper bound of the integral (b).");
    let mut b_in = String::new();
    io::stdin()
        .read_line(&mut b_in)
        .expect("Failed to read line");
    let b: f64 = b_in.trim().replace("pi","3.1415926535897932384626433832795028841971693993751058209749445923").parse()
        .expect("Not a number!");

    println!("");

    // integrate with the previously defined parameters
    integrate(func, trap, points_tot, n, a, b, A, n_p, false, efficient);
}

// integration funcion
fn integrate(func: Func, trap: i32, points_tot: i32, n: i32, a: f64, b: f64, A: f64, n_p: f64, auto: bool, efficient: bool) {
    let generating_points = false; // constant that determines if points are to be generated (for Figure 1)

    let f_desc: String = desc(func); // function String description

    let mut mc_res: Vec<f64> = Vec::new(); // results of the MC integration (stored as a vector of `f64`)
    let mut trap_res: Vec<f64> = Vec::new(); // results of the trapezoid integration (stored as a vector of `f64`)
    let mut points: Vec<Point> = Vec::new(); // generated points in the MC integration (stored as a vector of `Point`)

    let (tx, rx) = mpsc::channel::<f64>(); // `f64` sender and receivers for mc_res
    let (tx_trap, rx_trap) = mpsc::channel::<f64>(); // `f64` sender and receivers for trap_res
    let (tx_points, rx_points) = mpsc::channel::<Vec<Point>>(); // `Vec<Point>` sender and receivers for points

    let mut handles = vec![]; // vector that stores the handles of the generated threads

    let now = Instant::now(); // save the current 'instant'; used to calculate the runtime of the program
    
    // Integration
    let mut min; // minimum value of the function for a <= x <= b
    let max: f64; // maximum value of the function  for a <= x <= b

    max = max_of_f(a, b, func, A, n_p); // function to determine the maximum value of the function for a <= x <= b
    min = min_of_f(a, b, func, A, n_p); // function to determine the minimum value of the function for a <= x <= b

    let mut offset: f64 = 0.0; // stores how much the min of the function is above the x-axis

    if min > 0.0 && efficient { // if efficient mode is activated, then the offset is the minimum value
        offset = min;
    } else if min > 0.0 && !efficient { // otherwise, min = 0.0
        min = 0.0;
    }

    let mut i = 1; // thread generator counter

    let threads = 10; // number of threads to generate
    let cycles_per_thread = n / threads; // this is the number of values that each thread needs to generate

    while i <= threads { // repeat this code to generate each thread
        let tx_temp = tx.clone(); // create a `clone` of tx (this is needed because the original object is 'owned' by the main thread and can't be used within 'secondary' threads)
        let tx_temp_trap = tx_trap.clone(); // create a `clone` of tx_trap
        let tx_temp_points = tx_points.clone(); // create a `clone` of tx_points
    
        let handle = thread::spawn(move || { // create a thread and save its handle which will be used later
            let mut i_loc = 1; // index to store the current area count (of this thread)

            while i_loc <= cycles_per_thread { // do for each area needed to be generated by this thread
                let area_mc = generate_area_mc(points_tot, min, max, a, b, func, A, n_p); // calculate an area value with MC method
                let area_traps = calc_area_trap(trap, a, b, func, A, n_p); // calculate an area value with trapezoids method
                // only if points are to be generated generate 1000 points for the selected function
                if generating_points {
                    let points = generate_points(a, b, func, min, max, 1000, A, n_p); // generate 1000 points
                    tx_temp_points.send(points).unwrap(); // send points to the Sender<Vec<Point>> object
                }

                tx_temp.send(area_mc).unwrap(); // send MC calculated area to the tx_temp Sender<f64> object
                tx_temp_trap.send(area_traps).unwrap(); // send trapezoid calculated aread to the tx_temp Sender<f64> object
                
                // Progress indicator (used for debugging, as it consumes resources and increases computational time)
                //let prog = f64::from(i_loc) * f64::from(threads) * 10.0 / f64::from(cycles_per_thread);
                //println!("{prog}%");

                i_loc += 1; // add 1 to the area counter of this thread
            }
            if !auto { // if the function is not performing the autorun routine, print when a thread finishes
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
                let area = mean(&mc_res[..]).unwrap() + offset * (b - a); // average of the generated area values stored in 'mc_res', the Vec<64> object; IMPORTANT: if in efficient mode, add the offset * (b-a) to the result from the MC integration
                let stdev = std_deviation(&mc_res[..]).unwrap(); // standard deviation of the generated area values stored in 'mc_res', the Vec<64> object
                let se = stdev / f64::from(length).sqrt(); // calculate the standard error of the mean value by dividing the std. dev. by the sqrt. of the number of areas (converted to a float)

                let t = now.elapsed(); // elapsed runtime
                let extra: String; // String to store the A and n_p parameter ONLY for the polynomial function
                let extra_latex: String; // String to store the A and n_p parameter ONLY for the polynomial function (in LaTeX format for gnuplot)

                // if the selected function is the Polynomial function
                if func == Func::Polynomial {
                    extra = format!(", A: {A}, n_p: {n_p}"); // show A and n_p
                    extra_latex = format!(" {A} {n_p}"); // show A and n_p (without labels)
                } else { // otherwise, extra and extra_latex should be empty strings
                    extra = "".to_string();
                    extra_latex = "".to_string();
                }

                // print MC results to the terminal
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
                );

                // save MC results to mc_results.txt
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

                // generate a numerical index of the function (in order to easily filter results)
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

                let t_secs = t.as_secs_f64(); // elapsed time in seconds (but in an `f64`, for non-integer values)

                // LaTeX Results; save results in LaTeX format for gnuplot to mc_res_latex.txt
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
            break; // break from the loop, so that the main thread can start the processing the other results
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

                // if the selected function is the Polynomial function
                if func == Func::Polynomial {
                    extra = format!(", A: {A}, n_p: {n_p}"); // show A and n_p
                } else { // otherwise, extra and extra_latex should be empty strings
                    extra = "".to_string();
                }
                
                // print trapezoid results to the terminal
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
                );

                // save trapezod results to trapezoid_results.txt
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

    if generating_points { // only if points were generated
        for received in rx_points {
            for p in received {
                points.push(p); // 'push' (i.e., save) each point from the received `Vec<Point>` object to the points  `Vec<Point>`
            }
    
            if points.len() as i32 == 1000 { // once the area count is equal to the desired number of points to be generated...
                for point in points { // write the values of each point to the generated_points.txt file
                    let file_name = "generated_points.txt"; // name of the file where the results will be stored.
                    let mut file = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .append(true)
                        .open(file_name)
                        .expect("Error reading file!");
                    writeln!(file, "{} {} {}",
                        point.x,
                        point.y,
                        point.value
                    ) // write results to the file
                        .expect("Error writing to file"); // throw an error if there was a problem writing to the file 
                }
                break; // break from the loop once all points have been written to the file
            }
            
        }      
    }

    if !auto { // if not in autorun, do not terminate program until the user inputs anything (so that the window doesn't close automatically)
        println!("Press any key to terminate the program and close the window.");

        let mut wait_i = String::new();
        io::stdin() // save terminal input to the input String object
            .read_line(&mut wait_i)
            .expect("Failed to read line");
    }
}

// calculate f(x) for any given x, func, A and n_p; returns an `f64`
fn f(x: f64, func: Func, A: f64, n_p: f64) -> f64 {
    let res: f64;

    match func {
        Func::Normal => res = E.powf(-1.0 * x.powf(2.0)),
        Func::Quadratic => res = x * x,
        Func::Sine => res = x.sin(),
        Func::SqrtSine => res = (x.sin()).sqrt(),
        Func::Cosine => res = x.cos(),
        Func::LN => res = x.ln(),
        Func::MIT => res = (x.tan().powf(1.0/3.0)) / (x.sin() + x.cos()).powf(2.0),
        Func::Polynomial => res = A * x.powf(n_p)
    }
    
    return res;
}

// generate points for Figure 1 of the report; returns a `Vec<Point>`
fn generate_points(a: f64, b: f64, func: Func, min: f64, max: f64, points_tot: i32, A: f64, n_p: f64) -> Vec<Point> {
    let mut rng = rand::thread_rng(); // the thread random number generator

    let mut i = 1; // index to count the number of generated points
    let mut points: Vec<Point> = Vec::new();

    while i <= points_tot { // repeat until all points have been generated
        let x: f64 = rng.gen_range(a..=b); // generate x coordinate
        let y: f64 = rng.gen_range(min..=max); // generate y coordinate

        let f_x = f(x, func, A, n_p); // calculate f(x)

        let res: i32;

        // compute the in(x_rand, y_rand) function [Equation 3 in the report] for the randomly generated point
        if y > 0.0 && y <= f_x { // test if the point is within the function; above the x-axis
            res = 1; // if so, set the value to 1
        } else if y < 0.0 && y >= f_x { // test if the point is within the function; below the x-axis
            res = -1; // if so, set the value to -1
        } else {
            res = 0; // otherwise, set the value to 0
        }
        i += 1; // add 1 to the counter of the generated points

        let point = Point { // create the Point
            value: res,
            x: x,
            y: y
        };

        points.push(point); // 'push' the Point to the points Vec
    }
    
    return points; // return all the generated points
}

// calculate a definite integral with the MC method
fn generate_area_mc(points_tot: i32, min: f64, max:f64, a: f64, b: f64, func: Func, A: f64, n_p: f64) -> f64 {
    let mut rng = rand::thread_rng(); // the thread random number generator

    let mut i = 1; // index to count the number of generated points
    let mut points_in: f64 = 0.0; // variable to count the number of points that are within the function

    // let mut points_in_neg: f64 = 0.0;
    while i <= points_tot { // repeat until all points have been generated
        let x: f64 = rng.gen_range(a..=b); // generate x coordinate
        let y: f64 = rng.gen_range(min..=max); // generate y coordinate

        let f_x = f(x, func, A, n_p); // calculate f(x)

        // compute the in(x_rand, y_rand) function [Equation 3 in the report] for the randomly generated point
        if y > 0.0 && y <= f_x { // test if the point is within the function; above the x-axis
            points_in += 1.0; // if so, add 1 to the counter of the points within the function
        } else if y < 0.0 && y >= f_x { // test if the point is within the function; below the x-axis
            points_in += -1.0; // if so, add -1 to the counter of the points within the function
        }
        i += 1; // add 1 to the counter of the generated points
    }

    let area: f64 = points_in / f64::from(points_tot) * (b - a) * (max - min); // calculate the results of the definite integral
    return area; // return the generated area value
    
}

// calculate the max of f for a <= x <= b with sampling; returns an `f64`
fn max_of_f(a: f64, b: f64, func: Func, A: f64, n_p: f64) -> f64 {
    let n = 10000; // number of samples to be taken
    let mut i = 1; // counter for the sampling loop

    let range = b - a; // range to be sampled
    let delta_x = range / f64::from(n); // size of each uniformly distributed sample

    let mut max = -100000000000.0; // starting max value; as it is replaced, it needs to be very low so that any other value is greater than the starting value

    while i <= n {
        let f_i = f(a + delta_x * f64::from(i - 1), func, A, n_p); // sample for x = a + delta_x * (i - 1); with this, the sampling starts at a and finishes at b, adding delta_x for each iteration

        if f_i > max { // if the current sample is greater than the previous maximum value, replace it 
            max = f_i;
        }

        i += 1; // add 1 to the counter
    }

    return max * 1.05; // multiply the max by 1.05 to account for some possible missed values in the sampling
}

// calculate the min of f for a <= x <= b with sampling; returns an `f64`
fn min_of_f(a: f64, b: f64, func: Func, A: f64, n_p: f64) -> f64 {
    let n = 10000; // number of samples to be taken
    let mut i = 1; // counter for the sampling loop

    let range = b - a; // range to be sampled
    let delta_x = range / f64::from(n); // size of each uniformly distributed sample

    let mut min = 100000000000.0; // starting min value; as it is replaced, it needs to be very high so that any other value is smaller than the starting value

    while i <= n {
        let f_i = f(a + delta_x * f64::from(i - 1), func, A, n_p); // sample for x = a + delta_x * (i - 1); with this, the sampling starts at a and finishes at b, adding delta_x for each iteration

        if f_i < min { // if the current sample is smaller than the previous maximum value, replace it 
            min = f_i;
        }

        i += 1; // add 1 to the counter
    }

    if min <= 0.0 { // if min <= 0, multiply the min by 1.05 to account for some possible missed values in the sampling (so that the value is smaller, i.e. y is 'lower', therefore a greater area is covered)
        return min * 1.05;
    } else { // if min > 0, multiply the min by 0.95 to account for some possible missed values in the sampling (so that the value is smaller, i.e. y is 'lower', therefore a greater area is covered)
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

// fucntion to generate a string description of each function
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

// calculate the definite integral with the trapezoid integration method; NOT RELEVANT TO THIS REPORT (but preserved in order to keep compatibility)
// therefore, this will NOT BE DESCRIBED IN DETAIL
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