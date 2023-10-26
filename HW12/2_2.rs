trait Text {
    fn get_value(&self) -> String;
    fn clone_text(&self) -> Box<dyn Text>;
}

impl Clone for Box<dyn Text> {
    fn clone(&self) -> Self {
        self.clone_text()
    }
}

struct CustomPlainText {
    content: String,
}

struct CustomRepeatedText {
    text: Box<dyn Text>,
    repetitions: usize,
}

struct CustomJoinedText {
    texts: Vec<Box<dyn Text>>,
    separator: String,
}

impl From<&str> for CustomPlainText {
    fn from(text: &str) -> CustomPlainText {
        CustomPlainText {
            content: text.to_string(),
        }
    }
}

impl Text for CustomPlainText {
    fn get_value(&self) -> String {
        self.content.clone()
    }

    fn clone_text(&self) -> Box<dyn Text> {
        Box::new(CustomPlainText {
            content: self.content.clone(),
        })
    }
}


impl Text for CustomRepeatedText {
    fn get_value(&self) -> String {
        self.text.get_value().repeat(self.repetitions)
    }

    fn clone_text(&self) -> Box<dyn Text> {
        Box::new(CustomRepeatedText {
            text: self.text.clone_text(),
            repetitions: self.repetitions,
        })
    }
}

impl Text for CustomJoinedText {
    fn get_value(&self) -> String {
        self.texts
            .iter()
            .map(|t| t.get_value())
            .collect::<Vec<String>>()
            .join(&self.separator)
    }

    fn clone_text(&self) -> Box<dyn Text> {
        Box::new(CustomJoinedText {
            texts: self.texts.iter().map(|t| t.clone_text()).collect(),
            separator: self.separator.clone(),
        })
    }
}

impl AsRef<dyn Text> for CustomPlainText {
    fn as_ref(&self) -> &(dyn Text + 'static) {
        self
    }
}

impl CustomRepeatedText {
    fn create_with_parts(text: &dyn Text, repetitions: usize) -> CustomRepeatedText {
        CustomRepeatedText {
            text: text.clone_text(),
            repetitions,
        }
    }
}

impl CustomJoinedText {
    fn create_with_parts(texts: Vec<Box<dyn Text>>, separator: &str) -> CustomJoinedText {
        CustomJoinedText {
            texts,
            separator: separator.to_string(),
        }
    }
}

#[test]
fn test_text_composition() {
    let t1 = CustomPlainText::from("x|x");
    let t2 = CustomPlainText::from("[+]");
    let t3 = CustomRepeatedText::create_with_parts(&t2, 3);
    let t4 = CustomRepeatedText::create_with_parts(&t3, 5);

    let mut tvec: Vec<Box<dyn Text>> = Vec::new();
    tvec.push(t1.clone_text());
    tvec.push(t2.clone_text());
    tvec.push(t3.clone_text());
    tvec.push(t4.clone_text());
    let t5 = CustomPlainText::from("--");
    let t6 = CustomJoinedText::create_with_parts(tvec, &t5.get_value());

    let pattern = ["x|x", "[+]", &"[+]".repeat(3), &"[+]".repeat(15)];
    let expected = pattern.join("--");
    assert_eq!(t6.get_value(), expected);
}
