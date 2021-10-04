use crate::*;

pub fn solve_gear<F>(init: &State, ext_addr: &[Addr], test: F)
where
    F: Fn(&State) -> bool,
{
    for entry in entrypoints(init.instr.len() as u8, ext_addr) {
        println!("entry {:?}", entry);
        let p = init.with_entry(entry);
        visit_instr(&p, ext_addr, |p| {
            if test(p) {
                println!("===================\n{}===================\n", p);
            }
        })
    }
}

pub fn visit_instr<F>(p: &State, ext_addr: &[Addr], test: F)
where
    F: Fn(&State),
{
    assert!(p.instr.len() > 0);
    let mut p = p.clone();
    visit_instr_rec(0, (p.instr.len() - 1) as u8, &mut p, ext_addr, &test)
}

fn visit_instr_rec<F>(instr_addr: u8, max_instr: u8, p: &mut State, ext_addr: &[Addr], f: &F)
where
    F: Fn(&State),
{
    let jmp_targets = ((instr_addr + 1)..=max_instr).chain(ext_addr.iter().copied());

    //let min_mem_addr = u8::max(0, instr_addr - 1); // gear
    //let mem = [0, p.mem[max(0, instr_addr)]]

    let mut mem_addr = [instr_addr, 255];
    let mem_addr = if instr_addr == 0 {
        &mem_addr[..1]
    } else {
        mem_addr[1] = p.instr[(instr_addr - 1) as usize].mem;
        &mem_addr
    };

    for mem in [instr_addr] {
        for jmp0 in jmp_targets.clone() {
            for jmp1 in jmp_targets.clone() {
                p.instr[instr_addr as usize] = ijmp(mem, jmp0, jmp1);
                if instr_addr == max_instr {
                    f(p);
                } else {
                    visit_instr_rec(instr_addr + 1, max_instr, p, ext_addr, f)
                }
            }
        }
    }
}

pub fn solve_bits<F>(init: &State, ext_addr: &[Addr], test: F)
where
    F: Fn(&State) -> bool,
{
    let count = Counter::new();
    for entry in entrypoints(init.instr.len() as u8, ext_addr) {
        let p = init.with_entry(entry);
        visit_jmp01(&p, ext_addr, |p| {
            count.inc();
            if test(p) {
                println!(
                    "===================\nsolution{}:\n{}===================\n",
                    count, p
                );
            }
        })
    }
    println!("{} candidates tried", count);
}

//fn print_if_pass()

fn entrypoints(n_instr: u8, ext_addr: &[Addr]) -> Vec<[Addr; 2]> {
    let entry_b = (0..n_instr).into_iter().chain(ext_addr.iter().copied());
    let entry_r = (0..n_instr).into_iter().chain(ext_addr.iter().copied());

    let mut result = vec![];
    for b in entry_b {
        for r in entry_r.clone() {
            if n_instr == 0 || b == 0 || r == 0 {
                result.push([b, r])
            }
        }
    }
    result
}

/// Visit variations on the instructions'  jmp0, jmp1 arguments,
/// but don't vary their memory_address argument.
fn visit_jmp01<F>(p: &State, ext_addr: &[Addr], test: F)
where
    F: Fn(&State),
{
    assert!(p.instr.len() > 0);
    let mut p = p.clone();
    visit_jmp01_rec(0, (p.instr.len() - 1) as u8, &mut p, ext_addr, &test)
}

fn visit_jmp01_rec<F>(instr_addr: u8, max_instr: u8, p: &mut State, ext_addr: &[Addr], f: &F)
where
    F: Fn(&State),
{
    let jmp_targets = ((instr_addr + 1)..=max_instr).chain(ext_addr.iter().copied());

    for jmp0 in jmp_targets.clone() {
        for jmp1 in jmp_targets.clone() {
            p.instr[instr_addr as usize] = ijmp(instr_addr, jmp0, jmp1);
            if instr_addr == max_instr {
                f(p);
            } else {
                visit_jmp01_rec(instr_addr + 1, max_instr, p, ext_addr, f)
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn visit_jmp1() {
        let mut p = State::new(1);

        let ext_addr = [BLUE_LEVER, RED_LEVER, INTERC0];
        let count = Counter::new();
        visit_jmp01(&mut p, &ext_addr, |_| count.inc());
        // for (i, p) in collect.iter().enumerate(){
        //     println!("{}: {:?}", i, p.instr);
        // }
        assert_eq!(count.get(), 9); // 3x3 ext_addr, no next instr
    }

    #[test]
    fn visit_jmp2() {
        let mut p = State::new(2);

        let ext_addr = [BLUE_LEVER];
        let count = Counter::new();
        visit_jmp01(&mut p, &ext_addr, |_| count.inc());
        // for (i, p) in collect.iter().enumerate(){
        //     println!("{}: {:?}", i, p.instr);
        // }
        assert_eq!(count.get(), 4); // instr0: 11,1B,B1,BB, instr1: BB
    }
    #[test]
    fn visit_jmp3() {
        let mut p = State::new(3);

        let ext_addr = [BLUE_LEVER];
        let count = Counter::new();
        visit_jmp01(&mut p, &ext_addr, |_| count.inc());
        //for (i, p) in collect.iter().enumerate() {
        //    println!("{}: {:?}", i, p.instr);
        //}
        assert_eq!(count.get(), 36); // (3!)^2
    }

    #[test]
    fn visit_instr2() {
        use std::cell::RefCell;

        let mut p = State::new(2);

        let ext_addr = [BLUE_LEVER];
        //let count = Counter::new();
        let collect: RefCell<Vec<State>> = RefCell::new(vec![]);
        //visit_instr(&mut p, &ext_addr, |_| count.inc());
        visit_instr(&mut p, &ext_addr, |p| collect.borrow_mut().push(p.clone()));
        let collect = collect.take();
        for (i, p) in collect.iter().enumerate() {
            println!("{}: {:?}", i, p.instr);
        }
        //assert_eq!(count.get(), 4); // instr0: 11,1B,B1,BB, instr1: BB
    }

    //#[test]
    //fn test_entrypoints() {
    //    let B = BLUE_LEVER;
    //    let R = RED_LEVER;
    //    dbg!(entrypoints(0, &[B]));
    //    dbg!(entrypoints(0, &[B, R]));
    //    dbg!(entrypoints(0, &[B, R, INTERC0]));
    //    dbg!(entrypoints(1, &[B]));
    //    dbg!(entrypoints(1, &[B, R]));
    //    dbg!(entrypoints(1, &[B, R, INTERC0]));
    //    dbg!(entrypoints(2, &[B]));
    //    dbg!(entrypoints(3, &[B]));
    //    dbg!(entrypoints(4, &[]));
    //}
}
