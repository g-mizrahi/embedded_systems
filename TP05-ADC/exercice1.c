#include <avr/io.h>
#include <avr/interrupt.h>
#include <avr/power.h>
#include <avr/sleep.h>
#include <stdio.h>

ISR(TIMER1_COMPA_vect)
{
    // toggle the LED
    PORTD ^= _BV(PORTD4);
}

int main(void)
{
    // Power management section
    power_all_disable();             // Disable all modules
    power_timer1_enable();           // Enable the Timer1 module
    set_sleep_mode(SLEEP_MODE_IDLE); // Set the sleep mode to IDLE to keep the Timer1 running

    // LED setup section
    DDRD |= _BV(DDD4);     // Set the d4 pin to output mode
    PORTD &= ~_BV(PORTD4); // Initialize port d4 output to low

    // Timer1 setup section
    OCR1A = 0x7A12;                   // Set the TOP value to have 500ms time interval
    TCCR1B |= _BV(CS12) | _BV(WGM12); // Set the prescaler to 256 and the mode to CTC
    TIMSK1 |= _BV(OCIE1A);            // Set the interrupt mode to Match Compare A

    TCNT1 = 0; // Reset timer before starting the loop for consistent delays

    sei(); // Enable interuptions

    while (1)
    {
        // Sleep between every interruption
        sleep_mode();
    }
}