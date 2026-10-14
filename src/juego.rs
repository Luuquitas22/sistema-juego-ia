use rand::Rng;
use std::io::{self, Write};

use crate::jugador::Jugador;

pub enum ResultadoAtaque {
    ImpactoExitoso,
    Bloqueado,
    Fallo,
}

pub struct Juego {
    pub usuario: Jugador,
    pub ia: Jugador,
    pub turno_numero: u32,
}

impl Juego {
    pub fn new() -> Self {
        Self {
            usuario: Jugador::new("Usuario"),
            ia: Jugador::new("Sistema"),
            turno_numero: 1,
        }
    }

    pub fn iniciar(&mut self) {
        println!("============================================");
        println!("       COMBATE TACTICO POR TURNOS (1v1)");
        println!("============================================");

        // 1. Sorteo de numero (1 o 2)
        let mut rng = rand::thread_rng();
        let eleccion_usuario = self.pedir_numero();
        let numero_ganador = rng.gen_range(1..=2);

        let usuario_empieza = eleccion_usuario == numero_ganador;

        println!("\nSorteando orden de turnos...");
        if usuario_empieza {
            println!("Ganaste el sorteo! Eliges actuar primero en cada ronda.");
        } else {
            println!("El Sistema gano el sorteo. Actuara primero en cada ronda.");
        }

        // 2. Bucle de combate hasta agotar las 6 vidas
        while self.usuario.esta_vivo() && self.ia.esta_vivo() {
            println!("\n--------------------------------------------");
            println!("--- RONDA {} ---", self.turno_numero);

            if usuario_empieza {
                self.ejecutar_turno_usuario();
                if self.ia.esta_vivo() {
                    self.ejecutar_turno_ia();
                }
            } else {
                self.ejecutar_turno_ia();
                if self.usuario.esta_vivo() {
                    self.ejecutar_turno_usuario();
                }
            }
            self.turno_numero += 1;
        }

        // 3. Fin de juego
        println!("\n============================================");
        if self.usuario.esta_vivo() {
            println!("VICTORIA! Derrotaste al Sistema.");
        } else {
            println!("DERROTA! El Sistema te ha dejado sin vidas.");
        }
    }

    fn pedir_numero(&self) -> i32 {
        loop {
            print!("\nElije un numero para el sorteo (1 o 2): ");
            io::stdout().flush().unwrap();
            let mut entrada = String::new();
            if io::stdin().read_line(&mut entrada).is_ok() {
                if let Ok(num) = entrada.trim().parse::<i32>() {
                    if num == 1 || num == 2 {
                        return num;
                    }
                }
            }
            println!("Entrada invalida. Ingresa 1 o 2.");
        }
    }

    fn calcular_impacto(prob_acierto: f32, prob_bloqueo: f32) -> ResultadoAtaque {
        let mut rng = rand::thread_rng();

        // Check acierto
        let tirada_acierto: f32 = rng.r#gen();
        if tirada_acierto > prob_acierto {
            return ResultadoAtaque::Fallo;
        }

        // Check bloqueo
        let tirada_bloqueo: f32 = rng.r#gen();
        if tirada_bloqueo < prob_bloqueo {
            return ResultadoAtaque::Bloqueado;
        }

        ResultadoAtaque::ImpactoExitoso
    }

    fn resolver_ataque(&mut self, atacante_es_usuario: bool) {
        let (atacante, defensor) = if atacante_es_usuario {
            (&self.usuario, &mut self.ia)
        } else {
            (&self.ia, &mut self.usuario)
        };

        let prob_bloqueo_efectiva = if defensor.stats.postura_defensiva {
            (defensor.stats.prob_bloqueo + 0.35).min(0.90) // +35% de bloqueo en postura defensiva
        } else {
            defensor.stats.prob_bloqueo
        };

        match Self::calcular_impacto(atacante.stats.prob_acierto, prob_bloqueo_efectiva) {
            ResultadoAtaque::ImpactoExitoso => {
                defensor.vidas -= 1;
                println!(
                    "Ataque de {} EXITOSO! (Vidas de {}: {}/6)",
                    atacante.nombre, defensor.nombre, defensor.vidas
                );
            }
            ResultadoAtaque::Bloqueado => {
                println!("{} BLOQUEO el ataque con exito!", defensor.nombre);
            }
            ResultadoAtaque::Fallo => {
                println!("{} FALLO el ataque!", atacante.nombre);
            }
        }
    }

