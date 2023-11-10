; NIST Team TP02 - Arduino LED
; Members :
;       - Rayane Bait
;       - Guilhem Mizrahi
;
; Goal of the program :
;       - Blink the LED at 500ms intervals
; Result :
;       - LED is blinking every 512 ms
;       - Experiment :
;           - 100 full periods (light + dark) in 1m42s30
;           - real wait time is 0.5115s

; Setting some useful names for better readability
.EQU SPH    , 0x3E; Stack Pointer SPH location
.EQU SPL    , 0x3D; Stack Pointer SPL location
.EQU DDB5   , 0x04; LED DDB5 pin address location
.EQU PORTB5 , 0x05; LED PORTB5 pin address location
.EQU DDB5_BIT   , 5; LED DDB5 bit number
.EQU PORTB5_BIT , 5; LED PORTB5 bit number 

; 1 - Preparing the stack
;   - Section 7.5 of the ATmega328 specs
;       - Initial Stack Pointer value equals the last address of the internal SRAM
;       - Stack Pointer must be set to point above start of the SRAM
;   - Because the Arduino Uno has 2KB of SRAM, the top of the SRAM is at address 0x8FF (cf. Fig 8.3 page 28 of specs)
;   - The registers SPH:SPL need to be set to 0x08FF

; Setting the stack pointer to 0x08FF
LDI r18, 0x08;
LDI r19, 0xFF;
OUT SPH, r18;
OUT SPL, r19;

; 2 - Setting the LED pins 
;   - The pin controlling the LED is B5
;       - To set the pin to high output DDB5 = 1 and PORTB5 = 1
;       - To set the pin to low output DDB5 = 1 and PORTB5 = 0
START:
    SBI DDB5, DDB5_BIT;

LIGHT:
    SBI PORTB5, PORTB5_BIT;
    RCALL sleep_500_ms;

DARK:
    CBI PORTB5, PORTB5_BIT;
    RCALL sleep_500_ms;
    RJMP LIGHT;

; 3 - The sleep_500_ms function
;   - Each iteration performs the following operations :
;       - SUBI : 1 CPU cycle
;       - SBCI x2 : 1 CPU cycle each
;       - BRCC : 1 CPU cycle when C is set (exit), 2 CPU cycles when C is cleared (loop)
;   - Total : 5 CPU cycles per loop iteration
;   - The Arduino Uno is clocked at 16MHz
;   - The number of loop iterations to wait 500ms is :
;       - 500ms * 16MHz / 5cycles = 1.6M iterations
;       - 0x18FFFF = 1638399 ~ 1.6M
;       - this value should produce a wait time of 512ms
;       - 0x186A00 = 1.6M
;       - this value should produce a wait time of 500ms
sleep_500_ms:
; Set the registers to 0x186A00
    LDI r18, 0x18;
    LDI r19, 0x6A;
    LDI r20, 0x00;
LOOP:
; Loop until all r18 has reached 0
    SUBI r20, 1; use SUBI so that the carry bit C is set when r20 has reached 0
    SBCI r19, 0; decrement only if C is set
    SBCI r18, 0; decrement only if C is set
    BRNE LOOP; C is set if r18 reached 0 and exit the loop
    RET;