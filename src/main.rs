use regex::Regex;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

const LUNGIME_MAX_LINIE: usize = 120;
const JUM_LUNGIME: usize = LUNGIME_MAX_LINIE / 2;

fn ajutor() {
    println!("Utilizare: nume_program <adresa_director/fisier> <sir_cautat> [-count] [-ignore] [-max numarul maxim de linii analizate] [-regex]");
    println!("Argumente:");
    println!("  -count           : Afiseaza doar numarul de aparitii per fisier.");
    println!(
        "  -ignore          : Ignora distinctia dintre majuscule si minuscule in timpul cautarii."
    );
    println!("  -max numar_linii : Seteaza numarul maxim de linii analizate.");
    println!("  -regex           : Activeaza cautarea cu expresii regulate.");
    println!("   help            : Afiseaza aceasta informatie.");
}

fn parcurgere_recursiva(
    director: &Path,
    sir: &str,
    rezultat: &mut bool,
    count: bool,
    ignore: bool,
    max_linii: usize,
    regex: bool,
) {
    if let Ok(intrari) = fs::read_dir(director) {
        for e in intrari.flatten() {
            let path = e.path();
            if !path.is_dir() {
                // Daca este un fisier, efectuam cautarea
                match cautare_fisier(&path, sir, count, ignore, max_linii, regex) {
                    Ok(true) => *rezultat = true,
                    Err(e) => {
                        if let Some(eroare) = e.source() {
                            if eroare.is::<std::str::Utf8Error>() {
                                // Se ignora fisierele care nu sunt in format UTF-8 valid
                                continue;
                            } else {
                                eprintln!(
                                    "Eroare în timpul citirii fisierului {}: {}",
                                    path.display(),
                                    e
                                );
                            }
                        }
                    }
                    Ok(false) => {}
                }
            } else {
                // Dacă este un director, apelăm recursiv functia pe acel director
                parcurgere_recursiva(&path, sir, rezultat, count, ignore, max_linii, regex);
            }
        }
    } else {
        match cautare_fisier(director, sir, count, ignore, max_linii, regex) {
            Ok(true) => *rezultat = true,
            Err(e) => {
                if let Some(eroare) = e.source() {
                    if eroare.is::<std::str::Utf8Error>() {
                        // Se ignora fisierele care nu sunt in format UTF-8 valid
                    } else {
                        eprintln!(
                            "Eroare în timpul citirii fișierului {}: {}",
                            director.display(),
                            e
                        );
                    }
                }
            }
            Ok(false) => {}
        }
    }
}

