use cursive::views::{Dialog, EditView, LinearLayout, TextView};
use cursive::{Cursive, CursiveExt};
use std::net::TcpStream;
use std::io::Write;
use cursive::traits::Resizable;
use serde_json::json;
use cursive::view::Nameable;
fn main() {
    let mut siv = Cursive::default();

    siv.add_layer(
        Dialog::new()
            .title("Send Transaction")
            .content(
                LinearLayout::vertical()
                    .child(TextView::new("From:"))
                    .child(EditView::new().with_name("from").fixed_width(20))
                    .child(TextView::new("To:"))
                    .child(EditView::new().with_name("to").fixed_width(20))
                    .child(TextView::new("Amount:"))
                    .child(EditView::new().with_name("amount").fixed_width(20)),
            )
            .button("Send", |s| {
                let from = s.call_on_name("from", |v: &mut EditView| v.get_content()).unwrap();
                let to = s.call_on_name("to", |v: &mut EditView| v.get_content()).unwrap();
                let amount = s.call_on_name("amount", |v: &mut EditView| v.get_content()).unwrap();

                if let Ok(amount) = amount.parse::<u64>() {
                    send_transaction(&from, &to, amount);
                    s.add_layer(Dialog::info("Transaction sent!"));
                } else {
                    s.add_layer(Dialog::info("Invalid amount!"));
                }
            })
            .button("Quit", |s| s.quit()),
    );

    siv.run();
}

fn send_transaction(from: &str, to: &str, amount: u64) {
    let transaction = json!({
        "from": from,
        "to": to,
        "amount": amount
    });

    if let Ok(mut stream) = TcpStream::connect("localhost:7878") {
        let serialized = serde_json::to_string(&transaction).unwrap() + "\n";
        if let Err(e) = stream.write_all(serialized.as_bytes()) {
            eprintln!("Failed to send transaction: {}", e);
        }
    } else {
        eprintln!("Could not connect to prover");
    }
}
