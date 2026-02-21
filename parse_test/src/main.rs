use lopdf::{Document, content::Content};

fn main() {
    let doc = Document::load("pdf/technical-catalogue-alufire-oct-2018.pdf").unwrap();
    let mut operators = std::collections::HashMap::new();

    for (_, page_id) in doc.get_pages() {
        if let Ok(content_data) = doc.get_page_content(page_id) {
            if let Ok(content) = Content::decode(&content_data) {
                for op in &content.operations {
                    *operators.entry(op.operator.clone()).or_insert(0) += 1;
                }
            }
        }
        break; // Analyser uniquement la première page pour se donner une idée de ce qui est utilisé.
    }

    let mut ops: Vec<_> = operators.into_iter().collect();
    ops.sort_by_key(|&(_, c)| std::cmp::Reverse(c));
    for (op, count) in ops {
        println!("{}: {}", op, count);
    }
}
