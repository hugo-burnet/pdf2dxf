use lopdf::Document;
fn main() {
    let doc = Document::load("pdf/technical-catalogue-alufire-oct-2018.pdf").unwrap();
    let pages = doc.get_pages();
    if let Some(&page_id) = pages.values().next() {
        let page = doc.get_object(page_id).unwrap().as_dict().unwrap();
        println!("Page Dict: {:?}", page.get(b"MediaBox"));
    }
}
