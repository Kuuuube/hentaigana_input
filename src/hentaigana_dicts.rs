use regex::Regex;
use std::collections::BTreeMap;

fn get_hentaigana_group(romaji: &str) -> BTreeMap<String, String> {
    let a_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛀂".to_string()),
        ("2".to_string(), "𛀅".to_string()),
        ("3".to_string(), "𛀃".to_string()),
        ("4".to_string(), "𛀄".to_string()),
        ("!".to_string(), "あ".to_string()),
        ("@".to_string(), "ぁ".to_string()),
        ("#".to_string(), "ア".to_string()),
        ("$".to_string(), "ァ".to_string()),
        ("%".to_string(), "ｱ".to_string()),
        ("^".to_string(), "ｧ".to_string()),
    ]);

    let i_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛀆".to_string()),
        ("2".to_string(), "𛀇".to_string()),
        ("3".to_string(), "𛀈".to_string()),
        ("4".to_string(), "𛀉".to_string()),
        ("!".to_string(), "い".to_string()),
        ("@".to_string(), "ぃ".to_string()),
        ("#".to_string(), "イ".to_string()),
        ("$".to_string(), "ィ".to_string()),
        ("%".to_string(), "ｲ".to_string()),
        ("^".to_string(), "ｨ".to_string()),
    ]);

    let u_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛀊".to_string()),
        ("2".to_string(), "𛀋".to_string()),
        ("3".to_string(), "𛀌".to_string()),
        ("4".to_string(), "𛀍".to_string()),
        ("5".to_string(), "𛀎".to_string()),
        ("!".to_string(), "う".to_string()),
        ("@".to_string(), "ぅ".to_string()),
        ("#".to_string(), "ウ".to_string()),
        ("$".to_string(), "ゥ".to_string()),
        ("%".to_string(), "ｳ".to_string()),
        ("^".to_string(), "ｩ".to_string()),
    ]);

    let e_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛀁".to_string()),
        ("2".to_string(), "𛀏".to_string()),
        ("3".to_string(), "𛀐".to_string()),
        ("4".to_string(), "𛀑".to_string()),
        ("5".to_string(), "𛀒".to_string()),
        ("6".to_string(), "𛀓".to_string()),
        ("7".to_string(), "𛀀".to_string()),
        ("!".to_string(), "え".to_string()),
        ("@".to_string(), "ぇ".to_string()),
        ("#".to_string(), "エ".to_string()),
        ("$".to_string(), "ェ".to_string()),
        ("%".to_string(), "ｴ".to_string()),
        ("^".to_string(), "ｪ".to_string()),
    ]);

    let o_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛀔".to_string()),
        ("2".to_string(), "𛀕".to_string()),
        ("3".to_string(), "𛀖".to_string()),
        ("!".to_string(), "お".to_string()),
        ("@".to_string(), "ぉ".to_string()),
        ("#".to_string(), "オ".to_string()),
        ("$".to_string(), "ォ".to_string()),
        ("%".to_string(), "ｵ".to_string()),
        ("^".to_string(), "ｫ".to_string()),
    ]);

    let ka_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛀗".to_string()),
        ("2".to_string(), "𛀘".to_string()),
        ("3".to_string(), "𛀙".to_string()),
        ("4".to_string(), "𛀚".to_string()),
        ("5".to_string(), "𛀛".to_string()),
        ("6".to_string(), "𛀢".to_string()),
        ("7".to_string(), "𛀜".to_string()),
        ("8".to_string(), "𛀝".to_string()),
        ("9".to_string(), "𛀞".to_string()),
        ("0".to_string(), "𛀟".to_string()),
        ("-".to_string(), "𛀠".to_string()),
        ("=".to_string(), "𛀡".to_string()),
        ("!".to_string(), "か".to_string()),
        ("@".to_string(), "ゕ".to_string()),
        ("#".to_string(), "カ".to_string()),
        ("$".to_string(), "ヵ".to_string()),
        ("%".to_string(), "ｶ".to_string()),
        ("^".to_string(), "ヶ".to_string()),
        ("&".to_string(), "🈀".to_string()),
    ]);

    let ki_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛀣".to_string()),
        ("2".to_string(), "𛀤".to_string()),
        ("3".to_string(), "𛀥".to_string()),
        ("4".to_string(), "𛀦".to_string()),
        ("5".to_string(), "𛀻".to_string()),
        ("6".to_string(), "𛀧".to_string()),
        ("7".to_string(), "𛀨".to_string()),
        ("8".to_string(), "𛀩".to_string()),
        ("9".to_string(), "𛀪".to_string()),
        ("!".to_string(), "き".to_string()),
        ("@".to_string(), "キ".to_string()),
        ("#".to_string(), "ｷ".to_string()),
    ]);

    let ku_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛀫".to_string()),
        ("2".to_string(), "𛀬".to_string()),
        ("3".to_string(), "𛀭".to_string()),
        ("4".to_string(), "𛀮".to_string()),
        ("5".to_string(), "𛀯".to_string()),
        ("6".to_string(), "𛀰".to_string()),
        ("7".to_string(), "𛀱".to_string()),
        ("!".to_string(), "く".to_string()),
        ("@".to_string(), "ク".to_string()),
        ("#".to_string(), "ㇰ".to_string()),
        ("$".to_string(), "ｸ".to_string()),
    ]);

    let ke_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛀳".to_string()),
        ("2".to_string(), "𛀲".to_string()),
        ("3".to_string(), "𛀢".to_string()),
        ("4".to_string(), "𛀴".to_string()),
        ("5".to_string(), "𛀵".to_string()),
        ("6".to_string(), "𛀶".to_string()),
        ("7".to_string(), "𛀷".to_string()),
        ("!".to_string(), "け".to_string()),
        ("@".to_string(), "ゖ".to_string()),
        ("#".to_string(), "ケ".to_string()),
        ("$".to_string(), "ｹ".to_string()),
    ]);

    let ko_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛀸".to_string()),
        ("2".to_string(), "𛂘".to_string()),
        ("3".to_string(), "𛀹".to_string()),
        ("4".to_string(), "𛀻".to_string()),
        ("5".to_string(), "𛀺".to_string()),
        ("!".to_string(), "こ".to_string()),
        ("@".to_string(), "𛄲".to_string()),
        ("#".to_string(), "コ".to_string()),
        ("$".to_string(), "ｺ".to_string()),
        ("%".to_string(), "𛅕".to_string()),
        ("^".to_string(), "🈁".to_string()),
    ]);

    let sa_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛀼".to_string()),
        ("2".to_string(), "𛀽".to_string()),
        ("3".to_string(), "𛀾".to_string()),
        ("4".to_string(), "𛀿".to_string()),
        ("5".to_string(), "𛁀".to_string()),
        ("6".to_string(), "𛁁".to_string()),
        ("7".to_string(), "𛁂".to_string()),
        ("8".to_string(), "𛁃".to_string()),
        ("!".to_string(), "さ".to_string()),
        ("@".to_string(), "サ".to_string()),
        ("#".to_string(), "ｻ".to_string()),
        ("$".to_string(), "🈂".to_string()),
    ]);

    let shi_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛁄".to_string()),
        ("2".to_string(), "𛁅".to_string()),
        ("3".to_string(), "𛁆".to_string()),
        ("4".to_string(), "𛁇".to_string()),
        ("5".to_string(), "𛁈".to_string()),
        ("6".to_string(), "𛁉".to_string()),
        ("!".to_string(), "し".to_string()),
        ("@".to_string(), "シ".to_string()),
        ("#".to_string(), "ｼ".to_string()),
        ("$".to_string(), "ㇱ".to_string()),
    ]);

    let su_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛁊".to_string()),
        ("2".to_string(), "𛁋".to_string()),
        ("3".to_string(), "𛁌".to_string()),
        ("4".to_string(), "𛁍".to_string()),
        ("5".to_string(), "𛁎".to_string()),
        ("6".to_string(), "𛁏".to_string()),
        ("7".to_string(), "𛁐".to_string()),
        ("8".to_string(), "𛁑".to_string()),
        ("!".to_string(), "す".to_string()),
        ("@".to_string(), "ス".to_string()),
        ("#".to_string(), "ㇲ".to_string()),
        ("$".to_string(), "ｽ".to_string()),
    ]);

    let se_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛁒".to_string()),
        ("2".to_string(), "𛁓".to_string()),
        ("3".to_string(), "𛁔".to_string()),
        ("4".to_string(), "𛁕".to_string()),
        ("5".to_string(), "𛁖".to_string()),
        ("!".to_string(), "せ".to_string()),
        ("@".to_string(), "セ".to_string()),
        ("#".to_string(), "ｾ".to_string()),
    ]);

    let so_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛁗".to_string()),
        ("2".to_string(), "𛁘".to_string()),
        ("3".to_string(), "𛁙".to_string()),
        ("4".to_string(), "𛁚".to_string()),
        ("5".to_string(), "𛁛".to_string()),
        ("6".to_string(), "𛁜".to_string()),
        ("7".to_string(), "𛁝".to_string()),
        ("!".to_string(), "そ".to_string()),
        ("@".to_string(), "ソ".to_string()),
        ("#".to_string(), "ｿ".to_string()),
    ]);

    let ta_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛁞".to_string()),
        ("2".to_string(), "𛁟".to_string()),
        ("3".to_string(), "𛁠".to_string()),
        ("4".to_string(), "𛁡".to_string()),
        ("!".to_string(), "た".to_string()),
        ("@".to_string(), "タ".to_string()),
        ("#".to_string(), "ﾀ".to_string()),
        ("$".to_string(), "🈕".to_string()),
    ]);

    let chi_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛁢".to_string()),
        ("2".to_string(), "𛁣".to_string()),
        ("3".to_string(), "𛁤".to_string()),
        ("4".to_string(), "𛁥".to_string()),
        ("5".to_string(), "𛁦".to_string()),
        ("6".to_string(), "𛁧".to_string()),
        ("7".to_string(), "𛁨".to_string()),
        ("!".to_string(), "ち".to_string()),
        ("@".to_string(), "チ".to_string()),
        ("#".to_string(), "ﾁ".to_string()),
    ]);

    let tsu_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛁩".to_string()),
        ("2".to_string(), "𛁪".to_string()),
        ("3".to_string(), "𛁫".to_string()),
        ("4".to_string(), "𛁬".to_string()),
        ("5".to_string(), "𛁭".to_string()),
        ("!".to_string(), "つ".to_string()),
        ("@".to_string(), "っ".to_string()),
        ("#".to_string(), "ツ".to_string()),
        ("$".to_string(), "ッ".to_string()),
        ("%".to_string(), "ﾂ".to_string()),
        ("^".to_string(), "ｯ".to_string()),
    ]);

    let te_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛁮".to_string()),
        ("2".to_string(), "𛁯".to_string()),
        ("3".to_string(), "𛁰".to_string()),
        ("4".to_string(), "𛁱".to_string()),
        ("5".to_string(), "𛁲".to_string()),
        ("6".to_string(), "𛁳".to_string()),
        ("7".to_string(), "𛁴".to_string()),
        ("8".to_string(), "𛁵".to_string()),
        ("9".to_string(), "𛁶".to_string()),
        ("0".to_string(), "𛂎".to_string()),
        ("!".to_string(), "て".to_string()),
        ("@".to_string(), "テ".to_string()),
        ("#".to_string(), "ﾃ".to_string()),
        ("$".to_string(), "🈓".to_string()),
    ]);

    let to_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛁷".to_string()),
        ("2".to_string(), "𛁸".to_string()),
        ("3".to_string(), "𛁹".to_string()),
        ("4".to_string(), "𛁺".to_string()),
        ("5".to_string(), "𛁻".to_string()),
        ("6".to_string(), "𛁼".to_string()),
        ("7".to_string(), "𛁽".to_string()),
        ("8".to_string(), "𛁭".to_string()),
        ("!".to_string(), "と".to_string()),
        ("@".to_string(), "ト".to_string()),
        ("#".to_string(), "ㇳ".to_string()),
        ("$".to_string(), "ﾄ".to_string()),
    ]);

    let na_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛁾".to_string()),
        ("2".to_string(), "𛁿".to_string()),
        ("3".to_string(), "𛂀".to_string()),
        ("4".to_string(), "𛂁".to_string()),
        ("5".to_string(), "𛂂".to_string()),
        ("6".to_string(), "𛂃".to_string()),
        ("7".to_string(), "𛂄".to_string()),
        ("8".to_string(), "𛂅".to_string()),
        ("9".to_string(), "𛂆".to_string()),
        ("!".to_string(), "な".to_string()),
        ("@".to_string(), "ナ".to_string()),
        ("#".to_string(), "ﾅ".to_string()),
    ]);

    let ni_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛂇".to_string()),
        ("2".to_string(), "𛂈".to_string()),
        ("3".to_string(), "𛂉".to_string()),
        ("4".to_string(), "𛂊".to_string()),
        ("5".to_string(), "𛂋".to_string()),
        ("6".to_string(), "𛂌".to_string()),
        ("7".to_string(), "𛂍".to_string()),
        ("8".to_string(), "𛂎".to_string()),
        ("!".to_string(), "に".to_string()),
        ("@".to_string(), "ニ".to_string()),
        ("#".to_string(), "ﾆ".to_string()),
        ("$".to_string(), "🈔".to_string()),
        ("%".to_string(), "🉂".to_string()),
    ]);

    let nu_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛂏".to_string()),
        ("2".to_string(), "𛂐".to_string()),
        ("3".to_string(), "𛂑".to_string()),
        ("!".to_string(), "ぬ".to_string()),
        ("@".to_string(), "ヌ".to_string()),
        ("#".to_string(), "ㇴ".to_string()),
        ("$".to_string(), "ﾇ".to_string()),
    ]);

    let ne_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛂒".to_string()),
        ("2".to_string(), "𛂓".to_string()),
        ("3".to_string(), "𛂔".to_string()),
        ("4".to_string(), "𛂕".to_string()),
        ("5".to_string(), "𛂖".to_string()),
        ("6".to_string(), "𛂗".to_string()),
        ("7".to_string(), "𛂘".to_string()),
        ("!".to_string(), "ね".to_string()),
        ("@".to_string(), "ネ".to_string()),
        ("#".to_string(), "ﾈ".to_string()),
    ]);

    let no_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛂙".to_string()),
        ("2".to_string(), "𛂚".to_string()),
        ("3".to_string(), "𛂛".to_string()),
        ("4".to_string(), "𛂜".to_string()),
        ("5".to_string(), "𛂝".to_string()),
        ("!".to_string(), "の".to_string()),
        ("@".to_string(), "ノ".to_string()),
        ("#".to_string(), "ﾉ".to_string()),
    ]);

    let ha_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛂞".to_string()),
        ("2".to_string(), "𛂟".to_string()),
        ("3".to_string(), "𛂠".to_string()),
        ("4".to_string(), "𛂡".to_string()),
        ("5".to_string(), "𛂢".to_string()),
        ("6".to_string(), "𛂣".to_string()),
        ("7".to_string(), "𛂤".to_string()),
        ("8".to_string(), "𛂥".to_string()),
        ("9".to_string(), "𛂦".to_string()),
        ("0".to_string(), "𛂧".to_string()),
        ("-".to_string(), "𛂨".to_string()),
        ("!".to_string(), "は".to_string()),
        ("@".to_string(), "ハ".to_string()),
        ("#".to_string(), "ㇵ".to_string()),
        ("$".to_string(), "ﾊ".to_string()),
    ]);

    let hi_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛂩".to_string()),
        ("2".to_string(), "𛂪".to_string()),
        ("3".to_string(), "𛂫".to_string()),
        ("4".to_string(), "𛂬".to_string()),
        ("5".to_string(), "𛂭".to_string()),
        ("6".to_string(), "𛂮".to_string()),
        ("7".to_string(), "𛂯".to_string()),
        ("!".to_string(), "ひ".to_string()),
        ("@".to_string(), "ヒ".to_string()),
        ("#".to_string(), "ㇶ".to_string()),
        ("$".to_string(), "ﾋ".to_string()),
    ]);

    let fu_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛂰".to_string()),
        ("2".to_string(), "𛂱".to_string()),
        ("3".to_string(), "𛂲".to_string()),
        ("!".to_string(), "ふ".to_string()),
        ("@".to_string(), "フ".to_string()),
        ("#".to_string(), "ㇷ".to_string()),
        ("$".to_string(), "ﾌ".to_string()),
    ]);

    let he_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛂳".to_string()),
        ("2".to_string(), "𛂴".to_string()),
        ("3".to_string(), "𛂵".to_string()),
        ("4".to_string(), "𛂶".to_string()),
        ("5".to_string(), "𛂷".to_string()),
        ("6".to_string(), "𛂸".to_string()),
        ("7".to_string(), "𛂹".to_string()),
        ("!".to_string(), "へ".to_string()),
        ("@".to_string(), "ヘ".to_string()),
        ("#".to_string(), "ㇸ".to_string()),
        ("$".to_string(), "ﾍ".to_string()),
    ]);

    let ho_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛂺".to_string()),
        ("2".to_string(), "𛂻".to_string()),
        ("3".to_string(), "𛂼".to_string()),
        ("4".to_string(), "𛂽".to_string()),
        ("5".to_string(), "𛂾".to_string()),
        ("6".to_string(), "𛂿".to_string()),
        ("7".to_string(), "𛃀".to_string()),
        ("8".to_string(), "𛃁".to_string()),
        ("!".to_string(), "ほ".to_string()),
        ("@".to_string(), "ホ".to_string()),
        ("#".to_string(), "ㇹ".to_string()),
        ("$".to_string(), "ﾎ".to_string()),
    ]);

    let ma_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛃂".to_string()),
        ("2".to_string(), "𛃃".to_string()),
        ("3".to_string(), "𛃄".to_string()),
        ("4".to_string(), "𛃅".to_string()),
        ("5".to_string(), "𛃆".to_string()),
        ("6".to_string(), "𛃇".to_string()),
        ("7".to_string(), "𛃈".to_string()),
        ("8".to_string(), "𛃖".to_string()),
        ("!".to_string(), "ま".to_string()),
        ("@".to_string(), "マ".to_string()),
        ("#".to_string(), "ﾏ".to_string()),
    ]);

    let mi_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛃉".to_string()),
        ("2".to_string(), "𛃊".to_string()),
        ("3".to_string(), "𛃋".to_string()),
        ("4".to_string(), "𛃌".to_string()),
        ("5".to_string(), "𛃍".to_string()),
        ("6".to_string(), "𛃎".to_string()),
        ("7".to_string(), "𛃏".to_string()),
        ("!".to_string(), "み".to_string()),
        ("@".to_string(), "ミ".to_string()),
        ("#".to_string(), "ﾐ".to_string()),
    ]);

    let mu_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛃐".to_string()),
        ("2".to_string(), "𛃑".to_string()),
        ("3".to_string(), "𛃒".to_string()),
        ("4".to_string(), "𛃓".to_string()),
        ("5".to_string(), "𛄝".to_string()),
        ("6".to_string(), "𛄞".to_string()),
        ("!".to_string(), "む".to_string()),
        ("@".to_string(), "ム".to_string()),
        ("#".to_string(), "ㇺ".to_string()),
        ("$".to_string(), "ﾑ".to_string()),
    ]);

    let me_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛃔".to_string()),
        ("2".to_string(), "𛃕".to_string()),
        ("3".to_string(), "𛃖".to_string()),
        ("!".to_string(), "め".to_string()),
        ("@".to_string(), "メ".to_string()),
        ("#".to_string(), "ﾒ".to_string()),
    ]);

    let mo_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛃗".to_string()),
        ("2".to_string(), "𛃘".to_string()),
        ("3".to_string(), "𛃙".to_string()),
        ("4".to_string(), "𛃚".to_string()),
        ("5".to_string(), "𛃛".to_string()),
        ("6".to_string(), "𛃜".to_string()),
        ("7".to_string(), "𛄝".to_string()),
        ("8".to_string(), "𛄞".to_string()),
        ("!".to_string(), "も".to_string()),
        ("@".to_string(), "モ".to_string()),
        ("#".to_string(), "ﾓ".to_string()),
    ]);

    let ya_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛃝".to_string()),
        ("2".to_string(), "𛃞".to_string()),
        ("3".to_string(), "𛃟".to_string()),
        ("4".to_string(), "𛃠".to_string()),
        ("5".to_string(), "𛃡".to_string()),
        ("6".to_string(), "𛃢".to_string()),
        ("!".to_string(), "や".to_string()),
        ("@".to_string(), "ゃ".to_string()),
        ("#".to_string(), "ヤ".to_string()),
        ("$".to_string(), "ャ".to_string()),
        ("%".to_string(), "ﾔ".to_string()),
        ("^".to_string(), "ｬ".to_string()),
    ]);

    let yi_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛀆".to_string()),
        ("2".to_string(), "𛄠".to_string()),
    ]);

    let yu_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛃣".to_string()),
        ("2".to_string(), "𛃤".to_string()),
        ("3".to_string(), "𛃥".to_string()),
        ("4".to_string(), "𛃦".to_string()),
        ("!".to_string(), "ゆ".to_string()),
        ("@".to_string(), "ゅ".to_string()),
        ("#".to_string(), "ユ".to_string()),
        ("$".to_string(), "ュ".to_string()),
        ("%".to_string(), "ﾕ".to_string()),
        ("^".to_string(), "ｭ".to_string()),
    ]);

    let ye_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛀁".to_string()),
        ("2".to_string(), "𛄡".to_string()),
    ]);

    let yo_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛃧".to_string()),
        ("2".to_string(), "𛃨".to_string()),
        ("3".to_string(), "𛃩".to_string()),
        ("4".to_string(), "𛃪".to_string()),
        ("5".to_string(), "𛃫".to_string()),
        ("6".to_string(), "𛃬".to_string()),
        ("7".to_string(), "𛃢".to_string()),
        ("!".to_string(), "よ".to_string()),
        ("@".to_string(), "ょ".to_string()),
        ("#".to_string(), "ヨ".to_string()),
        ("$".to_string(), "ョ".to_string()),
        ("%".to_string(), "ﾖ".to_string()),
        ("^".to_string(), "ｮ".to_string()),
    ]);

    let ra_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛃭".to_string()),
        ("2".to_string(), "𛃮".to_string()),
        ("3".to_string(), "𛃯".to_string()),
        ("4".to_string(), "𛃰".to_string()),
        ("5".to_string(), "𛁽".to_string()),
        ("!".to_string(), "ら".to_string()),
        ("@".to_string(), "ラ".to_string()),
        ("#".to_string(), "ㇻ".to_string()),
        ("$".to_string(), "ﾗ".to_string()),
    ]);

    let ri_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛃱".to_string()),
        ("2".to_string(), "𛃲".to_string()),
        ("3".to_string(), "𛃳".to_string()),
        ("4".to_string(), "𛃴".to_string()),
        ("5".to_string(), "𛃵".to_string()),
        ("6".to_string(), "𛃶".to_string()),
        ("7".to_string(), "𛃷".to_string()),
        ("!".to_string(), "り".to_string()),
        ("@".to_string(), "リ".to_string()),
        ("#".to_string(), "ㇼ".to_string()),
        ("$".to_string(), "ﾘ".to_string()),
    ]);

    let ru_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛃸".to_string()),
        ("2".to_string(), "𛃹".to_string()),
        ("3".to_string(), "𛃺".to_string()),
        ("4".to_string(), "𛃻".to_string()),
        ("5".to_string(), "𛃼".to_string()),
        ("6".to_string(), "𛃽".to_string()),
        ("!".to_string(), "る".to_string()),
        ("@".to_string(), "ル".to_string()),
        ("#".to_string(), "ㇽ".to_string()),
        ("$".to_string(), "ﾙ".to_string()),
    ]);

    let re_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛃾".to_string()),
        ("2".to_string(), "𛃿".to_string()),
        ("3".to_string(), "𛄀".to_string()),
        ("4".to_string(), "𛄁".to_string()),
        ("!".to_string(), "れ".to_string()),
        ("@".to_string(), "レ".to_string()),
        ("#".to_string(), "ㇾ".to_string()),
        ("$".to_string(), "ﾚ".to_string()),
    ]);

    let ro_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛄂".to_string()),
        ("2".to_string(), "𛄃".to_string()),
        ("3".to_string(), "𛄄".to_string()),
        ("4".to_string(), "𛄅".to_string()),
        ("5".to_string(), "𛄆".to_string()),
        ("6".to_string(), "𛄇".to_string()),
        ("!".to_string(), "ろ".to_string()),
        ("@".to_string(), "ロ".to_string()),
        ("#".to_string(), "ㇿ".to_string()),
        ("$".to_string(), "ﾛ".to_string()),
    ]);

    let wa_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛄈".to_string()),
        ("2".to_string(), "𛄉".to_string()),
        ("3".to_string(), "𛄊".to_string()),
        ("4".to_string(), "𛄋".to_string()),
        ("5".to_string(), "𛄌".to_string()),
        ("!".to_string(), "わ".to_string()),
        ("@".to_string(), "ゎ".to_string()),
        ("#".to_string(), "ワ".to_string()),
        ("$".to_string(), "ヮ".to_string()),
        ("%".to_string(), "ﾜ".to_string()),
    ]);

    let wi_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛄍".to_string()),
        ("2".to_string(), "𛄎".to_string()),
        ("3".to_string(), "𛄏".to_string()),
        ("4".to_string(), "𛄐".to_string()),
        ("5".to_string(), "𛄑".to_string()),
        ("!".to_string(), "ゐ".to_string()),
        ("@".to_string(), "𛅐".to_string()),
        ("#".to_string(), "ヰ".to_string()),
        ("$".to_string(), "𛅤".to_string()),
        ("%".to_string(), "ﾜ".to_string()),
    ]);

    let wu_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛄟".to_string()),
        ("2".to_string(), "𛄢".to_string()),
    ]);

    let we_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛄒".to_string()),
        ("2".to_string(), "𛄓".to_string()),
        ("3".to_string(), "𛄔".to_string()),
        ("4".to_string(), "𛄕".to_string()),
        ("!".to_string(), "ゑ".to_string()),
        ("@".to_string(), "𛅑".to_string()),
        ("#".to_string(), "ヱ".to_string()),
        ("$".to_string(), "𛅥".to_string()),
    ]);

    let wo_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛄖".to_string()),
        ("2".to_string(), "𛄗".to_string()),
        ("3".to_string(), "𛄘".to_string()),
        ("4".to_string(), "𛄙".to_string()),
        ("5".to_string(), "𛄚".to_string()),
        ("6".to_string(), "𛄛".to_string()),
        ("7".to_string(), "𛄜".to_string()),
        ("8".to_string(), "𛀅".to_string()),
        ("!".to_string(), "を".to_string()),
        ("@".to_string(), "𛅒".to_string()),
        ("#".to_string(), "ヲ".to_string()),
        ("$".to_string(), "𛅦".to_string()),
    ]);

    let n_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𛄝".to_string()),
        ("2".to_string(), "𛄞".to_string()),
        ("!".to_string(), "ん".to_string()),
        ("@".to_string(), "ン".to_string()),
        ("#".to_string(), "𛅧".to_string()),
        ("$".to_string(), "ﾝ".to_string()),
    ]);

    let yori_dict: BTreeMap<String, String> = BTreeMap::from([("1".to_string(), "ゟ".to_string())]);

    let koto_dict: BTreeMap<String, String> = BTreeMap::from([("1".to_string(), "ヿ".to_string())]);

    let shime_dict: BTreeMap<String, String> =
        BTreeMap::from([("1".to_string(), "〆".to_string())]);

    let tomo_dict: BTreeMap<String, String> = BTreeMap::from([("1".to_string(), "𪜈".to_string())]);

    let shite_dict: BTreeMap<String, String> =
        BTreeMap::from([("1".to_string(), "𬼀".to_string())]);

    let masu_dict: BTreeMap<String, String> = BTreeMap::from([("1".to_string(), "〼".to_string())]);

    let nari_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "𬼂".to_string()),
        ("2".to_string(), "𬻿".to_string()),
    ]);

    let iter_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "々".to_string()),
        ("2".to_string(), "〻".to_string()),
        ("3".to_string(), "ゝ".to_string()),
        ("4".to_string(), "ゞ".to_string()),
        ("5".to_string(), "ゝ゚".to_string()),
        ("6".to_string(), "ヽ".to_string()),
        ("7".to_string(), "ヾ".to_string()),
        ("8".to_string(), "〱".to_string()),
        ("9".to_string(), "〲".to_string()),
        ("0".to_string(), "〱゚".to_string()),
        ("-".to_string(), "〳".to_string()),
        ("=".to_string(), "〴".to_string()),
        ("!".to_string(), "〵".to_string()),
        ("@".to_string(), "／".to_string()),
        ("#".to_string(), "＼".to_string()),
        ("$".to_string(), "〃".to_string()),
    ]);

    let brackets_left_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "＜".to_string()),
        ("2".to_string(), "『".to_string()),
        ("3".to_string(), "【".to_string()),
        ("4".to_string(), "“".to_string()),
        ("5".to_string(), "〝".to_string()),
        ("6".to_string(), "〝".to_string()),
        ("7".to_string(), "｛".to_string()),
        ("8".to_string(), "「".to_string()),
        ("9".to_string(), "《".to_string()),
        ("0".to_string(), "｛".to_string()),
        ("-".to_string(), "≪".to_string()),
        ("=".to_string(), "〔".to_string()),
        ("!".to_string(), "‹".to_string()),
        ("@".to_string(), "«".to_string()),
        ("#".to_string(), "｟".to_string()),
        ("$".to_string(), "〈".to_string()),
        ("%".to_string(), "〖".to_string()),
        ("^".to_string(), "〘".to_string()),
    ]);

    let brackets_right_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "＞".to_string()),
        ("2".to_string(), "』".to_string()),
        ("3".to_string(), "】".to_string()),
        ("4".to_string(), "”".to_string()),
        ("5".to_string(), "〟".to_string()),
        ("6".to_string(), "〞".to_string()),
        ("7".to_string(), "｝".to_string()),
        ("8".to_string(), "」".to_string()),
        ("9".to_string(), "》".to_string()),
        ("0".to_string(), "｝".to_string()),
        ("-".to_string(), "≫".to_string()),
        ("=".to_string(), "〕".to_string()),
        ("!".to_string(), "›".to_string()),
        ("@".to_string(), "»".to_string()),
        ("#".to_string(), "｠".to_string()),
        ("$".to_string(), "〉".to_string()),
        ("%".to_string(), "〗".to_string()),
        ("^".to_string(), "〙".to_string()),
    ]);

    let quotes_dict: BTreeMap<String, String> = BTreeMap::from([
        ("1".to_string(), "“".to_string()),
        ("2".to_string(), "”".to_string()),
        ("3".to_string(), "〝".to_string()),
        ("4".to_string(), "〞".to_string()),
        ("5".to_string(), "〟".to_string()),
        ("6".to_string(), "「".to_string()),
        ("7".to_string(), "」".to_string()),
        ("8".to_string(), "『".to_string()),
        ("9".to_string(), "』".to_string()),
        ("0".to_string(), "【".to_string()),
        ("-".to_string(), "】".to_string()),
    ]);

    let dashes_dict: BTreeMap<String, String> = BTreeMap::from([
        ("!".to_string(), "ー".to_string()),
        ("@".to_string(), "ｰ".to_string()),
        ("#".to_string(), "〰".to_string()),
        ("$".to_string(), "〜".to_string()),
        ("%".to_string(), "―".to_string()),
        ("^".to_string(), "～".to_string()),
        ("&".to_string(), "￣".to_string()),
        ("*".to_string(), "⁻".to_string()),
        ("(".to_string(), "₋".to_string()),
        (")".to_string(), "-".to_string()),
        ("_".to_string(), "〽".to_string()),
    ]);

    let dots_dict: BTreeMap<String, String> = BTreeMap::from([
        ("!".to_string(), "゙".to_string()),
        ("@".to_string(), "゚".to_string()),
        ("#".to_string(), "゛".to_string()),
        ("$".to_string(), "゜".to_string()),
        ("%".to_string(), "・".to_string()),
        ("^".to_string(), "、".to_string()),
        ("&".to_string(), "。".to_string()),
        ("^".to_string(), "、".to_string()),
    ]);

    let dhyphen_dict: BTreeMap<String, String> =
        BTreeMap::from([("!".to_string(), "゠".to_string())]);

    let romaji_dict: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::from([
        ("a".to_string(), a_dict.to_owned()),
        ("i".to_string(), i_dict.to_owned()),
        ("u".to_string(), u_dict.to_owned()),
        ("e".to_string(), e_dict.to_owned()),
        ("o".to_string(), o_dict.to_owned()),
        ("ka".to_string(), ka_dict.to_owned()),
        ("ki".to_string(), ki_dict.to_owned()),
        ("ku".to_string(), ku_dict.to_owned()),
        ("ke".to_string(), ke_dict.to_owned()),
        ("ko".to_string(), ko_dict.to_owned()),
        ("sa".to_string(), sa_dict.to_owned()),
        ("shi".to_string(), shi_dict.to_owned()),
        ("si".to_string(), shi_dict.to_owned()),
        ("su".to_string(), su_dict.to_owned()),
        ("se".to_string(), se_dict.to_owned()),
        ("so".to_string(), so_dict.to_owned()),
        ("ta".to_string(), ta_dict.to_owned()),
        ("chi".to_string(), chi_dict.to_owned()),
        ("ti".to_string(), chi_dict.to_owned()),
        ("tu".to_string(), tsu_dict.to_owned()),
        ("tsu".to_string(), tsu_dict.to_owned()),
        ("te".to_string(), te_dict.to_owned()),
        ("to".to_string(), to_dict.to_owned()),
        ("na".to_string(), na_dict.to_owned()),
        ("ni".to_string(), ni_dict.to_owned()),
        ("nu".to_string(), nu_dict.to_owned()),
        ("ne".to_string(), ne_dict.to_owned()),
        ("no".to_string(), no_dict.to_owned()),
        ("ha".to_string(), ha_dict.to_owned()),
        ("hi".to_string(), hi_dict.to_owned()),
        ("hu".to_string(), fu_dict.to_owned()),
        ("fu".to_string(), fu_dict.to_owned()),
        ("he".to_string(), he_dict.to_owned()),
        ("ho".to_string(), ho_dict.to_owned()),
        ("ma".to_string(), ma_dict.to_owned()),
        ("mi".to_string(), mi_dict.to_owned()),
        ("mu".to_string(), mu_dict.to_owned()),
        ("me".to_string(), me_dict.to_owned()),
        ("mo".to_string(), mo_dict.to_owned()),
        ("ya".to_string(), ya_dict.to_owned()),
        ("yi".to_string(), yi_dict.to_owned()),
        ("yu".to_string(), yu_dict.to_owned()),
        ("ye".to_string(), ye_dict.to_owned()),
        ("yo".to_string(), yo_dict.to_owned()),
        ("ra".to_string(), ra_dict.to_owned()),
        ("ri".to_string(), ri_dict.to_owned()),
        ("ru".to_string(), ru_dict.to_owned()),
        ("re".to_string(), re_dict.to_owned()),
        ("ro".to_string(), ro_dict.to_owned()),
        ("wa".to_string(), wa_dict.to_owned()),
        ("wyi".to_string(), wi_dict.to_owned()),
        ("wi".to_string(), wi_dict.to_owned()),
        ("wu".to_string(), wu_dict.to_owned()),
        ("wye".to_string(), we_dict.to_owned()),
        ("we".to_string(), we_dict.to_owned()),
        ("wo".to_string(), wo_dict.to_owned()),
        ("n".to_string(), n_dict.to_owned()),
        ("nn".to_string(), n_dict.to_owned()),
        ("ga".to_string(), ka_dict.to_owned()),
        ("gi".to_string(), ki_dict.to_owned()),
        ("gu".to_string(), ku_dict.to_owned()),
        ("ge".to_string(), ke_dict.to_owned()),
        ("go".to_string(), ko_dict.to_owned()),
        ("za".to_string(), sa_dict.to_owned()),
        ("ji".to_string(), shi_dict.to_owned()),
        ("zi".to_string(), shi_dict.to_owned()),
        ("zu".to_string(), su_dict.to_owned()),
        ("ze".to_string(), se_dict.to_owned()),
        ("zo".to_string(), so_dict.to_owned()),
        ("da".to_string(), ta_dict.to_owned()),
        ("di".to_string(), chi_dict.to_owned()),
        ("du".to_string(), tsu_dict.to_owned()),
        ("de".to_string(), te_dict.to_owned()),
        ("do".to_string(), to_dict.to_owned()),
        ("ba".to_string(), ha_dict.to_owned()),
        ("bi".to_string(), hi_dict.to_owned()),
        ("bu".to_string(), fu_dict.to_owned()),
        ("be".to_string(), he_dict.to_owned()),
        ("bo".to_string(), ho_dict.to_owned()),
        ("pa".to_string(), ha_dict.to_owned()),
        ("pi".to_string(), hi_dict.to_owned()),
        ("pu".to_string(), fu_dict.to_owned()),
        ("pe".to_string(), he_dict.to_owned()),
        ("po".to_string(), ho_dict.to_owned()),
        ("vu".to_string(), u_dict.to_owned()),
        ("va".to_string(), wa_dict.to_owned()),
        ("vi".to_string(), wi_dict.to_owned()),
        ("ve".to_string(), we_dict.to_owned()),
        ("vo".to_string(), wo_dict.to_owned()),
        ("yori".to_string(), yori_dict.to_owned()),
        ("koto".to_string(), koto_dict.to_owned()),
        ("shime".to_string(), shime_dict.to_owned()),
        ("tomo".to_string(), tomo_dict.to_owned()),
        ("goto".to_string(), koto_dict.to_owned()),
        ("shite".to_string(), shite_dict.to_owned()),
        ("masu".to_string(), masu_dict.to_owned()),
        ("nari".to_string(), nari_dict.to_owned()),
        ("iter".to_string(), iter_dict.to_owned()),
        ("noma".to_string(), iter_dict.to_owned()),
        ("rep".to_string(), iter_dict.to_owned()),
        ("kuno".to_string(), iter_dict.to_owned()),
        ("choo".to_string(), iter_dict.to_owned()),
        ("odor".to_string(), iter_dict.to_owned()),
        ("(".to_string(), brackets_left_dict.to_owned()),
        (")".to_string(), brackets_right_dict.to_owned()),
        ("<".to_string(), brackets_left_dict.to_owned()),
        (">".to_string(), brackets_right_dict.to_owned()),
        ("{".to_string(), brackets_left_dict.to_owned()),
        ("}".to_string(), brackets_right_dict.to_owned()),
        ("\"".to_string(), quotes_dict.to_owned()),
        ("\'".to_string(), quotes_dict.to_owned()),
        (".".to_string(), dots_dict.to_owned()),
        (",".to_string(), dots_dict.to_owned()),
        ("-".to_string(), dashes_dict.to_owned()),
        ("~".to_string(), dashes_dict.to_owned()),
        ("=".to_string(), dhyphen_dict.to_owned()),
    ]);

    let lowered_romaji = romaji.to_lowercase();

    let output_dict = match romaji_dict.get(&lowered_romaji) {
        Some(some) => some.to_owned(),
        None => BTreeMap::default(),
    };

    let dakuten = vec![
        "ga".to_string(),
        "gi".to_string(),
        "gu".to_string(),
        "ge".to_string(),
        "go".to_string(),
        "za".to_string(),
        "ji".to_string(),
        "zi".to_string(),
        "zu".to_string(),
        "ze".to_string(),
        "zo".to_string(),
        "da".to_string(),
        "di".to_string(),
        "du".to_string(),
        "de".to_string(),
        "do".to_string(),
        "ba".to_string(),
        "bi".to_string(),
        "bu".to_string(),
        "be".to_string(),
        "bo".to_string(),
        "vu".to_string(),
        "goto".to_string(),
    ];
    if dakuten.contains(&lowered_romaji) {
        let mut new_dict: BTreeMap<String, String> = Default::default();
        for (key, value) in output_dict.iter() {
            let dakuten_append = format!("{}{}", value, "゙");
            new_dict.insert(key.to_string(), dakuten_append);
        }
        return new_dict;
    }

    let handakuten = vec![
        "pa".to_string(),
        "pi".to_string(),
        "pu".to_string(),
        "pe".to_string(),
        "po".to_string(),
    ];
    if handakuten.contains(&lowered_romaji) {
        let mut new_dict: BTreeMap<String, String> = Default::default();
        for (key, value) in output_dict.iter() {
            let handakuten_append = format!("{}{}", value, "゚");
            new_dict.insert(key.to_string(), handakuten_append);
        }
        return new_dict;
    }

    let mut new_dict: BTreeMap<String, String> = Default::default();
    for (key, value) in output_dict.iter() {
        new_dict.insert(key.to_string(), value.to_string());
    }

    return new_dict;
}

