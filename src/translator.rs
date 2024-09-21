use serde_json::Value;

pub async fn translate(
    client: &reqwest::Client,
    text: &str,
    source_lang: &str,
    target_lang: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://text-translator2.p.rapidapi.com/translate";
    let params = [
        ("source_language", source_lang),
        ("target_language", target_lang),
        ("text", text),
    ];

    let resp = client.post(url).form(&params).send().await?;
    let resp_json: Value = resp.json().await?;

    if let Some(data) = resp_json.get("data") {
        if let Some(translation) = data.get("translatedText") {
            return Ok(translation.as_str().unwrap_or("").to_string());
        }
    }

    Err("No se pudo obtener la traducción.\n
        Elija los idiomas correctos:
        Español (en)
        Inglés (es)
        Francés (fr), etc..."
        .into())
}
