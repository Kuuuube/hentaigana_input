use std::collections::BTreeMap;
use regex::Regex;

fn get_hentaigana_group(romaji: &str) -> BTreeMap<&str, &str> {
    let a_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛀂"),
        ("2", "𛀅"),
        ("3", "𛀃"),
        ("4", "𛀄")
    ]);

    let i_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛀆"),
        ("2", "𛀇"),
        ("3", "𛀈"),
        ("4", "𛀉"),
    ]);

    let u_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛀊"),
        ("2", "𛀋"),
        ("3", "𛀌"),
        ("4", "𛀍"),
        ("5", "𛀎"),
    ]);

    let e_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛀁"),
        ("2", "𛀏"),
        ("3", "𛀐"),
        ("4", "𛀑"),
        ("5", "𛀒"),
        ("6", "𛀓"),
    ]);

    let o_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛀔"),
        ("2", "𛀕"),
        ("3", "𛀖"),
    ]);

    let ka_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛀗"),
        ("2", "𛀘"),
        ("3", "𛀙"),
        ("4", "𛀚"),
        ("5", "𛀛"),
        ("6", "𛀢"),
        ("7", "𛀜"),
        ("8", "𛀝"),
        ("9", "𛀞"),
        ("0", "𛀟"),
        ("-", "𛀠"),
        ("=", "𛀡"),
    ]);

    let ki_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛀣"),
        ("2", "𛀤"),
        ("3", "𛀥"),
        ("4", "𛀦"),
        ("5", "𛀻"),
        ("6", "𛀧"),
        ("7", "𛀨"),
        ("8", "𛀩"),
        ("9", "𛀪"),
    ]);

    let ku_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛀫"),
        ("2", "𛀬"),
        ("3", "𛀭"),
        ("4", "𛀮"),
        ("5", "𛀯"),
        ("6", "𛀰"),
        ("7", "𛀱"),
    ]);

    let ke_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛀳"),
        ("2", "𛀲"),
        ("3", "𛀢"),
        ("4", "𛀴"),
        ("5", "𛀵"),
        ("6", "𛀶"),
        ("7", "𛀷"),
    ]);

    let ko_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛀸"),
        ("2", "𛂘"),
        ("3", "𛀹"),
        ("4", "𛀻"),
        ("5", "𛀺"),
    ]);

    let sa_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛀼"),
        ("2", "𛀽"),
        ("3", "𛀾"),
        ("4", "𛀿"),
        ("5", "𛁀"),
        ("6", "𛁁"),
        ("7", "𛁂"),
        ("8", "𛁃"),
    ]);

    let shi_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛁄"),
        ("2", "𛁅"),
        ("3", "𛁆"),
        ("4", "𛁇"),
        ("5", "𛁈"),
        ("6", "𛁉"),
    ]);

    let su_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛁊"),
        ("2", "𛁋"),
        ("3", "𛁌"),
        ("4", "𛁍"),
        ("5", "𛁎"),
        ("6", "𛁏"),
        ("7", "𛁐"),
        ("8", "𛁑"),
    ]);

    let se_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛁒"),
        ("2", "𛁓"),
        ("3", "𛁔"),
        ("4", "𛁕"),
        ("5", "𛁖"),
    ]);

    let so_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛁗"),
        ("2", "𛁘"),
        ("3", "𛁙"),
        ("4", "𛁚"),
        ("5", "𛁛"),
        ("6", "𛁜"),
        ("7", "𛁝"),
    ]);

    let ta_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛁞"),
        ("2", "𛁟"),
        ("3", "𛁠"),
        ("4", "𛁡"),
    ]);

    let chi_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛁢"),
        ("2", "𛁣"),
        ("3", "𛁤"),
        ("4", "𛁥"),
        ("5", "𛁦"),
        ("6", "𛁧"),
        ("7", "𛁨"),
    ]);

    let tsu_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛁩"),
        ("2", "𛁪"),
        ("3", "𛁫"),
        ("4", "𛁬"),
        ("5", "𛁭"),
    ]);

    let te_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛁮"),
        ("2", "𛁯"),
        ("3", "𛁰"),
        ("4", "𛁱"),
        ("5", "𛁲"),
        ("6", "𛁳"),
        ("7", "𛁴"),
        ("8", "𛁵"),
        ("9", "𛁶"),
        ("0", "𛂎"),
    ]);

    let to_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛁷"),
        ("2", "𛁸"),
        ("3", "𛁹"),
        ("4", "𛁺"),
        ("5", "𛁻"),
        ("6", "𛁼"),
        ("7", "𛁽"),
        ("8", "𛁭"),
    ]);

    let na_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛁾"),
        ("2", "𛁿"),
        ("3", "𛂀"),
        ("4", "𛂁"),
        ("5", "𛂂"),
        ("6", "𛂃"),
        ("7", "𛂄"),
        ("8", "𛂅"),
        ("9", "𛂆"),
    ]);

    let ni_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛂇"),
        ("2", "𛂈"),
        ("3", "𛂉"),
        ("4", "𛂊"),
        ("5", "𛂋"),
        ("6", "𛂌"),
        ("7", "𛂍"),
        ("8", "𛂎"),
    ]);

    let nu_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛂏"),
        ("2", "𛂐"),
        ("3", "𛂑"),
    ]);

    let ne_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛂒"),
        ("2", "𛂓"),
        ("3", "𛂔"),
        ("4", "𛂕"),
        ("5", "𛂖"),
        ("6", "𛂗"),
        ("7", "𛂘"),
    ]);

    let no_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛂙"),
        ("2", "𛂚"),
        ("3", "𛂛"),
        ("4", "𛂜"),
        ("5", "𛂝"),
    ]);

    let ha_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛂞"),
        ("2", "𛂟"),
        ("3", "𛂠"),
        ("4", "𛂡"),
        ("5", "𛂢"),
        ("6", "𛂣"),
        ("7", "𛂤"),
        ("8", "𛂥"),
        ("9", "𛂦"),
        ("0", "𛂧"),
        ("-", "𛂨"),
    ]);

    let hi_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛂩"),
        ("2", "𛂪"),
        ("3", "𛂫"),
        ("4", "𛂬"),
        ("5", "𛂭"),
        ("6", "𛂮"),
        ("7", "𛂯"),
    ]);

    let fu_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛂰"),
        ("2", "𛂱"),
        ("3", "𛂲"),
    ]);

    let he_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛂳"),
        ("2", "𛂴"),
        ("3", "𛂵"),
        ("4", "𛂶"),
        ("5", "𛂷"),
        ("6", "𛂸"),
        ("7", "𛂹"),
    ]);

    let ho_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛂺"),
        ("2", "𛂻"),
        ("3", "𛂼"),
        ("4", "𛂽"),
        ("5", "𛂾"),
        ("6", "𛂿"),
        ("7", "𛃀"),
        ("8", "𛃁"),
    ]);

    let ma_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛃂"),
        ("2", "𛃃"),
        ("3", "𛃄"),
        ("4", "𛃅"),
        ("5", "𛃆"),
        ("6", "𛃇"),
        ("7", "𛃈"),
        ("8", "𛃖"),
    ]);

    let mi_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛃉"),
        ("2", "𛃊"),
        ("3", "𛃋"),
        ("4", "𛃌"),
        ("5", "𛃍"),
        ("6", "𛃎"),
        ("7", "𛃏"),
    ]);

    let mu_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛃐"),
        ("2", "𛃑"),
        ("3", "𛃒"),
        ("4", "𛃓"),
        ("5", "𛄝"),
        ("6", "𛄞"),
    ]);

    let me_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛃔"),
        ("2", "𛃕"),
        ("3", "𛃖"),
    ]);

    let mo_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛃗"),
        ("2", "𛃘"),
        ("3", "𛃙"),
        ("4", "𛃚"),
        ("5", "𛃛"),
        ("6", "𛃜"),
        ("7", "𛄝"),
        ("8", "𛄞"),
    ]);

    let ya_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛃝"),
        ("2", "𛃞"),
        ("3", "𛃟"),
        ("4", "𛃠"),
        ("5", "𛃡"),
        ("6", "𛃢"),
    ]);

    let yi_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛀆"),
    ]);

    let yu_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛃣"),
        ("2", "𛃤"),
        ("3", "𛃥"),
        ("4", "𛃦"),
    ]);

    let ye_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛀁"),
    ]);

    let yo_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛃧"),
        ("2", "𛃨"),
        ("3", "𛃩"),
        ("4", "𛃪"),
        ("5", "𛃫"),
        ("6", "𛃬"),
        ("7", "𛃢"),
    ]);

    let ra_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛃭"),
        ("2", "𛃮"),
        ("3", "𛃯"),
        ("4", "𛃰"),
        ("5", "𛁽"),
    ]);

    let ri_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛃱"),
        ("2", "𛃲"),
        ("3", "𛃳"),
        ("4", "𛃴"),
        ("5", "𛃵"),
        ("6", "𛃶"),
        ("7", "𛃷"),
    ]);

    let ru_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛃸"),
        ("2", "𛃹"),
        ("3", "𛃺"),
        ("4", "𛃻"),
        ("5", "𛃼"),
        ("6", "𛃽"),
    ]);

    let re_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛃾"),
        ("2", "𛃿"),
        ("3", "𛄀"),
        ("4", "𛄁"),
    ]);

    let ro_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛄂"),
        ("2", "𛄃"),
        ("3", "𛄄"),
        ("4", "𛄅"),
        ("5", "𛄆"),
        ("6", "𛄇"),
    ]);

    let wa_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛄈"),
        ("2", "𛄉"),
        ("3", "𛄊"),
        ("4", "𛄋"),
        ("5", "𛄌"),
    ]);

    let wi_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛄍"),
        ("2", "𛄎"),
        ("3", "𛄏"),
        ("4", "𛄐"),
        ("5", "𛄑"),
    ]);

    let we_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛄒"),
        ("2", "𛄓"),
        ("3", "𛄔"),
        ("4", "𛄕"),
    ]);

    let wo_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛄖"),
        ("2", "𛄗"),
        ("3", "𛄘"),
        ("4", "𛄙"),
        ("5", "𛄚"),
        ("6", "𛄛"),
        ("7", "𛄜"),
        ("8", "𛀅"),
    ]);

    let n_dict: BTreeMap<&str, &str> = BTreeMap::from([
        ("1", "𛄝"),
        ("2", "𛄞"),
    ]);

    let romaji_dict: BTreeMap<&str, &BTreeMap<&str, &str>> = BTreeMap::from ([
        ("a", &a_dict),
        ("i", &i_dict),
        ("u", &u_dict),
        ("e", &e_dict),
        ("o", &o_dict),
        ("ka", &ka_dict),
        ("ki", &ki_dict),
        ("ku", &ku_dict),
        ("ke", &ke_dict),
        ("ko", &ko_dict),
        ("sa", &sa_dict),
        ("shi", &shi_dict),
        ("si", &shi_dict),
        ("su", &su_dict),
        ("se", &se_dict),
        ("so", &so_dict),
        ("ta", &ta_dict),
        ("chi", &chi_dict),
        ("ti", &chi_dict),
        ("tu", &tsu_dict),
        ("tsu", &tsu_dict),
        ("te", &te_dict),
        ("to", &to_dict),
        ("na", &na_dict),
        ("ni", &ni_dict),
        ("nu", &nu_dict),
        ("ne", &ne_dict),
        ("no", &no_dict),
        ("ha", &ha_dict),
        ("hi", &hi_dict),
        ("hu", &fu_dict),
        ("fu", &fu_dict),
        ("he", &he_dict),
        ("ho", &ho_dict),
        ("ma", &ma_dict),
        ("mi", &mi_dict),
        ("mu", &mu_dict),
        ("me", &me_dict),
        ("mo", &mo_dict),
        ("ya", &ya_dict),
        ("yi", &yi_dict),
        ("yu", &yu_dict),
        ("ye", &ye_dict),
        ("yo", &yo_dict),
        ("ra", &ra_dict),
        ("ri", &ri_dict),
        ("ru", &ru_dict),
        ("re", &re_dict),
        ("ro", &ro_dict),
        ("wa", &wa_dict),
        ("wyi", &wi_dict),
        ("wi", &wi_dict),
        ("wye", &we_dict),
        ("we", &we_dict),
        ("wo", &wo_dict),
        ("n", &n_dict),
        ("nn", &n_dict),
        ("ga", &ka_dict),
        ("gi", &ki_dict),
        ("gu", &ku_dict),
        ("ge", &ke_dict),
        ("go", &ko_dict),
        ("za", &sa_dict),
        ("ji", &shi_dict),
        ("zi", &shi_dict),
        ("zu", &su_dict),
        ("ze", &se_dict),
        ("zo", &so_dict),
        ("da", &ta_dict),
        ("di", &chi_dict),
        ("du", &tsu_dict),
        ("de", &te_dict),
        ("do", &to_dict),
        ("ba", &ha_dict),
        ("bi", &hi_dict),
        ("bu", &fu_dict),
        ("be", &he_dict),
        ("bo", &ho_dict),
        ("pa", &ha_dict),
        ("pi", &hi_dict),
        ("pu", &fu_dict),
        ("pe", &he_dict),
        ("po", &ho_dict)
    ]);

    return match romaji_dict.get(romaji) {
        Some(some) => some.to_owned().to_owned(),
        None => BTreeMap::default()
    }
}

