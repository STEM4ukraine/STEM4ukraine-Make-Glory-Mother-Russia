/*
 *
 *   Copyright (C) 2024 STEM4ukraine
 *   Website https://github.com/STEM4ukraine
 *   
 *   This program is free software: you can redistribute it and/or modify
 *   it under the terms of the GNU General Public License as published by
 *   the Free Software Foundation, either version 3 of the License, or
 *   (at your option) any later version.
 *
 *   This program is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU General Public License at <http://www.gnu.org/licenses/> 
 *   for more details.
 *
 */

#include <avr/pgmspace.h>

// the code has been uploaded using the USB-ASP programmer 
// uses the https://github.com/SpenceKonde/ATTinyCore 
// set to "ATtiny25/45/85 (No bootloader)" and "Chip ATtiny85"
// "timer 1 clock" set to "CPU (CPU frequency)"
// uses the "USB-ASP slow (Microcore)" setting in the arduino IDE
// for attiny85 running at 4MHz internal clock

// define notes to index into array of durations = period/2 in uS
#define PAUSE 27
#define D6 26
#define C6SH 25
#define C6 24
#define B5 23
#define A5SH 22
#define A5 21
#define G5SH 20
#define G5 19
#define F5SH 18
#define F5 17
#define E5 16
#define D5SH 15
#define D5 14
#define C5SH 13
#define C5 12
#define B4 11
#define A4SH 10
#define A4 9
#define G4SH 8
#define G4 7
#define F4SH 6
#define F4 5
#define E4 4
#define D4SH 3
#define D4 2
#define C4SH 1
#define C4 0

// define lengths of notes as ((duration - 1) * 32) with fundamental unit semiquaver
// equivalent to/indexing into semiquaver durations 2,3,4,6,8,12,16,24
#define SQ 0
#define SQD 32
#define QU 64
#define QUD 96
#define CR 128
#define CRD 160
#define MI 192
#define MID 224

static const uint16_t note_periods[] PROGMEM = {3213,3034,2864,2703,2551,2407,2273,2145,2024,1911,1804,1702,1607,1517,1432,1351,1276,1204,1136,1073,1012,955,902,851,803,758,716,1000};

static const uint16_t durations[] PROGMEM = {2,3,4,6,8,12,16,24};

/* rate of 72 bpm */
static const uint8_t anthem15[] PROGMEM = {D5+QU, G5+CR, D5+QUD, E5+SQ, F5SH+CR, B4+QU, B4+QU, E5+CR, D5+QUD, C5+QU, PAUSE+QU, C5+QU, PAUSE+QU, C5+QU, PAUSE+QU};

/* rate of 250 bpm */
static const uint8_t yakkety[] PROGMEM = {C5+MID, F5+CR, F5+CR, D5+QU, C5+QU, G4SH+QU, A4+QU, C5+QU, A4+QU, D5+CR, C5+QU, A4+QU, G4+QU, E4+QU, F4+CR, F4+QU, G4+QU,
				A4+QU, C5+QU, D5+QU, C5+QU, F5+CR, PAUSE+CRD, C5+QU, D5+QU, C5+QU, F5+CR, F5+CR, D5+QU, C5+QU, G4SH+QU, A4+QU, C5+QU, A4+QU,
				D5+CR, C5+QU, A4+QU, G4+QU, A4+QU, 
				C5+CR, C5+QU, D5+QU, E5+QU, G5+QU, E5+CR, C5+CR, PAUSE+CRD, C5+QU, D5+QU, C5+QU, F5+CR, F5+CR, F5+CR, F5+CR, F5+CR, F5+CR,
				D5+QU, C5+QU, A4+QU, F4+QU, A4SH+CR, A4SH+CR, A4SH+CR, A4SH+CR, D5+CR, F5+QU, G5+QU, G5SH+QU, F5+CR, G5+QU, A5+QU, G5SH+QU,
				A5+QU, G5+QU, A5+QU, C6+CR, G5+QU,
				A5+QU, C6+QU, A5+CR, F5+CR, C5+CR, D6+QU, G5SH+QU, G5+QU, F5+QU, G5+CR, F5+CR};


#define NOTE_DELAY_MS 40 // between notes

#define DITMS 160
#define DAHMS DITMS*3

// for an attiny85 running at 4MHz internal clock
#define BPM250 20
#define BPM63 80
#define BPM72 69

// note period/2 (uS) and duration in semiquavers encoded as 10LSB and 6MSB respectively

uint8_t led = 0;
uint8_t button = 0;

