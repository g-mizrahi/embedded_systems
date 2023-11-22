#include <avr/io.h>
#include <avr/interrupt.h>
#include <avr/power.h>
#include <avr/sleep.h>
#include <stdio.h>

ISR(ADC_vect)
{
    OCR0A = ADCH; // Change brightness of LED
}

int main(void)
{
    // Power management section
    power_all_disable();             // Disable all modules
    power_timer0_enable();           // Enable the Timer0 module
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
    // Set the result to 8 bits precision
    // MUX3..0 left to 0 to select pin AD0
    ADMUX = _BV(REFS0) | _BV(ADLAR);
    // ADCSRB is left to 0 to select free running mode

    // Timer0 setup section
    OCR0A = 0x19;                                   // Set the TOP value to have 0.1 duty cycle
    TCCR0A = _BV(WGM01) | _BV(WGM00) | _BV(COM0A1); // Set the mode to Fast PWM with the desired mode for Timer0
    TCCR0B = _BV(CS00);                             // Set the clock to io with no prescaler
    TCNT0 = 0;                                      // Reset timer before starting the loop for consistent delays

    sei();
    ADCSRA |= _BV(ADSC); // Start conversion

    while (1)
    {
        sleep_mode();
    }
}