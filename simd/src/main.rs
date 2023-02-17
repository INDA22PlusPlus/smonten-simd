use std::arch::x86_64::*;
use std::time::{Duration, Instant};
// use thousands::Separable;

fn main() {
    let cases = vec![Case::Tup, Case::Simd];
    for case in cases {
        case.test(
            vec![1, 10,100,1000,10_000, 100_000, 1_000_000, 10_000_000],
            200
        );
    }
}


#[derive(Debug)]
enum Case {
    Tup,
    Simd
}
impl Case {
    fn test(&self, iters: Vec<usize>, n: usize) {
        println!("-------------------------------------");
        println!("{:?}_fib({})", self, n);
        for i in iters {
            let start = Instant::now();
            for j in 0..i {
                let _result = self.run(n);
            }
            let duration = start.elapsed();
            println!("Iterations: {}, Total time: {:?}",
                my_fancy_thousands(i),
                duration);
        }
    }
    fn run(&self, n: usize) {
        let _result = match self {
            Case::Tup => fib_tup(n),
            Case::Simd => fib_simd(n)
        };
    }
}

fn fib_simd(n: usize) -> u32 {
    unsafe {
        let mut b = _mm_set_ps(0.0, 0.0, 1.0, 0.0); // (0, 1, 0, 0)
        for _i in 0..n {
            b = _mm_add_ps(
                _mm_shuffle_ps(b, b, 245), //11110101 -> (x1, x1, 0, 0)
                _mm_shuffle_ps(b, b, 243)  //11110011 -> (0 , x0, 0, 0)
            );
        }
        return _mm_cvtss_f32(b) as u32;         //get first element of b
    }
}

fn fib_tup(n: usize) -> u32 {
    let mut b: (f32, f32) = (0.0, 1.0);
    for _i in 0..n {
        b = (b.1, b.0+b.1);
    }
    b.0 as u32
}

fn my_fancy_thousands(i: usize) -> String {
    i.to_string()
    .as_bytes()
    .rchunks(3)
    .rev()
    .map(std::str::from_utf8)
    .collect::<Result<Vec<&str>, _>>()
    .unwrap()
    .join(",")
}