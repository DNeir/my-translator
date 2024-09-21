mod client;
mod inputs;
mod translator;
use client::create_client;
use inputs::inputs;
use translator::translate;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = create_client()?;

    let (text_to_translate, source_lang, target_lang) = inputs()?;

    match translate(
        &client,
        text_to_translate.as_str(),
        source_lang.as_str(),
        target_lang.as_str(),
    )
    .await
    {
        Ok(translation) => println!("Traducción: {}", translation),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}
