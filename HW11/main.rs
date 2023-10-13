use std::fs;
use std::io;
use std::path::Path;

fn count_paragraphs(text: &str) -> usize {
    text.split("\n\n").count()
}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn main() -> io::Result<()> {
    let file_paths = vec!["document1.txt", "document2.txt", "document3.txt"]; 

    let mut documents: Vec<(String, String)> = Vec::new();

    for file_path in &file_paths {
        let content = fs::read_to_string(file_path)?;

        documents.push((file_path.to_string(), content));
    }

    let paragraph_table = generate_html_table(&documents, count_paragraphs);
    save_html_table("paragraph_counts.html", &paragraph_table)?;

    let word_table = generate_html_table(&documents, count_words);
    save_html_table("word_counts.html", &word_table)?;

    Ok(())
}

fn save_html_table(file_name: &str, html_content: &str) -> io::Result<()> {
    fs::write(file_name, html_content)
}

fn generate_html_table<F>(documents: &[(String, String)], counting_function: F) -> String
where
    F: Fn(&str) -> usize,
{
    let mut counts: Vec<(String, usize)> = documents
        .iter()
        .map(|(file, content)| (file.clone(), counting_function(content)))
        .collect();

    counts.sort_by(|a, b| b.1.cmp(&a.1));

    let mut table = String::from("<table>");
    table.push_str("<tr><th>File</th><th>Count</th></tr>");

    for (file, count) in counts {
        table.push_str(&format!("<tr><td>{}</td><td>{}</td></tr>", file, count));
    }

    table.push_str("</table>");
    table
}