    fn ejecutar_turno_usuario(&mut self) {
        self.usuario.stats.postura_defensiva = false;
        let mut acciones_restantes = 2;

        println!("\n>>> Tu turno ({}/6 Vidas) <<<", self.usuario.vidas);

        while acciones_restantes > 0 && self.ia.esta_vivo() {
            println!("\nAcciones disponibles: {}", acciones_restantes);
            print!("Ingresa comando (atacar / defender / objeto / estado): ");
            io::stdout().flush().unwrap();

            let mut entrada = String::new();
            if io::stdin().read_line(&mut entrada).is_err() {
                continue;
            }

            match entrada.trim().to_lowercase().as_str() {
                "atacar" => {
                    self.resolver_ataque(true);
                    acciones_restantes -= 1;
                }
                "defender" => {
                    self.usuario.stats.postura_defensiva = true;
                    println!("Te pones en postura defensiva (+35% probabilidad de bloqueo en esta ronda).");
                    acciones_restantes -= 1;
                }
                "objeto" => {
                    if self.menu_usar_objeto_usuario() {
                        acciones_restantes -= 1;
                    }
                }
                "estado" => {
                    println!("\n[ESTADO ACTUAL]");
                    println!(
                        "Tu: {}/6 vidas | Precision: {:.0}% | Bloqueo: {:.0}%",
                        self.usuario.vidas,
                        self.usuario.stats.prob_acierto * 100.0,
                        self.usuario.stats.prob_bloqueo * 100.0
                    );
                    println!(
                        "Sistema: {}/6 vidas | Precision: {:.0}% | Bloqueo: {:.0}%",
                        self.ia.vidas,
                        self.ia.stats.prob_acierto * 100.0,
                        self.ia.stats.prob_bloqueo * 100.0
                    );
                }
                _ => println!("Comando no valido. Opciones: atacar, defender, objeto, estado."),
            }
        }
    }

    fn menu_usar_objeto_usuario(&mut self) -> bool {
        if self.usuario.inventario.is_empty() {
            println!("No te quedan objetos en el inventario.");
            return false;
        }

        println!("\n-- Inventario --");
        for (i, obj) in self.usuario.inventario.iter().enumerate() {
            println!("{}. {} ({:?})", i + 1, obj.nombre, obj.tipo);
        }

        print!("Selecciona el numero de objeto a usar (0 para cancelar): ");
        io::stdout().flush().unwrap();

        let mut entrada = String::new();
        if io::stdin().read_line(&mut entrada).is_ok() {
            if let Ok(num) = entrada.trim().parse::<usize>() {
                if num > 0 && num <= self.usuario.inventario.len() {
                    return self.usuario.usar_objeto(num - 1);
                }
            }
        }
        println!("Seleccion cancelada.");
        false
    }

    fn ejecutar_turno_ia(&mut self) {
        self.ia.stats.postura_defensiva = false;
        let mut rng = rand::thread_rng();

        println!("\n>>> Turno del Sistema ({}/6 Vidas) <<<", self.ia.vidas);

        for _ in 0..2 {
            if !self.usuario.esta_vivo() {
                break;
            }

            let decision = rng.gen_range(1..=3);
            match decision {
                1 => self.resolver_ataque(false),
                2 => {
                    self.ia.stats.postura_defensiva = true;
                    println!("El Sistema adopta una postura defensiva.");
                }
                _ => {
                    if !self.ia.inventario.is_empty() {
                        let idx = rng.gen_range(0..self.ia.inventario.len());
                        self.ia.usar_objeto(idx);
                    } else {
                        self.resolver_ataque(false);
                    }
                }
            }
        }
    }
}

