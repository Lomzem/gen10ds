/// Calculate the output format for a group (Group 1 or Group 2)
/// based on DIP switch settings
pub fn calculate_group_format(
    sync_switch: bool,  // false = SD, true = HD
    sw4_format: bool,   // false = 59.94 (NTSC), true = 50 (PAL)
    sw5_hd_fmt: bool,   // false = 1080, true = 720
    sw6_s1: bool,
    sw7_s2: bool,
    sw8_s3: bool,
) -> String {
    // If sync switch is LEFT (SD mode), show SD format
    if !sync_switch {
        return if !sw4_format {
            "SD - 525i (NTSC)".to_string()
        } else {
            "SD - 625i (PAL)".to_string()
        };
    }

    // HD mode - calculate from Table 2
    let hd_format = calculate_hd_format_from_table2(sw4_format, sw5_hd_fmt, sw6_s1, sw7_s2, sw8_s3);
    format!("HD - {}", hd_format)
}

/// Implementation of Table 2 from DIPSWITCHES.md
/// Returns the HD format string based on switches 4-8
fn calculate_hd_format_from_table2(
    sw4_format: bool,  // false = 59.94, true = 50
    sw5_hd_fmt: bool,  // false = 1080, true = 720
    sw6_s1: bool,
    sw7_s2: bool,
    sw8_s3: bool,
) -> String {
    // Calculate binary value from S1, S2, S3 (switches 6, 7, 8)
    let s1_s2_s3 = (sw6_s1 as u8) | ((sw7_s2 as u8) << 1) | ((sw8_s3 as u8) << 2);

    match (sw4_format, sw5_hd_fmt, s1_s2_s3) {
        // SW4=LEFT (59.94), SW5=LEFT (1080)
        (false, false, 0b000) => "1080i59.94 (1080psf29.97)".to_string(),
        (false, false, 0b001) => "1080psf23.98".to_string(),
        (false, false, 0b010) => "1080p23.98".to_string(),
        (false, false, 0b011) => "1080p29.97".to_string(),
        (false, false, 0b100) => "1080i60 (1080psf30)".to_string(),
        (false, false, 0b101) => "1080psf24".to_string(),
        (false, false, 0b110) => "1080p24".to_string(),
        (false, false, 0b111) => "1080p30".to_string(),

        // SW4=LEFT (59.94), SW5=RIGHT (720)
        (false, true, 0b000) => "720p59.94".to_string(),
        (false, true, 0b001) => "720p23.98".to_string(),
        (false, true, 0b010) => "720p23.98".to_string(),
        (false, true, 0b011) => "720p29.97".to_string(),
        (false, true, 0b100) => "720p60".to_string(),
        (false, true, 0b101) => "720p24".to_string(),
        (false, true, 0b110) => "720p24".to_string(),
        (false, true, 0b111) => "720p30".to_string(),

        // SW4=RIGHT (50), SW5=LEFT (1080)
        (true, false, 0b000) => "1080i50".to_string(),
        (true, false, 0b001) => "1080i50".to_string(),
        (true, false, 0b010) => "1080p25".to_string(),
        (true, false, 0b011) => "1080p25".to_string(),
        (true, false, 0b100) => "1080i50 (1080psf25)".to_string(),
        (true, false, 0b101) => "1080i50".to_string(),
        (true, false, 0b110) => "1080p25".to_string(),
        (true, false, 0b111) => "1080p25".to_string(),

        // SW4=RIGHT (50), SW5=RIGHT (720)
        (true, true, 0b000) => "720p50".to_string(),
        (true, true, 0b001) => "720p25".to_string(),
        (true, true, 0b010) => "720p25".to_string(),
        (true, true, 0b011) => "720p25".to_string(),
        (true, true, 0b100) => "720p50".to_string(),
        (true, true, 0b101) => "720p25".to_string(),
        (true, true, 0b110) => "720p25".to_string(),
        (true, true, 0b111) => "720p25".to_string(),

        _ => unreachable!(), // All cases covered
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sd_ntsc_format() {
        let format = calculate_group_format(false, false, false, false, false, false);
        assert_eq!(format, "SD - 525i (NTSC)");
    }

    #[test]
    fn test_sd_pal_format() {
        let format = calculate_group_format(false, true, false, false, false, false);
        assert_eq!(format, "SD - 625i (PAL)");
    }

    #[test]
    fn test_hd_1080i59_94() {
        // SW4=LEFT (59.94), SW5=LEFT (1080), S1S2S3=000
        let format = calculate_group_format(true, false, false, false, false, false);
        assert_eq!(format, "HD - 1080i59.94 (1080psf29.97)");
    }

    #[test]
    fn test_hd_720p50() {
        // SW4=RIGHT (50), SW5=RIGHT (720), S1S2S3=000
        let format = calculate_group_format(true, true, true, false, false, false);
        assert_eq!(format, "HD - 720p50");
    }

    #[test]
    fn test_hd_1080p24() {
        // SW4=LEFT (59.94), SW5=LEFT (1080), S1S2S3=110
        let format = calculate_group_format(true, false, false, false, true, true);
        assert_eq!(format, "HD - 1080p24");
    }

    #[test]
    fn test_hd_1080p25() {
        // SW4=RIGHT (50), SW5=LEFT (1080), S1S2S3=010
        let format = calculate_group_format(true, true, false, false, true, false);
        assert_eq!(format, "HD - 1080p25");
    }

    #[test]
    fn test_all_table2_combinations() {
        // Test that all 32 combinations produce valid output
        for sw4 in [false, true] {
            for sw5 in [false, true] {
                for s1 in [false, true] {
                    for s2 in [false, true] {
                        for s3 in [false, true] {
                            let format = calculate_group_format(true, sw4, sw5, s1, s2, s3);
                            assert!(format.starts_with("HD - "));
                        }
                    }
                }
            }
        }
    }
}
