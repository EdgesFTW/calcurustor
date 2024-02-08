use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Button,
    Box, Grid, Entry,
};

const APP_ID: &str = "org.gtk_rs.calcurustor";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

// static mut EVAL_STRING: String = String::new();

fn process_expression(str: String) -> String {
    let mut operators: Vec<char> = Vec::new();
    for char in str.chars() {
        match char {
            '+' => operators.push(char),
            '-' => operators.push(char),
            '*' => operators.push(char),
            '/' => operators.push(char),
            '^' => operators.push(char),
            '=' => operators.push(char),
            _ => (),
        }
    }

    let pattern = |x: char| x == '+'
        || x == '-'
        || x == '*'
        || x == '/'
        || x == '^'
        || x == '=';
    let numbers: Vec<i32> = str.split(pattern).map(
        |x: &str| {
            if x != "" {
                let ret = x.parse::<i32>().unwrap();
                return ret;
            }
            return 0
        }).collect();

    let mut running_eval = numbers[0];
    for i in 1..numbers.len() {
        match operators[i-1] {
            '+' => running_eval += numbers[i],
            '-' => running_eval -= numbers[i],
            '*' => running_eval *= numbers[i],
            '/' => running_eval /= numbers[i],
            '^' => running_eval = running_eval.pow(numbers[i] as u32),
            _ => (),
        }
    }

    return running_eval.to_string();
}

fn process_input(c: char, b: &Entry){
    let mut p: String = b.buffer().text().into();
    p.insert(p.chars().count(), c);
    b.buffer().set_text(p.clone());
    match c {
        '=' => b.buffer().set_text(process_expression(p)),
        _ => (),
    }
}

fn build_keypad(text: Rc<Entry>) -> Grid{
    let keypad = Grid::builder()
        .valign(gtk4::Align::Fill)
        .build();
    for i in 0..10 {
        let c: char = i.to_string()
            .chars().nth(0).unwrap();
        let key = Button::builder()
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .margin_top(10)
            .label(c.to_string())
            .build();
        let tmp = text.clone();
        key.connect_clicked(move |_| {
            let t = tmp.clone();
            process_input(c, &*t);
        });
        if i==0 {
            keypad.attach(&key, 1, 3, 1, 1)
        }
        else {
            keypad.attach(&key, (i-1)%3, (9-i)/3, 1, 1)
        }
    }

    return keypad;
}

fn build_operator_pad(text: Rc<Entry>) -> Grid{
    let pad = Grid::builder()
        .build();
    let operators = "+-*/^=".chars();
    for (i, c) in operators.enumerate(){
        let key = Button::builder()
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .margin_top(10)
            .label(c.to_string())
            .build();
        let tmp = text.clone();
        key.connect_clicked(move |_| {
            let t = tmp.clone();
            process_input(c, &*t);
        });
        let row: i32 = i as i32 % 3 ;
        let col: i32 = i as i32 / 3 ;
        pad.attach(&key, row, col, 1, 1);
    }

    return pad;
}

fn build_ui(app: &Application){
    let text: Rc<Entry> = Rc::new(Entry::builder()
        .editable(false)
        .build()
        );
    let keypad: Grid = build_keypad(text.clone());
    let operators: Grid = build_operator_pad(text.clone());

    let pad = Grid::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .build();
    pad.attach(&*text,0,0,2,1);
    pad.attach(&keypad,0,1,1,1);
    pad.attach(&operators,1,1,1,1);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Calcurustor")
        .child(&pad)
        .build();

    window.present();
}