void setup() {
  pinMode(0, OUTPUT);
  pinMode(1, OUTPUT);
  pinMode(2, OUTPUT);
  pinMode(3, OUTPUT);
  pinMode(4, OUTPUT);
  pinMode(5, INPUT);
}

byte poll_button() {
  button = button|(analogRead(A0) < 900);
  return button;
}

void led_on(int milli_dur) {
  PORTB ^= 0b00001000;
  delay(milli_dur);
  poll_button();
}

void led_off(int milli_dur) {
  PORTB ^= 0b00001000;
  delay(milli_dur);
  poll_button();
}

void delay_with_polling(int delay_ms) {
  int i;
  int increment = delay_ms/10;
  for (i = 0; i < delay_ms; i=i+increment) {
    delay(increment);
    if (poll_button) break;
  }
}

void light_show() {
  led_on(DITMS);
  led_off(DITMS);
  led_on(DITMS);
  led_off(DITMS);
  led_on(DAHMS);
  led_off(DITMS);
  led_on(DITMS);
  led_off(DAHMS);

  if (button) return;

  led_on(DITMS);
  led_off(DITMS);
  led_on(DITMS);
  led_off(DITMS);
  led_on(DAHMS);
  led_off(DAHMS);

  if (button) return;

  led_on(DAHMS);
  led_off(DITMS);
  led_on(DITMS);
  led_off(DITMS);
  led_on(DAHMS);
  led_off(DITMS);
  led_on(DITMS);
  led_off(DAHMS);

  if (button) return;

  led_on(DAHMS);
  led_off(DITMS);
  led_on(DITMS);
  led_off(DITMS);
  led_on(DAHMS);
  led_off(DAHMS);

  if (button) return;

  led_on(DITMS);
  led_off(DITMS);
  led_on(DAHMS);
  led_off(DITMS);
  led_on(DAHMS);
  led_off(DITMS);
  led_on(DITMS);
  led_off(DAHMS);

  if (button) return;

  led_on(DITMS);
  led_off(DITMS);
  led_on(DITMS);
  led_off(DITMS);
  led_on(DAHMS);
  led_off(DAHMS);

  if (button) return;

  led_on(DAHMS);
  led_off(DAHMS);

  if (button) return;

  led_on(DITMS);
  led_off(DITMS);
  led_on(DITMS);
  led_off(DAHMS);

  if (button) return;

  led_on(DAHMS);
  led_off(DITMS);
  led_on(DITMS);
  led_off(DAHMS);
}


void play_note(const uint8_t * rawnote, int bpm) {
  int periods, note_dur, k;
  uint8_t raw_note;
  uint16_t note_per, cycles, cycles_;

  raw_note = pgm_read_byte(rawnote);
  note_dur = ((raw_note>>5) & 0b00000111);
  periods = raw_note & 0b00011111;
  memcpy_P (&note_per, note_periods+periods, 2);

// note durations work with attiny85 runing at 4MHz internal clock
// this would need modification for notes outside of the B4-A5 note range
//  cycles = (0x82*500)/note_per; // just avoids overflow for B4-A5 note range  0x82 = 130
//  cycles = bpm*(0x82*26)/note_per; // just avoids overflow for B4-A5 note range, was 0x82*50/note_per
  cycles = bpm*169/note_per; // just avoids overflow for B4-A5 note range, was 0x82*50/note_per

    for (k = 0; k < pgm_read_byte(durations+note_dur); k++) {
      for (int i = 0; i < 4; i++) { // this, times overflow constrained cycles
        cycles_ = cycles;           // gives required duration for semiquaver
        while (cycles_ > 0) {
          if (periods != PAUSE) PORTB |= (1 << PB4);
          delayMicroseconds(note_per);
         
          if (periods != PAUSE) PORTB &= ~(1 << PB4);
          delayMicroseconds(note_per);
          cycles_--;
        }
      }
    }
  poll_button();
  delay(NOTE_DELAY_MS);
}

void make_glory() {
  char j;

  // play first 15ish notes of anthem
  for (j = 0; j < 15; j++) {
    play_note(anthem15+j,BPM72); // ~72BPM
  }

  delay(500);

  // play yakkety
  for (j = 0; j < 88; j++) {
    play_note(yakkety+j,BPM250); // ~250BPM
  }

}

// the main loop which repeatedly calls the LED lighting code, polls the button
// and plays the anthem if the button has been pressed
void loop() {

  button = 0;  
  while(1) {
    light_show();
    if (poll_button()) {
      button = 0;
      delay(60); // debounce
      make_glory();
      button = 0; // in case button pressed during anthem to finish it early
    }
    delay(1000);
  }
}
