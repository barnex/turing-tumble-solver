use magnus::*;
use std::time::Instant;

const B: Addr = BLUE_LEVER;
const R: Addr = RED_LEVER;
const I: bool = true;
const O: bool = false;
use Color::*;

fn main() {
    time(ch32);
    time(ch21);
    time(sequence1);
    time(sequence2);
    time(ch32);
    time(sequence3);
}

fn time<F: Fn()>(f: F) {
    let start = Instant::now();
    f();
    println!("{}s", start.elapsed().as_secs_f32());
}

fn ch32() {
    println!("ch32_set_reset");

    solve_gear(&State::new(3).with_balls([1, 1]), &[B, R], |p| -> bool {
        for flipflop_state in [O, I] {
            if p.with_start(Blue).with_bit(0, flipflop_state).run().bit(0) != false {
                return false;
            }
            if p.with_start(Red).with_bit(0, flipflop_state).run().bit(0) != true {
                return false;
            }
        }
        true
    });
}

fn sequence1() {
    solve_bits(&State::new(4).with_balls([8, 8]), &[B, R, INTERC0], |p| {
        p.clone()
            .run()
            .out_seq
            .eq(&[Blue, Red, Red, Blue, Blue, Blue])
    })
}

fn sequence2() {
    solve_bits(&State::new(4).with_balls([4, 6]), &[B, R], |p| {
        p.clone()
            .run()
            .out_seq
            .eq(&[Blue, Red, Red, Blue, Blue, Blue, Red, Red, Red, Red])
    })
}

fn sequence3() {
    solve_bits(&State::new(6).with_balls([9, 6]), &[B, R], |p| {
        p.clone().run().out_seq.eq(&[
            Blue, Red, Red, Blue, Blue, Blue, Red, Red, Red, Red, Blue, Blue, Blue, Blue, Blue,
        ])
    })
}

fn ch21() {
    // 4-bit counter
    println!("ch21_quantum_number");

    solve_bits(&State::new(4), &[B], |p| {
        for n in 0..16 {
            if p.with_balls([n, 0]).run_verbosity(0).register(0..4) != n as u64 {
                return false;
            }
        }
        true
    })
}
