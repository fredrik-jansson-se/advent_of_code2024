use std::{cell::RefCell, collections::HashMap};

use anyhow::anyhow;
use inkwell::{
    targets::{InitializationConfig, Target, TargetMachine},
    OptimizationLevel,
};
use nom::Parser;

use crate::{Input, PResult};

thread_local! {
    static OUT: RefCell<Vec<i64>> = const { RefCell::new(Vec::new())};

    static INIT_A: RefCell<i64> = const {RefCell::new(0)};
    static OUT2: RefCell<Vec<i64>> = const {RefCell::new(Vec::new())};
    static OUT_ANS: RefCell<Vec<i64>> = const {RefCell::new(Vec::new())};
}

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day17.txt")?;

    println!("17:1 - {}", run_1(&input)?);
    println!("17:2 - {}", run_2(&input)?);

    Ok(())
}

#[derive(Debug)]
struct Program {
    a: i64,
    b: i64,
    c: i64,
    program: Vec<u8>,
}

fn parse(i: Input) -> PResult<Program> {
    use nom::bytes::complete::tag;
    use nom::character::complete::newline;
    let (i, _) = tag("Register A: ")(i)?;
    let (i, a) = nom::character::complete::i64(i)?;
    let (i, _) = newline(i)?;
    let (i, _) = tag("Register B: ")(i)?;
    let (i, b) = nom::character::complete::i64(i)?;
    let (i, _) = newline(i)?;
    let (i, _) = tag("Register C: ")(i)?;
    let (i, c) = nom::character::complete::i64(i)?;
    let (i, _) = newline(i)?;
    let (i, _) = newline(i)?;
    let (i, _) = tag("Program: ")(i)?;
    let (i, program) = nom::multi::separated_list1(tag(","), nom::character::complete::u8).parse(i)?;
    Ok((i, Program { a, b, c, program }))
}

#[unsafe(no_mangle)]
extern "C" fn append(c: i64) -> i64 {
    let c = c % 8;
    OUT.with_borrow_mut(|rc| rc.push(c));
    0
}

#[used]
static EXTERNAL_FNS: [extern "C" fn(i64) -> i64; 3] = [append, append2, init_a];

struct Ptrs<'a> {
    a: inkwell::values::PointerValue<'a>,
    b: inkwell::values::PointerValue<'a>,
    c: inkwell::values::PointerValue<'a>,
}

fn read_operand<'a>(
    ctx: &'a inkwell::context::Context,
    builder: &inkwell::builder::Builder<'a>,
    ptrs: &Ptrs<'a>,
    operand: u8,
) -> anyhow::Result<inkwell::values::IntValue<'a>> {
    match operand {
        op if op <= 3 => Ok(ctx.i64_type().const_int((op % 8) as _, false)),
        4 => Ok(builder
            .build_load(ctx.i64_type(), ptrs.a, "load a")?
            .into_int_value()),
        5 => Ok(builder
            .build_load(ctx.i64_type(), ptrs.b, "load b")?
            .into_int_value()),
        6 => Ok(builder
            .build_load(ctx.i64_type(), ptrs.c, "load c")?
            .into_int_value()),
        _ => unreachable!(),
    }
}

