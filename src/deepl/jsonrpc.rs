use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRPCRequest {
    id: u128,
    jsonrpc: String,
    method: Method,
    params: Params,
}

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

impl JsonRPCRequest {
    pub fn new(lang_from: Lang, lang_to: Lang, text: String) -> Self {
        let timestamp = get_epoch_ms();
        JsonRPCRequest {
            id: timestamp,
            jsonrpc: "2.0".to_string(),
            method: Method::LMTHandleJobs,
            params: Params {
                jobs: vec![ParamsJob::new(text)],
                lang: ParamsLang::new(lang_from, lang_to),
                priority: 1,
                common_job_params: ParamsCommonJobParams::new(),
                timestamp: timestamp,
            },
        }
    }
}

#[test]
fn test_new_request() {
    let req = JsonRPCRequest::new(Lang::EN, Lang::ZH, "Hello World!".to_string());

    println!("Debug: {:#?}", req);

    let json = serde_json::to_string(&req).unwrap();
    println!("Json: {}", json);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    jobs: Vec<ParamsJob>,
    lang: ParamsLang,
    priority: u32,
    common_job_params: ParamsCommonJobParams,
    timestamp: u128,
}

#[derive(Serialize, Deserialize, Debug)]
struct ParamsJob {
    kind: String,
    raw_en_sentence: String,
    raw_en_context_before: Vec<String>,
    raw_en_context_after: Vec<String>,
    preferred_num_beams: u32,
}

impl ParamsJob {
    pub fn new(from_text: String) -> Self {
        Self {
            kind: "default".to_string(),
            raw_en_sentence: from_text,
            raw_en_context_before: vec![],
            raw_en_context_after: vec![],
            preferred_num_beams: 4,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ParamsLang {
    // computed of selected should only choose one
    source_lang_computed: Lang,
    source_lang_user_selected: Lang,

    target_lang: Lang,
    preference: ParamsLangPreference,
}

impl ParamsLang {
    pub fn new(lang_from: Lang, lang_to: Lang) -> Self {
        Self {
            preference: ParamsLangPreference::default(),
            source_lang_computed: lang_from,
            source_lang_user_selected: lang_from,
            target_lang: lang_to,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ParamsLangPreference {
    weight: HashMap<String, f32>,
    default: String,
}

impl Default for ParamsLangPreference {
    fn default() -> Self {
        ParamsLangPreference {
            weight: HashMap::new(),
            default: "default".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ParamsCommonJobParams {
    // formality: null
}

impl ParamsCommonJobParams {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Method {
    #[serde(rename = "LMT_handle_jobs")]
    LMTHandleJobs,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Lang {
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
    ZH, // 中文（简体）
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
