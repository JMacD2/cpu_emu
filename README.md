# cpu_emu

A 64-bit CPU simulator written in Rust. The simulator follows a single-cycle model, with the exception of a one cycle penalty for cache misses on memory reads.

Information on the simulator can be found in comments throughout, and in the Design Document

One can write assembly instructions in the input_instr.txt file, and data to be loaded into memory in input_data.txt