fn execute(program: Program) -> anyhow::Result<Vec<i64>> {
    let ctx = inkwell::context::Context::create();
    let builder = ctx.create_builder();
    let module = ctx.create_module("aoc");

    // Init variables
    let a = ctx.i64_type().const_int(program.a as _, false);
    let b = ctx.i64_type().const_int(program.b as _, false);
    let c = ctx.i64_type().const_int(program.c as _, false);

    let append_fn_args: Vec<inkwell::types::BasicMetadataTypeEnum> = vec![ctx.i64_type().into()];
    let append_fn_type = ctx.i64_type().fn_type(&append_fn_args, false);
    let append_fn = module.add_function("append", append_fn_type, None);

    let main_fn_type = ctx.void_type().fn_type(&[], false);
    let main_fn = module.add_function("tmp", main_fn_type, None);

    let main_entry = ctx.append_basic_block(main_fn, "entry");

    builder.position_at_end(main_entry);

    let aptr = builder.build_alloca(ctx.i64_type(), "a")?;
    let bptr = builder.build_alloca(ctx.i64_type(), "b")?;
    let cptr = builder.build_alloca(ctx.i64_type(), "c")?;
    let ptrs = Ptrs {
        a: aptr,
        b: bptr,
        c: cptr,
    };

    builder.build_store(aptr, a)?;
    builder.build_store(bptr, b)?;
    builder.build_store(cptr, c)?;

    let blocks: HashMap<i64, _> = (0..program.program.len() / 2)
        .map(|pc| {
            (
                (pc * 2) as i64,
                ctx.append_basic_block(main_fn, &format!("blk-{}", pc * 2)),
            )
        })
        .collect();

    builder.build_unconditional_branch(blocks[&0])?;
    let end_entry = ctx.append_basic_block(main_fn, "end");

    let mut pc = 0;
    for op_op in program.program.chunks(2) {
        let op = op_op[0];
        let operand = op_op[1];
        let entry = blocks[&pc];
        builder.position_at_end(entry);
        match op {
            // adv
            0 => {
                let lhs = builder
                    .build_load(ctx.i64_type(), aptr, "a")?
                    .into_int_value();
                let rhs = read_operand(&ctx, &builder, &ptrs, operand)?;
                let rhs =
                    builder.build_left_shift(ctx.i64_type().const_int(1, false), rhs, "2pow")?;
                let div = builder.build_int_signed_div(lhs, rhs, "div")?;
                builder.build_store(aptr, div)?;
                builder.build_unconditional_branch(blocks[&(pc + 2)])?;
            }
            // bxl
            1 => {
                let lhs = builder.build_load(ctx.i64_type(), bptr, "b")?;
                //let rhs = read_operand(&ctx, &builder, &ptrs, operand)?;
                let rhs = ctx.i64_type().const_int(operand as _, false);
                let xor = builder.build_xor(lhs.into_int_value(), rhs, "xor")?;
                builder.build_store(bptr, xor)?;
                builder.build_unconditional_branch(blocks[&(pc + 2)])?;
            }
            // bst
            2 => {
                let val = read_operand(&ctx, &builder, &ptrs, operand)?;
                let val = builder.build_int_signed_rem(
                    val,
                    ctx.i64_type().const_int(8, false),
                    "mod-8",
                )?;
                builder.build_store(bptr, val)?;
                builder.build_unconditional_branch(blocks[&(pc + 2)])?;
            }
            // jnz
            3 => {
                let a_val = builder
                    .build_load(ctx.i64_type(), aptr, "a")?
                    .into_int_value();
                let new_pc = (operand % 8) as i64;
                let cmp = builder.build_int_compare(
                    inkwell::IntPredicate::EQ,
                    a_val,
                    ctx.i64_type().const_zero(),
                    "is a zero",
                )?;
                builder.build_conditional_branch(cmp, end_entry, blocks[&new_pc])?;
            }
            // bxc
            4 => {
                let lhs = builder.build_load(ctx.i64_type(), bptr, "b")?;
                let rhs = builder.build_load(ctx.i64_type(), cptr, "c")?;
                let xor = builder.build_xor(lhs.into_int_value(), rhs.into_int_value(), "xor")?;
                builder.build_store(bptr, xor)?;
                builder.build_unconditional_branch(blocks[&(pc + 2)])?;
            }
            // out
            5 => {
                let arg = read_operand(&ctx, &builder, &ptrs, operand)?;
                builder.build_call(append_fn, &[arg.into()], "append")?;
                builder.build_unconditional_branch(blocks[&(pc + 2)])?;
            }
            6 => {
                let lhs = builder
                    .build_load(ctx.i64_type(), aptr, "a")?
                    .into_int_value();
                let rhs = read_operand(&ctx, &builder, &ptrs, operand)?;
                let rhs =
                    builder.build_left_shift(ctx.i64_type().const_int(1, false), rhs, "2pow")?;
                let div = builder.build_int_signed_div(lhs, rhs, "div")?;
                builder.build_store(bptr, div)?;
                builder.build_unconditional_branch(blocks[&(pc + 2)])?;
            }
            //cdv
            7 => {
                let lhs = builder
                    .build_load(ctx.i64_type(), aptr, "a")?
                    .into_int_value();
                let rhs = read_operand(&ctx, &builder, &ptrs, operand)?;
                let rhs =
                    builder.build_left_shift(ctx.i64_type().const_int(1, false), rhs, "2pow")?;
                let div = builder.build_int_signed_div(lhs, rhs, "div")?;
                builder.build_store(cptr, div)?;
                builder.build_unconditional_branch(blocks[&(pc + 2)])?;
            }
            o => {
                println!("{o} - not yet");
                panic!();
            }
        }
        pc += 2;
    }

    builder.position_at_end(end_entry);
    builder.build_return(None)?;

    if !main_fn.verify(true) {
        eprintln!("Bad fn");
        return Ok(Vec::new());
    }

    let ee = module
        .create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .map_err(|e| anyhow!("{e}"))?;

    let comp_fn = unsafe { ee.get_function::<unsafe extern "C" fn()>("tmp")? };

    unsafe {
        comp_fn.call();
    }

    let mut res = Vec::new();
    OUT.with_borrow(|rc| res = rc.clone());

    Ok(res)
}

