use fltk::{
    app,
    button::Button,
    prelude::*,
    tree::{Tree, TreeSelect},
    window::Window,
};
use std::{env, fs};
use std::path::Path;

fn main() {
    let mut first_arg = env::args().nth(1).unwrap();
    let paths = visit_dirs((&*first_arg).as_ref());
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = Window::default().with_size(400, 300);
    let mut tree = Tree::new(5, 10, 190, 240, "");
    let mut root = first_arg.split("/");
    let mut roots = root.last().unwrap();
    root = roots.split("\\");
    roots = root.last().unwrap();
    tree.set_root_label(roots);
    for item in paths {
        tree.add(&item[first_arg.len()+1..]);
    }
    let mut but = Button::new(160, 255, 80, 40, "Get Items");
    wind.make_resizable(true);
    wind.show();

    but.set_callback(move |_| match tree.get_selected_items() {
        None => println!("No items selected"),
        Some(vals) => println!("{} items selected", vals.as_slice()[0].label().unwrap()),
    });

    app.run().unwrap();
}

fn visit_dirs(dir: &Path) -> Vec<String> {
    let mut returns:Vec<String> = Vec::new();
    if dir.is_dir() {
        let folder = fs::read_dir(dir).unwrap();
        for item in folder {
            let path = item.unwrap().path();
            if path.is_dir() {
                for items in visit_dirs(&*path) {
                    returns.push(items);
                }
            }
            else {
                returns.push(path.display().to_string())
            }
        }
    }
    return returns;
}