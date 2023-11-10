ifndef PROG
	PROG=exercice1
endif
PORT=ttyACM0
BAUDRATE=115200
WARNINGS := -Wall -Wextra -pedantic -Wshadow -Wpointer-arith -Wcast-align \
            -Wwrite-strings -Wmissing-prototypes -Wmissing-declarations \
            -Wredundant-decls -Wnested-externs -Winline -Wno-long-long \
            -Wstrict-prototypes -Wno-array-bounds

CFLAGS := -Os -DF_CPU=16000000UL -mmcu=atmega328p $(WARNINGS)

upload: $(PROG).hex
	avrdude -v -patmega328p -carduino -P/dev/$(PORT) -b$(BAUDRATE) -D -Uflash:w:$^

$(PROG).hex: $(PROG).elf
	avr-objcopy -O ihex $^ $@

$(PROG).elf: $(PROG).c
	avr-gcc $(CFLAGS) -o $@ $<

.PHONY: clean
clean:
	rm $(PROG).hex
	rm $(PROG).elf