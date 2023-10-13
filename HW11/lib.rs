

fn make_document(text: &str) -> Vec<String> {
    let mut paragraphs = Vec::new();
    let mut current_paragraph = String::new();

    for line in text.lines() {
        if line.trim().is_empty() {
            if !current_paragraph.is_empty() {
                paragraphs.push(current_paragraph);
                current_paragraph = String::new();
            }
        } else {
            current_paragraph.push_str(line);
            current_paragraph.push('\n');
        }
    }

    if !current_paragraph.is_empty() {
        paragraphs.push(current_paragraph);
    }

    paragraphs
}

fn rank_documents(docs: &[Vec<String>]) -> Vec<Vec<String>> {
    let mut indexed_docs: Vec<(usize, &Vec<String>)> = docs.iter().enumerate().collect();

    indexed_docs.sort_by(|a, b| {
        a.1.len().cmp(&b.1.len()).reverse()
    });

    indexed_docs.into_iter().map(|(_, doc)| doc.to_vec()).collect()
}



#[test]
fn test_make_document() {
    let fox = "The quick brown fox jumps over the lazy dog.";
    let para3 = "a\n\nb\n\nc";
    let bustle = "\
        The bustle in a house\n\
        The morning after death\n\
        Is solemnest of industries\n\
        Enacted upon earth,—\n\
        \n\
        The sweeping up the heart,\n\
        And putting love away\n\
        We shall not want to use again\n\
        Until eternity.\n\
    ";

    let doc1 = make_document(fox);
    let doc2 = make_document(bustle);
    let doc3 = make_document(para3);

    assert_eq!(doc1.len(), 1);
    assert_eq!(doc2.len(), 2);
    assert_eq!(doc3.len(), 3);
}

#[test]
fn test_rank_documents() {
    let fox = "The quick brown fox jumps over the lazy dog.";
    let para3 = "a\n\nb\n\nc";
    let bustle = "\
        The bustle in a house\n\
        The morning after death\n\
        Is solemnest of industries\n\
        Enacted upon earth,—\n\
        \n\
        The sweeping up the heart,\n\
        And putting love away\n\
        We shall not want to use again\n\
        Until eternity.\n\
    ";

    let doc1 = make_document(fox);
    let doc2 = make_document(bustle);
    let doc3 = make_document(para3);

    let docs = vec![doc1.clone(), doc3.clone(), doc2.clone()];
    let rnk_docs = rank_documents(&docs);

    assert_eq!(rnk_docs, [doc3, doc2, doc1]);
}
