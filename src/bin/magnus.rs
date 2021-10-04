use magnus::*;

const B: Addr = BLUE_LEVER;
const R: Addr = RED_LEVER;
const I: bool = true;
const O: bool = false;

use magnus::Color::*;

// Solutions to the challenges in the Turing Tumble (TM) Puzzle book.
fn main() {
    ch1();
    ch2();
    ch3();
    ch4();
    ch5();
    ch6();
    ch7();
    ch8();
    ch9();
    ch10();
    ch11();
    ch12();
    ch13();
    ch14();
    ch15();
    ch16();
    ch17();
    ch18();
    ch19();
    ch20();
    ch21();
    ch22();
    ch23();
    ch24();
    ch25();
    ch26();
    ch27();
    ch28();
    ch29();
    ch30();
    ch31();
    ch32();
    ch33();
}

fn default() -> State {
    State::default()
}

fn ch1() {
    println!("ch1_gravity");
    let result = State {
        balls: [8, 8],
        entry: [B, 0],
        ..default()
    }
    .run();

    assert_eq!(&result.output_str(), "bbbbbbbb");
}

fn ch2() {
    println!("ch1_reentry");
    let result = State {
        balls: [8, 8],
        entry: [B, 0],
        ..default()
    }
    .run();

    assert_eq!(&result.output_str(), "bbbbbbbb");
}

fn ch3() {
    println!("ch3_ignition");
    let result = State {
        balls: [8, 8],
        entry: [R, R],
        ..default()
    }
    .run();

    assert_eq!(&result.output_str(), "brrrrrrrr");
}

fn ch4() {
    println!("ch4_fusion");
    let result = State {
        balls: [8, 8],
        entry: [B, B],
        start_button: Red,
        ..default()
    }
    .run();

    assert_eq!(&result.output_str(), "rbbbbbbbb");
}


fn ch5() {
    println!("ch5_entropy");
    let result = State {
        balls: [8, 8],
        entry: [R, B],
        ..default()
    }
    .run();

    assert_eq!(&result.output_str(), "brbrbrbrbrbrbrbr");
}

fn ch6() {
    // equivalent to challenge 5 up to routing
    println!("ch6_total_internal_reflection");
    let result = State {
        balls: [8, 8],
        entry: [R, B],
        ..default()
    }
    .run();

    assert_eq!(&result.output_str(), "brbrbrbrbrbrbrbr");
}

fn ch7() {
    // equivalent to challenge 1 up to routing
    println!("ch7_path_of_least_resistance");
    let result = State {
        balls: [8, 8],
        entry: [B, 0],
        ..default()
    }
    .run();
    assert_eq!(&result.output_str(), "bbbbbbbb");
}

fn ch8() {
    println!("ch8_depolarization");
    let result = State {
        balls: [8, 8],
        entry: [0, 0],
        instr: vec![ijmp(0, B, R)],
        mem: vec![O],
        ..default()
    }
    .run();
    assert_eq!(&result.output_str(), "brbrbrbrbrbrbrbr");
}

fn ch9() {
    println!("ch9_dimers");
    let result = State {
        balls: [8, 8],
        entry: [0, B],
        instr: vec![ijmp(0, B, R)],
        mem: vec![I],
        ..default()
    }
    .run();
    assert_eq!(&result.output_str(), "bbrbbrbbrbbr");
}

fn ch10() {
    // note early optimisation:
    // bit0 jmp0 has to be B, regardless the rest of the program.
    // I.e. the ability to reject a partial program based on partial output.
    println!("ch10_double_bond");
    let result = State {
        balls: [8, 8],
        entry: [0, 1],
        instr: vec![
            ijmp(0, B, R), //
            ijmp(1, B, R), //
        ],
        mem: vec![I, O],
        ..default()
    }
    .run();
    assert_eq!(&result.output_str(), "bbrrbbrrbbrrbbrr");
}

fn ch11() {
    println!("ch11_selectivity");
    let result = State {
        balls: [2, 0],
        entry: [0, FALL],
        instr: vec![
            ijmp(0, 2, 5), //
            ijmp(1, B, B), //
            ijmp(2, B, B), //
            ijmp(3, B, B), //
            ijmp(4, B, B), //
            ijmp(5, B, B), //
        ],
        mem: vec![I, O, O, O, O, O],
        ..default()
    }
    .run();
    assert_eq!(&result.mem_str()[1..], "01001");
}

fn ch12() {
    println!("ch12_duality_part1");
    let result = State {
        balls: [8, 8],
        entry: [0, FALL],
        instr: vec![
            ijmp(0, FALL, INTERC0), //
        ],
        mem: vec![O],
        ..default()
    }
    .run();
    assert_eq!(result.intercept[0], Some(Blue));
}

