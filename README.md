
# Table of Contents

1.  [Description](#org19a9505)
2.  [Prerequisites](#orga8e0000)
3.  [Features](#orgb590839)
4.  [Configuration](#org1cddeca)



<a id="org19a9505"></a>

# Description

This is a 8080 Emulator built on Rust, that intends to emulate the 1978 Space
Invaders Arcade game, most of the logic was taken from the 8080 Emulator built
on C in the tutorial emulator101.com. The plan is to after the project is in a
usable state to convert into WASM and host in a heroku instance.


<a id="orga8e0000"></a>

# Prerequisites

1. Cargo and Rust
2. The sdl library installed, you can get further instructions in the [Project website](http://www.libsdl.org/)
3. You will need to get your own ROM, since i cannot provide for legal reasons, and put it in the root of the project.


<a id="orgb590839"></a>

# Features

1.  A Dissassembler that takes a vec with byte codes and convert at least the
    first index into 8080 Instructions
2.  The Emulator in itself, with a CPU Struct intended to emulate the 8080 processor and the machine around emulated by the invaders.rs file


<a id="org1cddeca"></a>

# Running

Just execute `cargo run`

