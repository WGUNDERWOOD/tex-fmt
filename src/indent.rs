pub mod indent {

    use core::cmp::max;
    use crate::regexes::regexes::*;

    const OPENS: [char; 3] = ['(', '[', '{'];
    const CLOSES: [char; 3] = [')', ']', '}'];

    #[derive(Debug)]
    pub struct Indent {
        /// actual running indentation count at end of current line
        pub actual: i8,
        /// visual indentation of current line
        pub visual: i8,
    }

    /// calculate total indentation change due to current line
    pub fn get_diff(line: &str) -> i8 {
        if RE_DOCUMENT_BEGIN.is_match(line) || RE_DOCUMENT_END.is_match(line) {
            return 0;
        };

        let mut diff: i8 = 0;
        if RE_ENV_BEGIN.is_match(line) {
            diff += 1
        };
        if RE_ENV_END.is_match(line) {
            diff -= 1
        };
        if RE_ENV_BEGIN.is_match(line) {
            diff += 1
        };
        if RE_ENV_END.is_match(line) {
            diff -= 1
        };

        for c in OPENS {
            diff += line.chars().filter(|&x| x == c).count() as i8;
        }
        for c in CLOSES {
            diff -= line.chars().filter(|&x| x == c).count() as i8;
        }
        diff
    }

    /// calculate dedentation for current line compared to previous
    pub fn get_back(line: &str) -> i8 {
        if RE_DOCUMENT_END.is_match(line) {
            return 0;
        };

        if RE_ENV_END.is_match(line) {
            return 1;
        };

        let mut back: i8 = 0;
        let mut cumul: i8 = 0;
        for c in line.chars() {
            cumul -= OPENS.contains(&c) as i8;
            cumul += CLOSES.contains(&c) as i8;
            back = max(cumul, back);
        }
        back
    }

    pub fn get_indent(line: &str, prev_indent: Indent) -> Indent {
        let diff = get_diff(line);
        let back = get_back(line);
        let actual = prev_indent.actual + diff;
        let visual: i8 = prev_indent.actual - back;
        Indent{actual, visual}
    }
}
