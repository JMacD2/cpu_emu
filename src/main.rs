#![feature(anonymous_pipe)]

mod assembler;
mod converter;
mod control_unit;
mod reg64;
mod reg_bank;
mod clock;
mod memory_chips;
mod main_memory;
mod alu;
mod adders;
mod transistors;
mod logic_gates;
mod buses;
mod multiplier;
mod caches;
mod bitwise_operator;

use std::io::*;
use std::fs::File;
use std::pipe::pipe;
use regex::Regex;
use crate::assembler::assembler::{Assembler as assembler_struct, ParsedInstruction};
use crate::control_unit::control_unit::ControlUnit as control_unit_struct;
use crate::control_unit::control_unit::CpuState;
use crate::reg64::reg64::Reg64 as reg64_struct;
use crate::reg_bank::reg_bank::RegBank as reg_bank_struct;
use crate::main_memory::main_memory::MainMemory;
use crate::alu::alu::Alu as alu_struct;
use crate::clock::clock::Clock as clock_struct;
use crate::converter::converter::{Converter as converter_struct, Converter};
use crate::buses::buses::{AddressBus, ControlBus, DataBus};
use crate::caches::caches::{DataAccessManager, L1Cache, L2Cache };

fn check_comment(str : String) -> bool {
    if (str[0..2] == "//".to_string()) || str.len() == 0 || str == "".to_string() {
        return true;
    }
    false
}

fn vec_to_str(mut val : Vec<bool>) -> String {
    val.reverse();
    let mut return_str : String = "".to_string();
    for i in val {
        if i{
            return_str.push_str("1");
        }
        else{
            return_str.push_str("0");
        }
    }
    return_str
}

fn main() {
    println!(" ----- START -----");
    load_memory();
}

fn load_memory() {

    println!(" ----- DATA LOAD START -----");

    let mut memory: MainMemory = MainMemory{
        ram_map: Default::default(),
        data_bus: DataBus::default(),
        address_bus: AddressBus::default(),
        control_bus: ControlBus::default()
    };
    memory.clear();

    for line in BufReader::new(File::open("./input_data.txt").expect("Input File Error")).lines() {
        let str: String = line.unwrap();

        if check_comment(str.clone()) { continue; }
        let mut split_str = str.split("/");
        let mut addr_hex = split_str.clone().nth(0).unwrap().to_string();

        if !Regex::new(r"^0x[0123456789ABCDEF]{8}$").unwrap().is_match(&*addr_hex.clone()) { continue; }
        let addr_bits : [bool; 48] = Converter::hex_val_to_bin(addr_hex[2..addr_hex.len()].parse().unwrap()).try_into().unwrap();

        let data_str = String::from(&split_str.clone().nth(1).unwrap()[2..split_str.clone().nth(1).unwrap().len()]);
        let datatype = split_str.nth(1).unwrap().chars().nth(1).unwrap();
        let mut new_data = [false; 64];

        match datatype {
            'x' => {
                if !Regex::new(r"^[0123456789ABCDEF]+$").unwrap().is_match(&*data_str.clone()) {
                    continue;
                }
                let mut data_vec = Converter::hex_val_to_bin(data_str);
                while data_vec.len() < 64 {
                    data_vec.push(data_vec[data_vec.len() - 1]);
                }
                if data_vec.len() > 64 {
                    data_vec.truncate(64);
                }
                let mut data_bits = [false; 64];
                data_bits[0..64].clone_from_slice(&data_vec);
                new_data[0..64].copy_from_slice(&data_bits);
            }
            'd' => {
                if !Regex::new(r"^-?[0123456789]+$").unwrap().is_match(&*data_str.clone()) {
                    continue;
                }
                new_data = converter_struct::dec_to_bin_2s_comp(data_str.parse().expect("Invalid Decimal Number")).try_into().unwrap();
            }
            'b' => {
                if !Regex::new(r"^[01]+$").unwrap().is_match(&*data_str.clone()) {
                    continue;
                }
                let mut binary_vector: Vec<bool> = Vec::new();
                for i in 0..data_str.len() {
                    match data_str.chars().nth(i).unwrap(){
                        '0' => { binary_vector.push(false); }
                        '1' => { binary_vector.push(true); }
                        _ => { continue; }
                    }
                }
                binary_vector.reverse();
                new_data = converter_struct::set_size(binary_vector, 64).try_into().unwrap();
            }
            _ => { continue; }
        }

        // STORE AT ADDR IN RAM
        memory.write(addr_bits, new_data);
    };

    println!(" ----- DATA LOAD COMPLETE -----");

    let mut assembler: assembler_struct = assembler_struct {};
    let mut line_counter: u32 = 0;
    for mut line in BufReader::new(File::open("./input_instr.txt").expect("Instr File Error")).lines() {
        let line_string= line.unwrap();
        if line_string.clone().len() == 15 { continue; }
        let mut offset = 15;
        while 1==1 {
            if line_string.clone().chars().nth(offset).unwrap() != ' ' {
                break;
            }
            else{
                offset += 1;
            }
        }
        let line_unwrap= line_string.clone()[offset..line_string.len()].to_string();
        if check_comment(line_unwrap.clone()) { continue; }
        let assembled: [bool; 64] = assembler.assemble(line_unwrap);
        let new : [bool; 48] = Converter::dec_to_bin_pos_only(line_counter as u64, 48).try_into().unwrap();
        memory.write(new, assembled);
        line_counter += 64;
    };

    println!(" ----- INSTRUCTION LOAD COMPLETE -----");

    let mut cpu_cu: control_unit_struct = control_unit_struct {
        alu: alu_struct::default(), memory_instr_reg: reg64_struct::default(),
        memory_instr_stall: false, memory_data_reg: reg64_struct::default(), memory_data_stall: false,
        pc: reg64_struct::default(), register_bank: reg_bank_struct::default(), halt: false,
        state: CpuState::Fetch, decoded_instruction: ParsedInstruction::default(),
        data_access_manager: DataAccessManager{
            l1_cache: L1Cache::default(), l2_cache: L2Cache::default(), main_memory: memory
        }
    };

    let mut clk: clock_struct = clock_struct{
        clock_speed : 100,
        running : false,
        ctrl : cpu_cu,
        cycle_count : 0
    };
    clk.start();

    println!("----- END -----");
}

pub fn send_reg(reg : u8, val : String){
    send_pipe_data("REG//".to_string() + &reg.to_string() + "//" + &val);
}

pub fn send_state(state : CpuState){
    send_pipe_data("STATE//".to_string() + &(state as u8).to_string());
}

pub fn send_instr(current_instr: String, incr_instr: String){
    send_pipe_data("INSTR//".to_string() + &current_instr + "//" + &incr_instr);
}

pub fn send_memory(memory_addr : String, memory_data : String){
    send_pipe_data("RAM//".to_string() + &memory_addr + "//" + &memory_data);
}

pub fn send_l1_util(util : String){
    send_pipe_data("L1_UTIL//".to_string() + &util);
}

pub fn send_l2_util(util : String){
    send_pipe_data("L2_UTIL//".to_string() + &util);
}

pub fn send_pipe_data(val : String){
    
}

pub fn read_pipe() -> String{
    let data : String = "".to_string();
    data
}