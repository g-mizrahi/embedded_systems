#include <avr/io.h>
#include <avr/interrupt.h>
#include <avr/power.h>
#include <avr/sleep.h>
#include <stdio.h>

ISR(TIMER1_COMPA_vect)
{
    // toggle the LED
    PORTD ^= _BV(PORTD6);
}

ISR(ADC_vect)
{
    OCR1A = ADCH << 8;
}

int main(void)
{
    // Power management section
    power_all_disable();             // Disable all modules
    power_timer1_enable();           // Enable the Timer0 module
    power_adc_enable();              // Enable the ADC
    set_sleep_mode(SLEEP_MODE_IDLE); // Set the sleep mode to IDLE to keep the Timer0 running and the ADC

    // LED setup section
    // Port D6 is the OC0A output
    DDRD |= _BV(DDD6); // Set the port D6 in output mode

    // ADC setup
    // ADEN : enable the ADC
    // ADATE : set the auto trigger
    // ADIE : set the interupt enable
    // Set the clock prescaler to 128 for a 125kHz frequency
    ADCSRA |= _BV(ADEN) | _BV(ADATE) | _BV(ADIE) | _BV(ADPS2) | _BV(ADPS1) | _BV(ADPS0);
    // Select AVcc as reference voltage
    // Set the result to 8 bits precision in the ADCH register
    // MUX3..0 left to 0 to select pin AD0
    ADMUX = _BV(REFS0) | _BV(ADLAR);
    // ADCSRB is left to 0 to select free running mode

    // Timer1 setup section
    OCR1A = 0x7A12;                   // Set the TOP value to have 500ms time interval initially (it will be erased by ADC)
    TCCR1B |= _BV(CS12) | _BV(WGM12); // Set the prescaler to 256 and the mode to CTC
    TIMSK1 |= _BV(OCIE1A);            // Set the interrupt mode to Match Compare A

    TCNT1 = 0; // Reset timer before starting the loop for consistent delays

    sei();
    ADCSRA |= _BV(ADSC); // Start conversion

    while (1)
    {
        sleep_mode();
    }
}