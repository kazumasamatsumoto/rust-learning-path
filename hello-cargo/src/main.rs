#[derive(Debug)]
struct KeyPress(String, char);
#[derive(Debug)]
struct MouseClick {
    x: i64,
    y: i64,
}

#[derive(Debug)]
enum WebEvent {
    WELoad(bool),
    WEClick(MouseClick),
    WEKeys(KeyPress),
}

fn main() {
    let we_load = WebEvent::WELoad(true);
    println!("{:?}", we_load);
    let click = MouseClick { x: 100, y: 250 };
    let we_click = WebEvent::WEClick(click);
    println!("{:#?}", we_click);
    let keys = KeyPress(String::from("Ctrl+"), 'N');
    let we_key = WebEvent::WEKeys(keys);
    println!("{:?}", we_key);
}
