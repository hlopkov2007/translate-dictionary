use rustlate;

pub fn translate(text: &str, from: &'static str, to: &'static str) -> Result<String, ()> {
    let translator_struct = rustlate::Translator{
        to: to,
        from: from
    };


    match translator_struct.translate(text) {
        Ok(translated) => return Ok(translated),
        Err(_) => return Err(()),
    }
}