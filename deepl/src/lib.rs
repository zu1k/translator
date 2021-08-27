use reqwest;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
mod lang;
pub use lang::*;

#[derive(Serialize, Debug)]
struct Req {
    text: String,
    source_lang: Lang,
    target_lang: Lang,
}

impl Req {
    pub fn new(text: String, source_lang: Lang, target_lang: Lang) -> Self {
        Req {
            text,
            source_lang,
            target_lang,
        }
    }
}

#[derive(Deserialize, Debug)]
struct Resp {
    code: i32,
    result: String,
}

pub async fn translate_async(
    text: String,
    target_lang: Lang,
    source_lang: Lang,
) -> Result<String, Box<dyn std::error::Error>> {
    let target_lang = if target_lang == Lang::Auto {
        Lang::ZH
    } else {
        target_lang
    };
    let req = Req::new(text, source_lang, target_lang);
    let client = reqwest::Client::new();

    let mut resp = client
        .post("https://deepl.lgf.im/translate")
        .json(&req)
        .send()
        .await?
        .json::<Resp>()
        .await;

    if let Err(_) = resp {
        resp = client
            .post("https://deepl.vercel.app/translate")
            .json(&req)
            .send()
            .await?
            .json::<Resp>()
            .await;
    }

    if let Ok(data) = resp {
        if data.code == 200 {
            return Ok(data.result);
        }
    }
    Ok("翻译接口失效".into())
}

pub fn translate(
    text: String,
    target_lang: Lang,
    source_lang: Lang,
) -> Result<String, Box<dyn std::error::Error>> {
    Runtime::new()
        .unwrap()
        .block_on(translate_async(text, target_lang, source_lang))
}
