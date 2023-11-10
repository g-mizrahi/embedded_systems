PROG=LED_flash
PORT=ttyACM0
BAUDRATE=115200

upload: $(PROG).hex
	avrdude -patmega328p -carduino -P/dev/$(PORT) -b$(BAUDRATE) -Uflash:w:$(PROG).hex

$(PROG).hex: $(PROG).elf
	avr-objcopy -O ihex $(PROG).elf $(PROG).hex

$(PROG).elf: $(PROG).o
	avr-ld $(PROG).o -o $(PROG).elf

$(PROG).o: $(PROG).asm
	avr-as $(PROG).asm -o $(PROG).o

.PHONY: clean
clean:
	rm $(PROG).hex
	rm $(PROG).elf
	rm $(PROG).o
