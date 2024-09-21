use std::io::{self, Write};
pub fn inputs() -> Result<(String, String, String), Box<dyn std::error::Error>> {
    let mut text = String::new();
    let mut source = String::new();
    let mut target = String::new();

    println!("Idiomas soportados por la API de traducción:");
    println!("- Español (es)");
    println!("- Inglés (en)");
    println!("- Francés (fr)");
    println!("- Alemán (de)");
    println!("- Italiano (it)");
    println!("- Portugués (pt)");
    println!("- Ruso (ru)");
    println!("- Chino (zh)");
    println!("- Japonés (ja)");
    println!("- Árabe (ar)");
    println!("(Use el código de dos letras entre paréntesis para especificar el idioma)\n");

    //Añadimos `io::stdout().flush()?;` después de cada `print!`
    //para asegurar que el prompt se muestre antes de leer la entrada.

    print!("Texto a traducir: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut text)?;

    print!("Idioma del texto origen: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut source)?;

    print!("Idioma a traducir: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut target)?;

    // Eliminar los saltos de línea al final
    text = text.trim().to_string();
    source = source.trim().to_string();
    target = target.trim().to_string();

    Ok((text, source, target))
}