fn get_hentaigana(romaji: &str, variant: &str) -> String {
    return match get_hentaigana_group(romaji).get(variant) {
        Some(some) => some.to_string(),
        None => "".to_owned(),
    };
}

pub fn get_hentaigana_replace(current_text: String, current_char: String) -> (String, String) {
    let regex_matches = vec![
        safe_regex_match(r".{5}$", &current_text),
        safe_regex_match(r".{4}$", &current_text),
        safe_regex_match(r".{3}$", &current_text),
        safe_regex_match(r".{2}$", &current_text),
        safe_regex_match(r".$", &current_text),
    ];
    for regex_match in regex_matches {
        let hentaigana_group = get_hentaigana_group(&regex_match);
        if hentaigana_group != BTreeMap::default() {
            let hentaigana_result = get_hentaigana(&regex_match, &current_char);
            if hentaigana_result != "".to_owned() {
                return (
                    get_hentaigana(&regex_match, &current_char),
                    create_dotstring(regex_match),
                );
            }
        }
    }

    return ("No_match_found.".to_string(), "No_match_found.".to_string());
}

pub fn get_hentaigana_display(current_text: String) -> Vec<(HentaiganaDisplay, HentaiganaDisplay)> {
    let regex_matches = vec![
        safe_regex_match(r".{5}$", &current_text),
        safe_regex_match(r".{4}$", &current_text),
        safe_regex_match(r".{3}$", &current_text),
        safe_regex_match(r".{2}$", &current_text),
        safe_regex_match(r".$", &current_text),
    ];

    for regex_match in regex_matches {
        let hentaigana_group = get_hentaigana_group(&regex_match);
        if hentaigana_group != BTreeMap::default() {
            return format_display(hentaigana_group);
        }
    }

    return vec![];
}

