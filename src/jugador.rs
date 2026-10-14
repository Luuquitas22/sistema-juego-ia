use crate::estadistica::Estadistica;
use crate::objeto::{Objeto, TipoObjeto};

#[derive(Debug, Clone)]
pub struct Jugador {
    pub nombre: String,
    pub vidas: i32,
    pub stats: Estadistica,
    pub inventario: Vec<Objeto>,
}

impl Jugador {
    pub fn new(nombre: &str) -> Self {
        Self {
            nombre: nombre.to_string(),
            vidas: 6,
            stats: Estadistica::default(),
            inventario: Objeto::crear_inventario_inicial(),
        }
    }

    pub fn esta_vivo(&self) -> bool {
        self.vidas > 0
    }

    pub fn usar_objeto(&mut self, idx: usize) -> bool {
        if idx >= self.inventario.len() {
            return false;
        }

        let obj = self.inventario.remove(idx);
        match obj.tipo {
            TipoObjeto::PocionVida(curacion) => {
                self.vidas = (self.vidas + curacion).min(6);
                println!(
                    "{} uso '{}' y recupero salud. Vidas actuales: {}/6",
                    self.nombre, obj.nombre, self.vidas
                );
            }
            TipoObjeto::ArmaMejorada(bonus) => {
                self.stats.prob_acierto = (self.stats.prob_acierto + bonus).min(1.0);
                println!(
                    "{} equipo '{}'. Su precision subio a {:.0}%.",
                    self.nombre,
                    obj.nombre,
                    self.stats.prob_acierto * 100.0
                );
            }
            TipoObjeto::EscudoMadera(bonus) => {
                self.stats.prob_bloqueo = (self.stats.prob_bloqueo + bonus).min(0.85);
                println!(
                    "{} uso '{}'. Su bloqueo base subio a {:.0}%.",
                    self.nombre,
                    obj.nombre,
                    self.stats.prob_bloqueo * 100.0
                );
            }
        }
        true
    }
}