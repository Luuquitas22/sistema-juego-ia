mod estadistica;
mod juego;
mod jugador;
mod objeto;

use juego::Juego;

fn main() {
    let mut juego = Juego::new();
    juego.iniciar();
}