fn run_1(input: &str) -> anyhow::Result<String> {
    let (_i, p) = parse(input).map_err(|e| anyhow!("{e}"))?;
    execute(p)?;
    let mut res = Vec::new();
    OUT.with_borrow(|rc| res = rc.clone());

    Ok(res
        .into_iter()
        .map(|r| r.to_string())
        .collect::<Vec<_>>()
        .join(","))
}

#[unsafe(no_mangle)]
extern "C" fn init_a(_c: i64) -> i64 {
    let mut a = 0;
    INIT_A.with_borrow(|v| a = *v);
    a
}
#[unsafe(no_mangle)]
extern "C" fn append2(c: i64) -> i64 {
    let eq = OUT2.with_borrow_mut(|o2| {
        let l = o2.len();
        o2.push(c);
        OUT_ANS.with_borrow(|ans| l < ans.len() && ans[l] == c)
    });
    if eq {
        1
    } else {
        0
    }
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let (_i, program) = parse(input).map_err(|e| anyhow!("{e}"))?;

    let ctx = inkwell::context::Context::create();
    let builder = ctx.create_builder();
    let module = ctx.create_module("aoc");

    // Init variables
    let b = ctx.i64_type().const_int(program.b as _, false);
    let c = ctx.i64_type().const_int(program.c as _, false);

    let append_fn_args: Vec<inkwell::types::BasicMetadataTypeEnum> = vec![ctx.i64_type().into()];
    let append_fn_type = ctx.i64_type().fn_type(&append_fn_args, false);
    let append_fn = module.add_function("append2", append_fn_type, None);

    let init_a_fn_args: Vec<inkwell::types::BasicMetadataTypeEnum> = vec![ctx.i64_type().into()];
    let init_a_fn_type = ctx.i64_type().fn_type(&init_a_fn_args, false);
    let init_a_fn = module.add_function("init_a", init_a_fn_type, None);

    let main_fn_type = ctx.void_type().fn_type(&[], false);
    let main_fn = module.add_function("tmp", main_fn_type, None);

    let main_entry = ctx.append_basic_block(main_fn, "entry");

    builder.position_at_end(main_entry);

    // call init_a
    let a = builder
        .build_call(
            init_a_fn,
            &[ctx.i64_type().const_zero().into()],
            "init_a",
        )?
        .try_as_basic_value()
        .left()
        .unwrap();

    let aptr = builder.build_alloca(ctx.i64_type(), "a")?;
    let bptr = builder.build_alloca(ctx.i64_type(), "b")?;
    let cptr = builder.build_alloca(ctx.i64_type(), "c")?;
    let ptrs = Ptrs {
        a: aptr,
        b: bptr,
        c: cptr,
    };

    builder.build_store(aptr, a)?;
    builder.build_store(bptr, b)?;
    builder.build_store(cptr, c)?;

    let blocks: HashMap<i64, _> = (0..program.program.len() / 2)
        .map(|pc| {
            (
                (pc * 2) as i64,
                ctx.append_basic_block(main_fn, &format!("blk-{}", pc * 2)),
            )
        })
        .collect();

    builder.build_unconditional_branch(blocks[&0])?;
    let end_entry = ctx.append_basic_block(main_fn, "end");

    let mut pc = 0;
    for op_op in program.program.chunks(2) {
        let op = op_op[0];
        let operand = op_op[1];
        let entry = blocks[&pc];
        builder.position_at_end(entry);
        match op {
            // adv
            0 => {
                let lhs = builder
                    .build_load(ctx.i64_type(), aptr, "a")?
                    .into_int_value();
                let rhs = read_operand(&ctx, &builder, &ptrs, operand)?;
                let rhs =
                    builder.build_left_shift(ctx.i64_type().const_int(1, false), rhs, "2pow")?;
                let div = builder.build_int_signed_div(lhs, rhs, "div")?;
                builder.build_store(aptr, div)?;
                builder.build_unconditional_branch(blocks[&(pc + 2)])?;
            }
            // bxl
            1 => {
                let lhs = builder.build_load(ctx.i64_type(), bptr, "b")?;
                //let rhs = read_operand(&ctx, &builder, &ptrs, operand)?;
                let rhs = ctx.i64_type().const_int(operand as _, false);
                let xor = builder.build_xor(lhs.into_int_value(), rhs, "xor")?;
                builder.build_store(bptr, xor)?;
                builder.build_unconditional_branch(blocks[&(pc + 2)])?;
            }
            // bst
            2 => {
                let val = read_operand(&ctx, &builder, &ptrs, operand)?;
                let val = builder.build_int_signed_rem(
                    val,
                    ctx.i64_type().const_int(8, false),
                    "mod-8",
                )?;
                builder.build_store(bptr, val)?;
                builder.build_unconditional_branch(blocks[&(pc + 2)])?;
            }
            // jnz
            3 => {
                let a_val = builder
                    .build_load(ctx.i64_type(), aptr, "a")?
                    .into_int_value();
                let new_pc = (operand % 8) as i64;
                let cmp = builder.build_int_compare(
                    inkwell::IntPredicate::EQ,
                    a_val,
                    ctx.i64_type().const_zero(),
                    "is a zero",
                )?;
                builder.build_conditional_branch(cmp, end_entry, blocks[&new_pc])?;
            }
            // bxc
            4 => {
                let lhs = builder.build_load(ctx.i64_type(), bptr, "b")?;
                let rhs = builder.build_load(ctx.i64_type(), cptr, "c")?;
                let xor = builder.build_xor(lhs.into_int_value(), rhs.into_int_value(), "xor")?;
                builder.build_store(bptr, xor)?;
                builder.build_unconditional_branch(blocks[&(pc + 2)])?;
            }
            // out
            5 => {
                let arg = read_operand(&ctx, &builder, &ptrs, operand)?;
                let arg =
                    builder.build_int_signed_rem(arg, ctx.i64_type().const_int(8, false), "mod")?;
                let res = builder
                    .build_call(append_fn, &[arg.into()], "append")?
                    .try_as_basic_value()
                    .left()
                    .unwrap();
                //builder.build_unconditional_branch(blocks[&(pc + 2)])?;
                let cmp = builder.build_int_compare(
                    inkwell::IntPredicate::EQ,
                    res.into_int_value(),
                    ctx.i64_type().const_zero(),
                    "is a zero",
                )?;
                builder.build_conditional_branch(cmp, end_entry, blocks[&(pc + 2)])?;
            }
            6 => {
                let lhs = builder
                    .build_load(ctx.i64_type(), aptr, "a")?
                    .into_int_value();
                let rhs = read_operand(&ctx, &builder, &ptrs, operand)?;
                let rhs =
                    builder.build_left_shift(ctx.i64_type().const_int(1, false), rhs, "2pow")?;
                let div = builder.build_int_signed_div(lhs, rhs, "div")?;
                builder.build_store(bptr, div)?;
                builder.build_unconditional_branch(blocks[&(pc + 2)])?;
            }
            //cdv
            7 => {
                let lhs = builder
                    .build_load(ctx.i64_type(), aptr, "a")?
                    .into_int_value();
                let rhs = read_operand(&ctx, &builder, &ptrs, operand)?;
                let rhs =
                    builder.build_left_shift(ctx.i64_type().const_int(1, false), rhs, "2pow")?;
                let div = builder.build_int_signed_div(lhs, rhs, "div")?;
                builder.build_store(cptr, div)?;
                builder.build_unconditional_branch(blocks[&(pc + 2)])?;
            }
            o => {
                println!("{o} - not yet");
                panic!();
            }
        }
        pc += 2;
    }

    builder.position_at_end(end_entry);
    builder.build_return(None)?;

    //println!("{}", module.to_string());
    if !main_fn.verify(true) {
        return Err(anyhow::anyhow!("bad function"));
    }

    let ee = module
        .create_jit_execution_engine(inkwell::OptimizationLevel::Default)
        .map_err(|e| anyhow!("{e}"))?;

    Target::initialize_all(&InitializationConfig::default());
    let target_triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&target_triple).unwrap();
    let target_machine = target
        .create_target_machine(
            &target_triple,
            "generic",
            "",
            OptimizationLevel::None,
            inkwell::targets::RelocMode::PIC,
            inkwell::targets::CodeModel::Default,
        )
        .unwrap();

    let passes: &[&str] = &[
        "instcombine",
        "reassociate",
        "gvn",
        "simplifycfg",
        // "basic-aa",
        "mem2reg",
    ];

    module
        .run_passes(
            passes.join(",").as_str(),
            &target_machine,
            inkwell::passes::PassBuilderOptions::create(),
        )
        .unwrap();

    println!("{}", module.to_string());

    let comp_fn = unsafe { ee.get_function::<unsafe extern "C" fn()>("tmp")? };

    OUT_ANS.with_borrow_mut(|ans| *ans = program.program.iter().map(|p| *p as i64).collect());
    for a in 0..200_000_000_000 {
        OUT2.with_borrow_mut(|o| o.clear());
        INIT_A.with_borrow_mut(|ia| *ia = a);
        unsafe {
            comp_fn.call();
        }
        let eq = OUT_ANS.with_borrow(|ans| OUT2.with_borrow(|out| *ans == *out));
        if eq {
            return Ok(a as _);
        }
    }

    Err(anyhow!("No answer"))
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const INPUT_2: &str = "Register A: 2048
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn day17_run_1() {
        let (_, p) = super::parse(INPUT).unwrap();
        assert_eq!(
            super::execute(p).unwrap(),
            vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]
        );
    }

    #[test]
    fn day17_run_2() {
        assert_eq!(super::run_2(INPUT_2).unwrap(), 117440);
    }
}
