#include <avr/io.h>
#include <stdint.h>

#define FOSC 16000000 // Clock Speed 16MHz
#define BAUD 9600
#define MYUBRR FOSC / 16 / BAUD - 1

void UART__init(uint16_t);
uint8_t UART__getc(void);
void UART__putc(uint8_t);

void UART__init(uint16_t ubrr)
{
    // Set baud rate
    UBRR0H = (uint8_t)(ubrr >> 8);
    UBRR0L = (uint8_t)ubrr;
    // Enable receiver and transmitter
    UCSR0B |= _BV(RXEN0) | _BV(TXEN0);
    // Set frame format: 8 data, no parity, 1 stop bit (default)
    UCSR0C |= _BV(UCSZ00) | _BV(UCSZ01); // 8 bits of data
    UCSR0C &= ~_BV(UPM01);               // no parity
    UCSR0C &= ~_BV(UPM00);               // no parity
    UCSR0C &= ~_BV(USBS0);               // 1 stop bit
}

uint8_t UART__getc()
{
    // Wait for data to be received
    while (!(UCSR0A & _BV(RXC0)))
    {
        // Simply wait
    }
    // Get and return received data from buffer
    return UDR0;
}

void UART__putc(uint8_t c)
{
    while (!(UCSR0A & _BV(UDRE0)))
    {
        // Simply wait
    }
    // Put data into buffer, sends the data
    UDR0 = c;
}

void main(void)
{
    UART__init(MYUBRR);
    uint8_t c;
    while (1)
    {
        c = UART__getc();
        UART__putc('#');
        UART__putc(c);
    }
}