fn format_fen(fen: &str) -> String {
    fen.replace("/", "\n")
        .replace("8", "")
        .replace("r", "♖ ")
        .replace("n", "♘ ")
        .replace("b", "♗ ")
        .replace("k", "♔ ")
        .replace("q", "♕ ")
        .replace("p", "♙ ")
        .replace("R", "♜ ")
        .replace("N", "♞ ")
        .replace("B", "♝ ")
        .replace("Q", "♛ ")
        .replace("K", "♚ ")
        .replace("P", "♟ ")

}

fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

    let fen = format_fen(fen);

    println!("{}", fen);
}

