#include <avr/io.h>
#include <util/delay.h>
#include <avr/pgmspace.h>
// #include <stdint.h>

#define BLINK_DELAY_MS 500

int main(void)
{
    /*PGM_P = const char* et char = PROGMEM prog_char */
    PGM_P signal = "=.=.=...===.===.===...=.=.=........";

    /*PROGMEM prog_uint8*/
    uint8_t i = 0;

    DDRB |= _BV(DDB5);

    while (1)
    {
        switch (signal[i])
        {
        case '\0':
            i = 0;
            continue;
        case '=':
            PORTB |= _BV(PORTB5);
            _delay_ms(BLINK_DELAY_MS);
            break;
        case '.':
            PORTB &= ~_BV(PORTB5);
            _delay_ms(BLINK_DELAY_MS);
            break;
        }
        i = i + 1;
    }
}