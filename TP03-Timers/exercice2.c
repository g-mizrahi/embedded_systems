#include <avr/io.h>
#include <avr/interrupt.h>
#include <avr/power.h>
#include <avr/sleep.h>
#include <avr/pgmspace.h>

/*PGM_P = const char* et char = PROGMEM prog_char */
volatile PGM_P signal = "=.=.=...===.===.===...=.=.=........";

/*PROGMEM prog_uint8*/
volatile uint8_t i = 0;

ISR(TIMER1_COMPA_vect)
{
    switch (signal[i])
    {
    case '\0':
        i = 0;
        break;
    case '=':
        PORTB |= _BV(PORTB5);
        i = i + 1;
        break;
    case '.':
        PORTB &= ~_BV(PORTB5);
        i = i + 1;
        break;
    }
}

int main(void)
{
    // Power management section
    power_all_disable();             // Disable all modules
    power_timer1_enable();           // Enable the counter 1 module
    set_sleep_mode(SLEEP_MODE_IDLE); // Set the sleep mode to IDLE to keep the timer/counter1 running

    // LED setup section
    DDRB |= _BV(DDB5); // Set DDRB to 1 for the LED

    // CTC setup section
    OCR1A = 0x7A12;        // Set the TOP value to have 500ms time interval
    TCCR1B |= _BV(CS12) | _BV(WGM12);  // Set the prescaler to 256 and the mode to CTC
    TIMSK1 |= _BV(OCIE1A); // Set the interrupt mode to Match Compare A

    TCNT1 = 0; // Reset timer before starting the loop for consistent delays

    sei(); // Enable interuptions

    while (1)
    {
        // Sleep between every interruption
        sleep_mode();
    }
}