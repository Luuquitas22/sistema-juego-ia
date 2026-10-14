#[derive(Debug, Clone)]
pub enum TipoObjeto {
    ArmaMejorada(f32),  // Aumenta la probabilidad de acierto (ej: +0.15)
    PocionVida(i32),    // Restaura vidas (ej: +2)
    EscudoMadera(f32),  // Aumenta la probabilidad de bloqueo (ej: +0.25)
}

#[derive(Debug, Clone)]
pub struct Objeto {
    pub nombre: String,
    pub tipo: TipoObjeto,
}

impl Objeto {
    pub fn crear_inventario_inicial() -> Vec<Objeto> {
        vec![
            Objeto {
                nombre: "Poción de Salud".into(),
                tipo: TipoObjeto::PocionVida(2),
            },
            Objeto {
                nombre: "Espada Afilada".into(),
                tipo: TipoObjeto::ArmaMejorada(0.15),
            },
            Objeto {
                nombre: "Escudo Reforzado".into(),
                tipo: TipoObjeto::EscudoMadera(0.25),
            },
        ]
    }
}