fn cautare_fisier(
    path: &Path,
    sir_cautat: &str,
    count: bool,
    ignore: bool,
    max_linii: usize,
    regex: bool,
) -> io::Result<bool> {
    let mut file = File::open(path)?;
    let mut continut_fisier = String::new();
    file.read_to_string(&mut continut_fisier)?;

    let mut counter = 0;
    //Fac o copie pt a afisa continutul original in cazul in care ignore == true
    let copie_continut = continut_fisier.clone();
    let sir;

    // Daca ignore e true fac textul cautat si cel din fisier sa contina doar minuscule
    if ignore {
        sir = sir_cautat.to_lowercase();
        continut_fisier = continut_fisier.to_lowercase();
    } else {
        sir = sir_cautat.to_string();
    }

    // Folosesc aceasta variabila pentru a nu afisa aceiasi linie de mai multe ori in cazul in care cuvantul cautat se gaseste de mai multe ori pe aceiasi linie
    let mut linie_afisata;

    let mut raspuns = false;
    if regex {
        let sir_regex = match Regex::new(&sir) {
            Ok(regex) => regex,
            Err(err) => {
                eprintln!("Eroare la compilarea regex: {}", err);
                return Ok(false);
            }
        };
        for (linie, continut_linie) in continut_fisier.lines().enumerate() {
            if max_linii < linie + 1 {
                break;
            }

            let mut stanga: usize = 0;
            let mut dreapta: usize = continut_linie.len();

            if sir_regex.is_match(continut_linie) {
                let pozitie = if let Some(gasit) = sir_regex.find(continut_linie) {
                    gasit.start()
                } else {
                    continue;
                };
                if !raspuns {
                    // println!("Fisierul: {:?}", path.file_name().unwrap_or_default());
                    println!("{}", path.display());
                    raspuns = true;
                }
                if continut_linie.len() > LUNGIME_MAX_LINIE {
                    if pozitie < JUM_LUNGIME {
                        stanga = 0;
                    } else {
                        stanga = pozitie - JUM_LUNGIME;
                    }
                    if pozitie > continut_linie.len() - JUM_LUNGIME {
                        dreapta = continut_linie.len();
                    } else {
                        dreapta = pozitie + JUM_LUNGIME;
                    }
                }

                if count {
                    counter += sir_regex.find_iter(continut_linie).count();
                } else if ignore {
                    if let Some(continut_linie) = copie_continut.lines().nth(linie) {
                        println!("{}: {}", linie + 1, &continut_linie[stanga..dreapta]);
                    } else {
                        println!("Indexul liniei depaseste numarul total de linii.");
                    }
                } else {
                    println!("{}: {}", linie + 1, &continut_linie[stanga..dreapta]);
                }
            }
        }
    } else {
        // Folosesc alg Boyer-Moore pentru cautare

        let m = sir.len();
        let mut skip = vec![m; 256];

        let mut i = 0;
        while i < m - 1 {
            let caracter = sir.as_bytes()[i];
            skip[caracter as usize] = m - 1 - i;
            i += 1;
        }
        for (linie, continut_linie) in continut_fisier.lines().enumerate() {
            if max_linii < linie + 1 {
                break;
            }
            linie_afisata = false;
            let mut stanga = 0;
            let mut dreapta = continut_linie.len();
            let n = continut_linie.len();
            let mut i = m - 1;
            while i < n {
                let mut j = m - 1;
                let mut k = i;
                while j > 0 && continut_linie.as_bytes()[k] == sir.as_bytes()[j] {
                    j -= 1;
                    k -= 1;
                }

                if continut_linie.as_bytes()[k] == sir.as_bytes()[j] && j == 0 {
                    if !raspuns {
                        // println!("Fisierul: {:?}", path.file_name().unwrap_or_default());
                        println!("{}", path.display());
                        raspuns = true;
                    }

                    if continut_linie.len() > LUNGIME_MAX_LINIE {
                        if i < JUM_LUNGIME {
                            stanga = 0;
                        } else {
                            stanga = i - JUM_LUNGIME;
                        }
                        if i > continut_linie.len() - JUM_LUNGIME {
                            dreapta = continut_linie.len();
                        } else {
                            dreapta = i + JUM_LUNGIME;
                        }
                    }
                    if count {
                        counter += 1;
                    } else if ignore && !linie_afisata {
                        if let Some(continut_linie) = copie_continut.lines().nth(linie) {
                            println!("{}: {}", linie + 1, &continut_linie[stanga..dreapta]);
                        } else {
                            println!("Indexul liniei depășește numărul total de linii.");
                        }
                    } else if !linie_afisata {
                        println!("{}: {}", linie + 1, &continut_linie[stanga..dreapta]);
                    }
                    linie_afisata = true;
                }

                i += skip[continut_linie.as_bytes()[i] as usize];
            }
        }
    }
    if counter > 0 {
        println!("{}", counter);
    }
    Ok(raspuns)
}

fn main() {
    let argumente: Vec<String> = env::args().collect();
    if argumente.len() < 3 {
        if argumente.len() == 2 && argumente[1] == "help" {
            ajutor();
            std::process::exit(0);
        } else {
            eprintln!(
                "Numar insuficient de argumente. Rulare cu 'help' pentru mai multe informatii."
            );
            std::process::exit(1);
        }
    }

    let mut rezultat = false;

    let director = Path::new(&argumente[1]);

    if !director.exists() {
        eprintln!("Calea nu exista.");
        std::process::exit(1);
    }

    let sir_cautat = &argumente[2];
    let mut max_linii = usize::MAX;
    let mut count = false;
    let mut ignore = false;
    let mut regex = false;

    let mut i = 3;
    while i < argumente.len() {
        let optiune = &argumente[i];
        match optiune.as_str() {
            "-count" => count = true,
            "-ignore" => ignore = true,
            "-regex" => regex = true,
            "-max" => {
                i += 1;
                if i < argumente.len() {
                    if let Ok(numar) = argumente[i].parse::<usize>() {
                        max_linii = numar;
                    } else {
                        eprintln!("Valoarea de dupa -max nu este un numar.\nFormat acceptat: {} <adresa_directot/fisier> <sir_cautat> [-count] [-ignore] [-max : numarul maxim de linii analizate] [-regex]",argumente[0]);
                        std::process::exit(2);
                    }
                } else {
                    eprintln!("Format necunoscut! Nu este precizat numarul de linii dupa argumentul '-max'.");
                    std::process::exit(3);
                }
            }
            _ => {
                eprintln!("Format acceptat: {} <adresa_director/fisier> <sir_cautat> [-count] [-ignore] [-max : numarul maxim de linii analizate] [-regex].\n Rulare cu 'help' pentru mai multe informatii.", argumente[0]);
                std::process::exit(1);
            }
        }
        i += 1;
    }
    parcurgere_recursiva(
        director,
        sir_cautat,
        &mut rezultat,
        count,
        ignore,
        max_linii,
        regex,
    );
    if !rezultat {
        println!("Nu s-a gasit!");
    }
}
