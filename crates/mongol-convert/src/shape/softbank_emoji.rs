//! Restore legacy SoftBank/iOS emoji that collide with MenkShape PUA.
//!
//! MenkShape uses part of the BMP private-use area. Old iOS/SoftBank emoji used the same PUA
//! code points, and some chat apps still rewrite those codes to modern Unicode emoji. This module
//! reverses only the mappings whose PUA side falls inside MenkShape's `E234..=E34F` range.
//!
//! Source table: https://raw.githubusercontent.com/iamcal/emoji-data/master/build/data_softbank_map.txt

use std::borrow::Cow;

pub(crate) fn restore_menk_shape(text: &str) -> Cow<'_, str> {
    let Some(first) = text
        .char_indices()
        .find_map(|(index, c)| to_menk_shape(c).map(|_| index))
    else {
        return Cow::Borrowed(text);
    };

    let mut out = String::with_capacity(text.len());
    out.push_str(&text[..first]);
    let mut skip_variation_selector = false;

    for c in text[first..].chars() {
        if skip_variation_selector && c == '\u{FE0F}' {
            skip_variation_selector = false;
            continue;
        }
        skip_variation_selector = false;

        if let Some(menk_shape) = to_menk_shape(c) {
            out.push(menk_shape);
            skip_variation_selector = true;
        } else {
            out.push(c);
        }
    }

    Cow::Owned(out)
}

fn to_menk_shape(c: char) -> Option<char> {
    Some(match c {
        '\u{27A1}' => '\u{E234}',
        '\u{2B05}' => '\u{E235}',
        '\u{2197}' => '\u{E236}',
        '\u{2196}' => '\u{E237}',
        '\u{2198}' => '\u{E238}',
        '\u{2199}' => '\u{E239}',
        '\u{25B6}' => '\u{E23A}',
        '\u{25C0}' => '\u{E23B}',
        '\u{23E9}' => '\u{E23C}',
        '\u{23EA}' => '\u{E23D}',
        '\u{1F52F}' => '\u{E23E}',
        '\u{2648}' => '\u{E23F}',
        '\u{2649}' => '\u{E240}',
        '\u{264A}' => '\u{E241}',
        '\u{264B}' => '\u{E242}',
        '\u{264C}' => '\u{E243}',
        '\u{264D}' => '\u{E244}',
        '\u{264E}' => '\u{E245}',
        '\u{264F}' => '\u{E246}',
        '\u{2650}' => '\u{E247}',
        '\u{2651}' => '\u{E248}',
        '\u{2652}' => '\u{E249}',
        '\u{2653}' => '\u{E24A}',
        '\u{26CE}' => '\u{E24B}',
        '\u{1F51D}' => '\u{E24C}',
        '\u{1F197}' => '\u{E24D}',
        '\u{00A9}' => '\u{E24E}',
        '\u{00AE}' => '\u{E24F}',
        '\u{1F4F3}' => '\u{E250}',
        '\u{1F4F4}' => '\u{E251}',
        '\u{26A0}' => '\u{E252}',
        '\u{1F481}' => '\u{E253}',
        '\u{1F4DD}' => '\u{E301}',
        '\u{1F454}' => '\u{E302}',
        '\u{1F33A}' => '\u{E303}',
        '\u{1F337}' => '\u{E304}',
        '\u{1F33B}' => '\u{E305}',
        '\u{1F490}' => '\u{E306}',
        '\u{1F334}' => '\u{E307}',
        '\u{1F335}' => '\u{E308}',
        '\u{1F6BE}' => '\u{E309}',
        '\u{1F3A7}' => '\u{E30A}',
        '\u{1F376}' => '\u{E30B}',
        '\u{1F37B}' => '\u{E30C}',
        '\u{3297}' => '\u{E30D}',
        '\u{1F6AC}' => '\u{E30E}',
        '\u{1F48A}' => '\u{E30F}',
        '\u{1F388}' => '\u{E310}',
        '\u{1F4A3}' => '\u{E311}',
        '\u{1F389}' => '\u{E312}',
        '\u{2702}' => '\u{E313}',
        '\u{1F380}' => '\u{E314}',
        '\u{3299}' => '\u{E315}',
        '\u{1F4BD}' => '\u{E316}',
        '\u{1F4E3}' => '\u{E317}',
        '\u{1F452}' => '\u{E318}',
        '\u{1F457}' => '\u{E319}',
        '\u{1F461}' => '\u{E31A}',
        '\u{1F462}' => '\u{E31B}',
        '\u{1F484}' => '\u{E31C}',
        '\u{1F485}' => '\u{E31D}',
        '\u{1F486}' => '\u{E31E}',
        '\u{1F487}' => '\u{E31F}',
        '\u{1F488}' => '\u{E320}',
        '\u{1F458}' => '\u{E321}',
        '\u{1F459}' => '\u{E322}',
        '\u{1F45C}' => '\u{E323}',
        '\u{1F3AC}' => '\u{E324}',
        '\u{1F514}' => '\u{E325}',
        '\u{1F3B6}' => '\u{E326}',
        '\u{1F493}' => '\u{E327}',
        '\u{1F497}' => '\u{E328}',
        '\u{1F498}' => '\u{E329}',
        '\u{1F499}' => '\u{E32A}',
        '\u{1F49A}' => '\u{E32B}',
        '\u{1F49B}' => '\u{E32C}',
        '\u{1F49C}' => '\u{E32D}',
        '\u{2728}' => '\u{E32E}',
        '\u{2B50}' => '\u{E32F}',
        '\u{1F4A8}' => '\u{E330}',
        '\u{1F4A6}' => '\u{E331}',
        '\u{2B55}' => '\u{E332}',
        '\u{274C}' => '\u{E333}',
        '\u{1F4A2}' => '\u{E334}',
        '\u{1F31F}' => '\u{E335}',
        '\u{2754}' => '\u{E336}',
        '\u{2755}' => '\u{E337}',
        '\u{1F375}' => '\u{E338}',
        '\u{1F35E}' => '\u{E339}',
        '\u{1F366}' => '\u{E33A}',
        '\u{1F35F}' => '\u{E33B}',
        '\u{1F361}' => '\u{E33C}',
        '\u{1F358}' => '\u{E33D}',
        '\u{1F35A}' => '\u{E33E}',
        '\u{1F35D}' => '\u{E33F}',
        '\u{1F35C}' => '\u{E340}',
        '\u{1F35B}' => '\u{E341}',
        '\u{1F359}' => '\u{E342}',
        '\u{1F362}' => '\u{E343}',
        '\u{1F363}' => '\u{E344}',
        '\u{1F34E}' => '\u{E345}',
        '\u{1F34A}' => '\u{E346}',
        '\u{1F353}' => '\u{E347}',
        '\u{1F349}' => '\u{E348}',
        '\u{1F345}' => '\u{E349}',
        '\u{1F346}' => '\u{E34A}',
        '\u{1F382}' => '\u{E34B}',
        '\u{1F371}' => '\u{E34C}',
        '\u{1F372}' => '\u{E34D}',
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn restores_only_when_needed() {
        assert!(matches!(
            restore_menk_shape("plain"),
            Cow::Borrowed("plain")
        ));
        assert_eq!(restore_menk_shape("➡️🎂").as_ref(), "\u{E234}\u{E34B}");
    }
}
