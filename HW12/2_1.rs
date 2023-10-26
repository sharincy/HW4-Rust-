#[derive(Clone)]
enum CustomText {
    Simple(String),
    RepeatedText(Box<CustomText>, usize),
    TextCollection(Vec<Box<CustomText>>, String),
}

impl CustomText {
    fn get_text(&self) -> String {
        match self {
            CustomText::Simple(t) => t.clone(),
            CustomText::RepeatedText(text, n) => text.get_text().repeat(*n),
            CustomText::TextCollection(texts, separator) => texts.iter().map(|text| text.get_text()).collect::<Vec<String>>().join(separator),
        }
    }
}

impl From<&CustomText> for Box<CustomText> {
    fn from(text: &CustomText) -> Box<CustomText> {
        match text {
            CustomText::Simple(content) => Box::new(CustomText::Simple(content.clone())),
            CustomText::RepeatedText(repeated_text, n) => Box::new(CustomText::RepeatedText(repeated_text.clone(), *n)),
            CustomText::TextCollection(texts, separator) => Box::new(CustomText::TextCollection(texts.clone(), separator.clone())),
        }
    }
}

impl AsRef<CustomText> for CustomText {
    fn as_ref(&self) -> &CustomText {
        &self
    }
}

#[test]
fn test_custom_text_composition() {
    let t1 = CustomText::Simple("x|x".into());
    let t2 = CustomText::Simple("[+]".into());
    let t3 = CustomText::RepeatedText(t2.as_ref().into(), 3);
    let t4 = CustomText::RepeatedText(t3.as_ref().into(), 5);

    let mut tvec: Vec<Box<CustomText>> = Vec::new();
    tvec.push(t1.into());
    tvec.push(t2.into());
    tvec.push(t3.into());
    tvec.push(t4.into());
    let t5 = CustomText::Simple("--".into());
    let t6 = CustomText::TextCollection(tvec, t5.get_text().into());

    let pattern = ["x|x", "[+]", &"[+]".repeat(3), &"[+]".repeat(15)];
    let expected = pattern.join("--");
    assert_eq!(t6.get_text(), expected);
}
