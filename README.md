
# Table of Contents

1.  [Description](#org19a9505)
2.  [Prerequisites](#orga8e0000)
3.  [Features](#orgb590839)
4.  [Configuration](#org1cddeca)



<a id="org19a9505"></a>

# Description

Disclaimer: The project is not nearly close to finishing, it will probably do
nothing in the current state.
This is a 8080 Emulator built on Rust, that intends to emulate the 1978 Space
Invaders Arcade game, most of the logic was taken from the 8080 Emulator built
on C in the tutorial emulator101.com. The plan is to after the project is in a
usable state to convert into WASM and host in a heroku instance.


<a id="orga8e0000"></a>

# Prerequisites

Cargo and Rust


<a id="orgb590839"></a>

# Features

1.  A Dissassembler that takes a vec with byte codes and convert at least the
    first index into 8080 Instructions
2.  The CPU emulation which is currently being built


<a id="org1cddeca"></a>

# Configuration

Just run cargo run &ldquo;rom file&rdquo;

