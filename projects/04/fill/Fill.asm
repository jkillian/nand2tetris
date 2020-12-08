// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// keep track of what pixel we're current filling in / clearing
@screenpx
M=-1

(LOOP)
  @screenpx
  M=M+1

  // if we reach the end of the screen, reset our counter back to 0
  @8192
  D=A
  @screenpx
  D=D-M

  @SKIP_RESET
  D;JGT

  @screenpx
  M=0

  (SKIP_RESET)

  @KBD
  D=M

  @KEYPRESSED
  D;JGT

  @NOKEY
  D;JEQ

(KEYPRESSED)
  @screenpx
  D=M

  @SCREEN
  A=A+D

  // set to all black
  M=-1

  @LOOP
  0;JMP

(NOKEY)
  @screenpx
  D=M

  @SCREEN
  A=A+D

  // set to all white
  M=0

  @LOOP
  0;JMP