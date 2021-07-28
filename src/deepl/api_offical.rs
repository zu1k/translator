use deepl_api::*;

#[allow(dead_code)]
fn deepl() {
    println!("Hello, world!");

    // Create a DeepL instance for our account.
    let deepl = DeepL::new(std::env::var("DEEPL_API_KEY").unwrap());

    // Translate Text
    let texts = TranslatableTextList {
        source_language: Some("DE".to_string()),
        target_language: "EN-US".to_string(),
        texts: vec!["ja".to_string()],
    };
    let translated = deepl.translate(None, texts).unwrap();
    assert_eq!(translated[0].text, "yes");

    // Fetch Usage Information
    let usage_information = deepl.usage_information().unwrap();
    assert!(usage_information.character_limit > 0);
}
