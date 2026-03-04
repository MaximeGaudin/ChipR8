#[cfg(test)]
mod tests {
    use std::error::Error;
    use test_case::test_case;

    use crate::load_rom;

    #[test_case("roms/rock-paper-scissors")]
    #[test_case("roms/corax89")]
    fn compare_rom_content_and_loaded_content(rom_prefix: &str) {
        let reference_opcodes = to_opcodes(format!("{}.dis", rom_prefix)).unwrap();
        let opcodes = load_rom(format!("{}.ch8", rom_prefix)).unwrap();

        assert_eq!(reference_opcodes, opcodes);
    }

    /*
     * This methods convert a normalized disassembled file from https://johnearnest.github.io/Octo/
     * into a list of u16 for comparison.
     */
    fn to_opcodes(path: String) -> Result<Vec<u16>, Box<dyn Error>> {
        let content = std::fs::read_to_string(path)?;
        let bytes: Vec<u8> = content
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .filter_map(|s| {
                // Remove "0x" and try to parse
                let hex = s.trim_start_matches("0x");
                u8::from_str_radix(hex, 16).ok()
            })
            .collect();

        Ok(bytes
            .chunks_exact(2)
            .map(|chunk| {
                // byte1 = high bits, byte2 = low bits
                ((chunk[0] as u16) << 8) | (chunk[1] as u16)
            })
            .collect())
    }
}
