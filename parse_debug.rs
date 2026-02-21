use lopdf::Document;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut doc = Document::load("pdf/technical-catalogue-alufire-oct-2018.pdf").unwrap();
    let pages = doc.get_pages();
    
    // get first page
    if let Some(&page_id) = pages.get(&1) {
        if let Ok(content_data) = doc.get_page_content(page_id) {
            let mut file = File::create("page1_stream.txt").unwrap();
            let content_str = String::from_utf8_lossy(&content_data);
            file.write_all(content_str.as_bytes()).unwrap();
            println!("Dumped page 1 stream to page1_stream.txt");
        }
    }
}
