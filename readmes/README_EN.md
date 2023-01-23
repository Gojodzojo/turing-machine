<div align="center">
<img src="https://raw.githubusercontent.com/Gojodzojo/turing-machine/main/icon.ico" alt="icon" width="140px" />

# Turing Machine
</div>

This is a program that simulates the operation of a [Turing machine](https://en.wikipedia.org/wiki/Turing_machine). This program was made as a final project for programming classes at the Silesian University of Technology.

## Operation of a Turing machine
A Turing machine consists of four parts:
* tape,
* table of rules,
* head (cursor),
* internal state of the machine.

During the operation of the machine, the head first reads the character over which it is currently located. Then it checks its internal state (it is always 0 at the beginning) and, based on the read character and machine's state, selects a cell from the table of rules.

Each such cell contains 3 pieces of information:
* new state of the machine,
* new character on the tape,
* head movement direction.

Based on the information from the cell, a new character is entered in place of the previously read character, then the machine changes its state to the new one, and finally the head moves to the right or left.

All of the above actions are repeated in a loop until one of the following occurs:
* in the table of rules there is no cell defined by the character and state,
* in the cell, 0 was given as the direction of movement,
* movement of the head would require going beyond the tape.

## User Interface
After opening the program, two parts are visible: the left column with the icon and the right column with the simulator.

### Left column
The column on the left contains buttons that allow you to:
* create a new file,
* open a previously saved file,
* save the file,
* save the file as a new file,

The column on the left also allows you to customize settings such as:
* application language,
* app theme.

The column on the left can be opened or closed using the button at the top right of the column line.

### Simulator
In edit mode (default) on the left, you can adjust settings such as:
* tape text,
* tape length,
* position of the cursor (head),
* number of table states,
* tape characters.

On the right side there is a table in which you can enter the values ​​of individual cells. These values ​​are set in the following order:
* new state of the machine (from 0 to 99),
* new character on the tape,
* head movement direction (`+`, `-` or `0`).

At the top is a preview of the initial tape characters.

After setting all the parameters, you can go to the simulation mode by clicking the Start button.

On the left are:
* information about the number of steps taken,
* information about the internal state of the machine,
* slider to change the machine's self-timer interval,
* button for manually changing steps,
* Stop button to return to edit mode.

On the right side there is a table in which the previously set cell values ​​are displayed.

At the top is a preview of the current state of the tape.

## Keyboard shortcuts
* `tab` = Switch the text input,
* `ctrl` + `s` = Save the file,
* `ctrl` + `+` = Zoom in,
* `ctrl` + `-` = Zoom out.

## Examples
[Here](https://github.com/Gojodzojo/turing-machine/tree/main/examples) are files with sample Turing machine tables of rules. Some of them were bundled with another Turing machine simulator.

## Compiling
To compile this program, install the Rust compiler according to [this manual](https://www.rust-lang.org/tools/install). Then, clone this repository, enter it in a terminal and execute the command:
```
cargo run --release
```
The compiled program will be in the `target/release` directory.

## Fun fact
The icon of this program was created by the generator [DALL·E 2](https://openai.com/dall-e-2/).