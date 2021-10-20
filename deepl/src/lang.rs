use serde::{Deserialize, Serialize};
use strum::EnumString;

#[derive(EnumString, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Lang {
    #[strum(serialize = "auto")]
    #[serde(rename = "auto")]
    Auto, // auto detect

    DE,
    EN,
    ES,
    FR,
    IT,
    JA,
    NL,
    PL,
    PT,
    RU,
    ZH,
    BG,
    CS,
    DA,
    EL,
    ET,
    FI,
    HU,
    LT,
    LV,
    RO,
    SK,
    SL,
    SV,
}

impl Default for Lang {
    fn default() -> Self {
        Lang::Auto
    }
}

impl Lang {
    pub fn description(&self) -> &'static str {
        let lang = *self;
        match lang {
            Lang::Auto => "自动检测",
            Lang::DE => "德语",
            Lang::EN => "英语",
            Lang::ES => "西班牙语",
            Lang::FR => "法语",
            Lang::IT => "意大利语",
            Lang::JA => "日语",
            Lang::NL => "荷兰语",
            Lang::PL => "波兰语",
            Lang::PT => "葡萄牙语",
            Lang::RU => "俄语",
            Lang::ZH => "中文",
            Lang::BG => "保加利亚语",
            Lang::CS => "捷克语",
            Lang::DA => "丹麦语",
            Lang::EL => "希腊语",
            Lang::ET => "爱沙尼亚语",
            Lang::FI => "芬兰语",
            Lang::HU => "匈牙利语",
            Lang::LT => "立陶宛语",
            Lang::LV => "拉脱维亚语",
            Lang::RO => "罗马尼亚语",
            Lang::SK => "斯洛伐克语",
            Lang::SL => "斯洛文尼亚语",
            Lang::SV => "瑞典语",
        }
    }

    pub fn lang_list_with_auto() -> Vec<Lang> {
        vec![
            Lang::Auto,
            Lang::EN,
            Lang::ZH,
            Lang::ES,
            Lang::FR,
            Lang::IT,
            Lang::JA,
            Lang::NL,
            Lang::PL,
            Lang::PT,
            Lang::RU,
            Lang::ZH,
            Lang::BG,
            Lang::CS,
            Lang::DA,
            Lang::EL,
            Lang::ET,
            Lang::FI,
            Lang::HU,
            Lang::LT,
            Lang::LV,
            Lang::RO,
            Lang::SK,
            Lang::SL,
            Lang::SV,
        ]
    }

    pub fn lang_list() -> Vec<Lang> {
        vec![
            Lang::ZH,
            Lang::EN,
            Lang::ES,
            Lang::FR,
            Lang::IT,
            Lang::JA,
            Lang::NL,
            Lang::PL,
            Lang::PT,
            Lang::RU,
            Lang::ZH,
            Lang::BG,
            Lang::CS,
            Lang::DA,
            Lang::EL,
            Lang::ET,
            Lang::FI,
            Lang::HU,
            Lang::LT,
            Lang::LV,
            Lang::RO,
            Lang::SK,
            Lang::SL,
            Lang::SV,
        ]
    }
}
