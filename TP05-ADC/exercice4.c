#include <avr/io.h>
#include <avr/interrupt.h>
#include <avr/power.h>
#include <avr/sleep.h>
#include <stdio.h>

ISR(TIMER0_OVF_vect)
{
    if (OCR0A == 255)
    {
        OCR0A = 11;
    }
    else
    {
        OCR0A++;
    }
}

int main(void)
{
    // Power management section
    power_all_disable();             // Disable all modules
    power_timer0_enable();           // Enable the Timer0 module
    set_sleep_mode(SLEEP_MODE_IDLE); // Set the sleep mode to IDLE to keep the Timer0 running

    // LED setup section
    // Port D6 is the OC0A output
    DDRD |= _BV(DDD6); // Set the port D6 in output mode

    // Timer0 setup section
    OCR0A = 0x0;                                    // Set the TOP value to 0 initially
    TCCR0A = _BV(WGM01) | _BV(WGM00) | _BV(COM0A1); // Set the mode to Fast PWM with the desired mode for Timer0
    TCCR0B = _BV(CS02);                             // Set the clock to io with prescaler 256
    TIMSK0 = _BV(TOIE0);                            // Set the interrupt mode to overflow
    TCNT0 = 0;                                      // Reset timer before starting the loop for consistent delays

    sei();

    while (1)
    {
        sleep_mode();
    }
}