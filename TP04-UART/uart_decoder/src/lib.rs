pub mod uart {
    #[derive(Debug)]
    pub struct UartError {
        pub details: String,
    }

    pub struct UartConfig {
        pub data_len: usize,
        pub parity: bool,
        pub frame_len: usize,
    }

    pub struct UartFrame {
        pub data: String,
        pub parity: bool,
    }

    impl UartConfig {
        pub fn new(len: usize, parity: bool) -> Result<Self, UartError> {
            if len >= 5 && len <= 9 {
                return Ok(UartConfig {
                    data_len: len,
                    parity: parity,
                    frame_len: if parity { len + 3 } else { len + 2 },
                });
            } else {
                return Err(UartError {
                    details: format!("Invalid invalid config value : data_len {len} should be usize between 5 and 9 included.").to_string(),
                });
            }
        }
    }

    pub fn parse_frame(config: &UartConfig, frame: &str) -> UartFrame {
        if config.parity {
            return UartFrame {
                data: frame[1..config.frame_len - 2].chars().rev().collect(),
                parity: (frame[1..config.frame_len - 1]
                    .chars()
                    .filter(|c| *c == '1')
                    .count()
                    % 2
                    == 0),
            };
        } else {
            return UartFrame {
                data: frame[1..config.frame_len - 1].chars().rev().collect(),
                parity: true,
            };
        }
    }

    pub fn parse_signal<'a>(config: &'a UartConfig, signal: &'a str) -> Vec<&'a str> {
        // Ignore leading 1 until reaching the first 0 indicating a start
        let mut frames: Vec<&str> = vec![];
        let mut i: usize = 0;
        while i < signal.len() {
            if signal.chars().nth(i) == Some('1') {
                i += 1;
            } else {
                frames.push(&signal[i..i + config.frame_len]);
                i += config.frame_len;
            }
        }
        return frames;
    }
}

#[cfg(test)]
mod tests {
    use crate::uart::*;

    #[test]
    fn tp04_config_new_success() {
        let config = UartConfig::new(8, false).unwrap();
        assert_eq!(config.data_len, 8);
        assert_eq!(config.parity, false);
        assert_eq!(config.frame_len, 10);
    }

    #[test]
    fn tp04_config_new_error() {
        assert!(UartConfig::new(10, false).is_err());
    }

    #[test]
    fn tp04_parse_signal() {
        let config = UartConfig::new(8, true).unwrap();
        let signal = "1110100001101100100011011011000110010001001101111111";
        let frames: Vec<UartFrame> = parse_signal(&config, signal)
            .iter()
            .map(|f| parse_frame(&config, f))
            .collect();
        for frame in frames.iter() {
            if frame.parity {
                println!("{}", u8::from_str_radix(&frame.data, 2).unwrap() as char);
            } else {
                println!("Corrupted data.");
            }
        }
    }
}