fn get_hentaigana(romaji: &str, variant: &str) -> String {
    return match get_hentaigana_group(romaji).get(variant) {
        Some(some) => some.to_owned().to_owned(),
        None => "".to_owned()
    }
}

pub fn get_hentaigana_replace(current_text: String, current_char: String) -> (String, String) {
    let regex_matches = vec![safe_regex_match(r"[A-z]{1,3}$", &current_text), safe_regex_match(r"[A-z]{1,2}$", &current_text), safe_regex_match(r"[A-z]$", &current_text)];
    for regex_match in regex_matches {
        let hentaigana_group = get_hentaigana_group(&regex_match);
        if hentaigana_group != BTreeMap::default() {
            let hentaigana_result = get_hentaigana(&regex_match, &current_char);
            if hentaigana_result != "".to_owned() {
                return (get_hentaigana(&regex_match, &current_char), regex_match);
            }
        }
    }

    return (current_char.clone(), "".to_string());
}

pub fn get_hentaigana_display(current_text: String) -> String {
    let regex_matches = vec![safe_regex_match(r"[A-z]{1,3}$", &current_text), safe_regex_match(r"[A-z]{1,2}$", &current_text), safe_regex_match(r"[A-z]$", &current_text)];

    for regex_match in regex_matches {
        let hentaigana_group = get_hentaigana_group(&regex_match);
        if hentaigana_group != BTreeMap::default() {
            return format_display(hentaigana_group);
        }
    }

    return "".to_owned();
}

fn format_display(btreemap: BTreeMap<&str, &str>) -> String {
    let mut display_string: String = "".to_owned();

    let keys = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "="];
    for key in keys {
        if btreemap.get(key).unwrap_or(&"") != &"" {
            display_string += &format!("{}{}{}{}", key, " ", btreemap.get(key).unwrap_or(&""), "\n");
        }
    }

    return display_string;
}

fn safe_regex_match(regex_string: &str, search_string: &str) -> String {
    let re = Regex::new(regex_string).unwrap();
    return match re.captures(&search_string) {
        Some(some) => match some.get(0) {
            Some(some) => some.as_str().to_owned(),
            None => "".to_owned()
        },
        None => "".to_owned()
    };
}
