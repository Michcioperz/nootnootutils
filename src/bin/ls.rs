extern crate minreq;
extern crate select;

use select::document::Document;
use select::predicate::{Name,Attr,Predicate};

fn main() {
    let resp = minreq::get("http://vger.kernel.org/vger-lists.html").send()
        .expect("Failed to download the list directory").body;
    let doc = Document::from(resp.as_str());
    let p = doc.find(Name("p")).skip(2).next()
        .expect("The list directory is missing a third paragraph");
    for a in p.find(Name("a").and(Attr("href", ()))) {
        println!("{}", a.text());
    }
}
