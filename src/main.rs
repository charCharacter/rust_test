// Implement basic function to split some generic computational work between threads.
// Split should occur only on some threshold - if computational work (input length) is shorter
// than this threshold, no splitting should occur and no threads should be created.
//
// You get as input:
//
// 1. Vec<T>
// 2. Function f(t: T) -> R
//
// Threshold can be just constant.
//
// You should return:
// 1. Up to you, but probably some Vec of the same length as input(1)
//
// Code should be published on github.

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::sync::{Arc, RwLock};
use num_cpus;


const THRESHOLD: usize = 5;

#[derive(Copy, Clone)]
pub struct TestStruct{
    a: i32,
    b: i32
}


fn safe_handle<T:Copy+Send + 'static,R:Copy +Send + 'static, F: Fn(T)->R+Clone + Send + 'static + std::marker::Sync>(vec:Vec<T>,function:F)->Vec<R>{

    let mut result = Vec::new();
    let el_count= vec.len();

    if el_count>THRESHOLD {
        let mut  num_threads: usize = num_cpus::get();
        if num_threads > vec.len() {
            num_threads=vec.len()
        }

        let offset = el_count/num_threads;
        let mut iter: usize = 0;
        let (tx, rx): (Sender<R>, Receiver<R>) = mpsc::channel();
        let function_arc = Arc::new(RwLock::new(function));
        let mut handles = vec![];
        while iter<el_count {
            let thread_tx = tx.clone();
            let vec_i = (&vec[iter..(iter+offset)]).to_vec();

            let function_arc_clone= function_arc.clone();
            vec_i.len();
            handles.push(thread::spawn( move || {
                let function = function_arc_clone.read().unwrap();

                for i in 0..vec_i.len(){
                    let result = function(vec_i[i]);
                    thread_tx.send(result).expect("Error while sending result");
                }
            })
            );

            iter += offset;
        }

        for _ in 0..vec.len(){
            result.push(rx.recv().unwrap())
        }
        for  handle in handles{
            handle.join().unwrap();
        }
    }
    else {
        for i in vec {
            result.push(function(i))
        }
    }
    result
}

fn fun (item: TestStruct)->i32{
    item.a*item.b
}

fn main() {
    let mut vec = Vec::new();
    let  test_1 = TestStruct{
        a:3,
        b:2
    };
    let  test_2 = TestStruct{
        a:5,
        b:2
    };
    let  test_3 = TestStruct{
        a:1,
        b:3
    };
    // let  test_4 = TestStruct{
    //     a:4,
    //     b:7
    // };
    // let  test_5 = TestStruct{
    //     a:4,
    //     b:3
    // };
    // let  test_6 = TestStruct{
    //     a:9,
    //     b:7
    // };
    // let  test_7 = TestStruct{
    //     a:3,
    //     b:2
    // };
    // let  test_8 = TestStruct{
    //     a:5,
    //     b:2
    // };
    // let  test_9 = TestStruct{
    //     a:1,
    //     b:3
    // };
    // let  test_10 = TestStruct{
    //     a:4,
    //     b:7
    // };
    // let  test_11 = TestStruct{
    //     a:4,
    //     b:3
    // };
    // let  test_12 = TestStruct{
    //     a:9,
    //     b:7
    // };
    // let  test_13 = TestStruct{
    //     a:3,
    //     b:2
    // };
    // let  test_14 = TestStruct{
    //     a:5,
    //     b:2
    // };
    // let  test_15 = TestStruct{
    //     a:1,
    //     b:3
    // };
    // let  test_16 = TestStruct{
    //     a:4,
    //     b:7
    // };
    // let  test_17 = TestStruct{
    //     a:4,
    //     b:3
    // };
    // let  test_18 = TestStruct{
    //     a:9,
    //     b:7
    // };
    // let  test_19 = TestStruct{
    //     a:3,
    //     b:2
    // };
    // let  test_20 = TestStruct{
    //     a:5,
    //     b:2
    // };
    // let  test_21 = TestStruct{
    //     a:1,
    //     b:3
    // };
    // let  test_22 = TestStruct{
    //     a:4,
    //     b:7
    // };
    // let  test_23 = TestStruct{
    //     a:4,
    //     b:3
    // };
    // let  test_24 = TestStruct{
    //     a:9,
    //     b:7
    // };
    vec.push(test_1);
    vec.push(test_2);
    vec.push(test_3);
    // vec.push(test_4);
    // vec.push(test_5);
    // vec.push(test_6);
    // vec.push(test_7);
    // vec.push(test_8);
    // vec.push(test_9);
    // vec.push(test_10);
    // vec.push(test_11);
    // vec.push(test_12);
    // vec.push(test_13);
    // vec.push(test_14);
    // vec.push(test_15);
    // vec.push(test_16);
    // vec.push(test_17);
    // vec.push(test_18);
    // vec.push(test_19);
    // vec.push(test_20);
    // vec.push(test_21);
    // vec.push(test_22);
    // vec.push(test_23);
    // vec.push(test_24);

    let result = safe_handle(vec,fun);

    for  vec_item in &result{
        println!("{}",vec_item);
    }

}