fn format_display(
    btreemap: BTreeMap<String, String>,
) -> Vec<(HentaiganaDisplay, HentaiganaDisplay)> {
    let mut display_values = vec![];

    let mut keys = vec![
        "1", "!", "2", "@", "3", "#", "4", "$", "5", "%", "6", "^", "7", "&", "8", "*", "9", "(",
        "0", ")", "-", "_", "=", "+",
    ]
    .into_iter();

    loop {
        let key1 = match keys.next() {
            Some(some) => some,
            None => break,
        };
        let key2 = match keys.next() {
            Some(some) => some,
            None => "",
        };
        display_values.push((
            HentaiganaDisplay {
                left: key1.to_owned(),
                right: btreemap.get(key1).unwrap_or(&"".to_owned()).to_owned(),
            },
            HentaiganaDisplay {
                left: key2.to_owned(),
                right: btreemap.get(key2).unwrap_or(&"".to_owned()).to_owned(),
            },
        ));
    }

    return display_values;
}

fn safe_regex_match(regex_string: &str, search_string: &str) -> String {
    let re = Regex::new(regex_string).unwrap();
    return match re.captures(&search_string) {
        Some(some) => match some.get(0) {
            Some(some) => some.as_str().to_owned(),
            None => "".to_owned(),
        },
        None => "".to_owned(),
    };
}

fn create_dotstring(input_string: String) -> String {
    let mut new_string = "".to_string();
    for _ in 0..input_string.len() {
        new_string += ".";
    }
    return new_string;
}

#[derive(Debug)]
pub struct HentaiganaDisplay {
    pub left: String,
    pub right: String,
}
