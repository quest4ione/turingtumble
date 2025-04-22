use turingtumble::grid::Grid;

fn main() {
    let text = r#"
\.-./
.\./.
-.X.-
./.\,
\.-./
    "#
    .trim();

    let grid = Grid::from_text(text.to_string());
    println!("{}", grid);
}