fn ch13() {
    println!("ch13_duality_part2");
    let result = State {
        balls: [8, 8],
        entry: [0, INTERC0],
        instr: vec![
            ijmp(0, R, INTERC0), //
        ],
        mem: vec![I],
        ..default()
    }
    .run();
    assert_eq!(result.intercept[0], Some(Red));
}

fn ch14() {
    println!("ch14_duality_part3");
    let case0 = State {
        balls: [8, 8],
        entry: [0, INTERC0],
        instr: vec![
            ijmp(0, R, INTERC0), //
        ],
        mem: vec![O],
        ..default()
    };

    let case1 = case0.with_bit(0, I);

    println!("case0");
    assert_eq!(case0.run().intercept[0], Some(Blue));

    println!("case1");
    assert_eq!(case1.run().intercept[0], Some(Red));
}

fn ch15() {
    println!("ch15_inversion");
    let case0 = State {
        balls: [8, 8],
        entry: [0, INTERC0],
        instr: vec![
            ijmp(0, 1, INTERC0), //
            ijmp(1, R, B),       //
        ],
        mem: vec![I, O],
        ..default()
    };
    let case1 = case0.with_bit(1, I);

    println!("case0");
    assert_eq!(case0.run().intercept[0], Some(Blue));

    println!("case1");
    assert_eq!(case1.run().intercept[0], Some(Red));
}

fn ch16() {
    println!("ch16_termination");
    let result = State {
        balls: [8, 8],
        entry: [0, FALL],
        instr: vec![
            ijmp(0, 1, B),       //
            ijmp(1, INTERC0, B), //
        ],
        mem: vec![O, O],
        ..default()
    }
    .run();
    assert_eq!(&result.output_str(), "bbb");
    assert_eq!(result.intercept[0], Some(Blue));
}

fn ch17() {
    // Note: test for unused inputs
    println!("ch17_fixed_ratio");
    let result = State {
        balls: [8, 8],
        entry: [0, 2],
        instr: vec![
            // left
            ijmp(0, B, 1), //
            ijmp(1, B, R), //
            // right
            ijmp(2, 3, R),       //
            ijmp(3, INTERC0, R), //
        ],
        mem: vec![O, I, O, O],
        ..default()
    }
    .run();
    assert_eq!(&result.output_str(), "bbbrrr");
}

fn ch18() {
    // NAND gate.
    // Note: test for unused inputs
    println!("ch18_entanglement");
    let p = State {
        balls: [8, 8],
        entry: [0, FALL],
        instr: vec![
            ijmp(0, 1, INTERC1),       //
            ijmp(1, INTERC0, INTERC1), //
        ],
        mem: vec![O, O],
        ..default()
    };

    println!("case 00 => F");
    assert_eq!(
        p.with_mem(vec![O, O]).run().intercept[..2],
        [None, Some(Blue)]
    );
    println!("case 01 => F");
    assert_eq!(
        p.with_mem(vec![O, I]).run().intercept[..2],
        [None, Some(Blue)]
    );
    println!("case 10 => F");
    assert_eq!(
        p.with_mem(vec![I, O]).run().intercept[..2],
        [None, Some(Blue)]
    );
    println!("case 00 => T");
    assert_eq!(
        p.with_mem(vec![I, I]).run().intercept[..2],
        [Some(Blue), None]
    );
}

fn ch19() {
    // AND gate.
    println!("ch19_entanglement");
    let p = State {
        balls: [8, 8],
        entry: [0, INTERC0],
        instr: vec![
            ijmp(0, 1, R),       //
            ijmp(1, INTERC0, R), //
        ],
        mem: vec![O, O],
        ..default()
    };

    println!("case 00 => r");
    assert_eq!(p.with_mem(vec![O, O]).run().intercept[0], Some(Red));
    println!("case 01 => r");
    assert_eq!(p.with_mem(vec![O, I]).run().intercept[0], Some(Red));
    println!("case 10 => r");
    assert_eq!(p.with_mem(vec![I, O]).run().intercept[0], Some(Red));
    println!("case 11 => b");
    assert_eq!(p.with_mem(vec![I, I]).run().intercept[0], Some(Blue));
}

