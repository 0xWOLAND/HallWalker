pub mod board;
pub mod graph;
fn main() {
    println!("Width: {}", graph::BOARD_WIDTH);
    board::print();
}
