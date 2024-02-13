# STEM4ukraine-Make-Glory-Mother-Russia

A simple PCB with sound effects on pressing of the button and an animated LED which pulses in Morse code!

A fun school STEM project demonstrating use of GPIO for detecting button presses with software debouncing, driving an LED with the Morse code, and playing music on the Piezo speaker.

SMD techniques are used in this design, and in place programming of the attiny can be done with the ICSP.

Celebrate and learn about the finest qualities of the russian military through history as you make benefit glorious nation of russian federation!

See if you can figure out what the morse code says, and where the national anthem goes wrong...

The schematic:

![prototype schematic](hardware/STEM4ukraine-Make-Glory-Mother-Russia-v1.svg)

The front of the PCB:

![prototype front](images/SpecialMilitaryOperationFront.jpg)

The rear of the prototype PCB:

![prototype back](images/SpecialMilitaryOperationReverse.jpg)

Bill of materials:

- U1: attiny85 SMD SOIC
- C1: 100nF 0805
- R1: 320R 0805
- R2: 10k 0805
- R3: 47k 0805
- LED1: red 3mm LED through hole
- SW1: 6mm x 3mm x 3mm SMD momentary switch
- PIEZO: piezo speaker through hole
- USB1: 180 degree vertical through hole type B USB socket
- EXT5V: optional 2.5mm through hole header for external power