fn ch20() {
    // OR gate.
    println!("ch20_symbiosis");
    let p = State {
        balls: [8, 8],
        entry: [0, INTERC0],
        instr: vec![
            ijmp(0, INTERC0, 1), //
            ijmp(1, INTERC0, R), //
        ],
        mem: vec![O, O],
        ..default()
    };

    println!("case 00 => r");
    assert_eq!(p.with_mem(vec![O, O]).run().intercept[0], Some(Red));
    println!("case 01 => b");
    assert_eq!(p.with_mem(vec![O, I]).run().intercept[0], Some(Blue));
    println!("case 10 => b");
    assert_eq!(p.with_mem(vec![I, O]).run().intercept[0], Some(Blue));
    println!("case 11 => b");
    assert_eq!(p.with_mem(vec![I, I]).run().intercept[0], Some(Blue));
}

fn ch21() {
    // 4-bit counter
    println!("ch21_quantum_number");
    let p = State {
        balls: [0, 0],
        entry: [0, FALL],
        instr: vec![
            ijmp(0, 1, B), //
            ijmp(1, 2, B), //
            ijmp(2, 3, B), //
            ijmp(3, B, B), //
        ],
        mem: vec![O, O, O, O],
        ..default()
    };

    for n in 0..16 {
        println!("case {}", n);
        assert_eq!(p.with_balls([n, 0]).run().register(0..4), n as u64);
    }
}

fn ch22() {
    // 4-bit count down
    println!("ch22_depletion");
    let p = State {
        balls: [0, 0],
        entry: [0, FALL],
        instr: vec![
            ijmp(0, B, 1), //
            ijmp(1, B, 2), //
            ijmp(2, B, 3), //
            ijmp(3, B, B), //
        ],
        mem: vec![I, I, I, I],
        ..default()
    };

    for n in 0..16 {
        println!("case {}", n);
        assert_eq!(p.with_balls([n, 0]).run().register(0..4), 15 - n as u64);
    }
}

fn ch23() {
    println!("ch23_tetrad");
    let p = State {
        balls: [8, 8],
        entry: [0, FALL],
        instr: vec![
            ijmp(0, B, 1),       //
            ijmp(1, B, 2),       //
            ijmp(2, B, INTERC0), //
        ],
        mem: vec![O, O, I],
        ..default()
    };

    assert_eq!(p.run().output_str(), "bbbb");
}

fn ch24() {
    println!("ch24_ennead");
    let p = State {
        balls: [12, 12],
        entry: [0, FALL],
        instr: vec![
            ijmp(0, B, 1),       //
            ijmp(1, B, 2),       //
            ijmp(2, B, 3),       //
            ijmp(3, B, INTERC0), //
        ],
        mem: vec![I, O, O, I],
        ..default()
    };

    assert_eq!(p.run().output_str(), "bbbbbbbbb");
}

fn ch25() {
    println!("ch25_regular_expression");
    let p = State {
        balls: [8, 8],
        entry: [0, 3],
        instr: vec![
            // left
            ijmp(0, B, 1), //
            ijmp(1, B, 2), //
            ijmp(2, B, R), //
            // right
            ijmp(3, R, 4),       //
            ijmp(4, R, INTERC0), //
        ],
        mem: vec![I, O, I, I, I],
        ..default()
    };

    assert_eq!(p.run().output_str(), "bbbbbbrrr");
}

fn ch26() {
    println!("ch26_nucleus");
    let p = State {
        balls: [10, 10],
        entry: [0, 2],
        instr: vec![
            // left
            ijmp(0, B, 1), //
            ijmp(1, B, R), //
            // right
            ijmp(2, B, INTERC0), //
        ],
        mem: vec![I, I, I],
        ..default()
    };

    assert_eq!(p.run().output_str(), "bbbbrbbbb");
}

fn ch27() {
    // Note: strong physical constraints
    // Note: high-cardinality specification (512 tests).
    println!("ch27_reflection");
    let p = State {
        balls: [8, 8],
        entry: [0, 9],
        instr: vec![
            ijmp(0, 1, 5),
            //left register
            ijmp(1, 2, 2),
            ijmp(2, 3, 3),
            ijmp(3, 4, 4),
            ijmp(4, B, B),
            //middle register
            ijmp(5, 6, 6),
            ijmp(6, 7, 7),
            ijmp(7, 8, 8),
            ijmp(8, R, R),
            //right register
            ijmp(9, INTERC0, INTERC0),
        ],
        mem: vec![I, O, O, O, O, O, O, O, O, O],
        ..default()
    };

    for n in 0..(1 << 9) {
        let complement = (!n) & 0b111111111;
        println!("case {}: {:09b} -> {:09b}", n, n, complement);
        assert_eq!(p.with_register(1..10, n).run().register(1..10), complement);
    }
}

fn ch28() {
    println!("ch28_latch");
    let p = State {
        balls: [8, 8],
        entry: [0, FALL],
        instr: vec![
            ijmp(0, 1, R),
            ijmp(0, FALL, B), // part of the latch @ mem[0]
        ],
        mem: vec![I],
        ..default()
    };
    assert_eq!(p.run().output_str(), "bbbbbbbb");
}

fn ch29() {
    // Note latch asymmetry. We could, in general, constrain that jmp0 <= jmp1 (except for special registers?),
    // to cull mirror symmetry.
    println!("ch29_one_shot_switch");
    let p = State {
        balls: [8, 8],
        entry: [0, B],
        instr: vec![
            ijmp(0, 1, R),
            ijmp(0, FALL, B), // part of the latch @ mem[0]
        ],
        mem: vec![O],
        ..default()
    };
    assert_eq!(p.run().output_str(), "brbbbbbbb");
}

fn ch30() {
    // Note latch asymmetry. We could, in general, constrain that jmp0 <= jmp1 (except for special registers?),
    // to cull mirror symmetry.

    println!("ch30_overflow");
    let p = State {
        balls: [0, 0],
        entry: [0, B],
        instr: vec![
            // register A
            ijmp(0, 1, B),
            ijmp(1, 2, B),
            ijmp(2, 3, B),
            // overflow latch
            ijmp(3, 3, B),
            ijmp(3, FALL, B),
        ],
        mem: vec![O, O, O, O],
        ..default()
    };

    for n in 0..20 {
        //println!("case {}", n);
        let out = p.with_balls([n, 0]).run_verbosity(0);
        if n <= 7 {
            assert_eq!(out.register(0..3), n as u64);
            assert_eq!(out.register(3..4), 0);
        } else {
            // Puzzle does not specify the register value in case of overflow
            assert_eq!(out.register(3..4), 1);
        }
    }
}

fn ch31() {
    println!("ch31_supervised_learning");
    let p = State {
        balls: [0, 0],
        entry: [6, FALL],
        instr: vec![
            // register A
            ijmp(0, 1, B),
            ijmp(1, 2, B),
            ijmp(2, B, B),
            // register B
            ijmp(3, 4, B),
            ijmp(4, 5, B),
            ijmp(5, B, B),
            // switch
            ijmp(6, 7, 8),    //6
            ijmp(6, FALL, 3), //7
            ijmp(6, 0, FALL), //8
        ],
        mem: vec![O; 7],
        ..default()
    };

    let verbosity = 1;
    for n in 0..=7 {
        println!("case {}, switch L", n);
        let l = p.with_bit(6, O).with_balls([n, 0]).run_verbosity(verbosity);
        assert_eq!(l.register(0..3), n as u64);
        assert_eq!(l.register(3..6), 0);

        println!("case {}, switch R", n);
        let r = p.with_bit(6, I).with_balls([n, 0]).run_verbosity(verbosity);
        assert_eq!(r.register(0..3), 0);
        assert_eq!(r.register(3..6), n as u64);
    }
}

fn ch32() {
    println!("ch32_set_reset");
    let p = State {
        balls: [1, 1],
        entry: [0, 1],
        instr: vec![ijmp(0, B, 2), ijmp(0, 2, R), ijmp(0, B, R)],
        mem: vec![O],
        ..default()
    };

    let b = p.with_start(Blue);
    let r = p.with_start(Red);

    for flipflop_state in [O, I] {
        println!("case: flipflop starts {}, release blue", flipflop_state);
        assert_eq!(b.with_bit(0, flipflop_state).run().bit(0), O);
        println!("case: flipflop starts {}, release red", flipflop_state);
        assert_eq!(r.with_bit(0, flipflop_state).run().bit(0), I);
    }
}

fn ch33() {
    println!("ch33_teleportation");
    let p = State {
        balls: [8, 8],
        entry: [6, 4],
        instr: vec![
            // switch A @ mem[0]
            ijmp(0, 1, 2),    // 0 (Input)
            ijmp(0, FALL, R), // 1 (Output for I)
            ijmp(0, B, FALL), // 2 (Output for O)
            // set-reset B @ mem[1]
            ijmp(1, INTERC0, 5),       // 3 (Reset)
            ijmp(1, 5, INTERC0),       // 4 (Set)
            ijmp(1, INTERC0, INTERC0), // 5 (Output)
            // toggle @ mem[2]
            ijmp(2, 0, 3), // 6
        ],
        mem: vec![O, O, I],
        ..default()
    };

    for init_a in [O, I] {
        for init_b in [O, I] {
            println!("case A={}, B={}", init_a, init_b);
            let p = p.with_mem(vec![init_a, init_b, I]).run();
            assert_eq!(p.bit(1), p.bit(0)); // make B point in the same direction as A...
            assert_eq!(p.bit(0), init_a); // ... without changing the final direction of A.
        }
    }
}
