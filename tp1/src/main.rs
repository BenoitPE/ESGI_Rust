use chrono;
use chrono::Datelike;

#[derive(Debug, Clone)]
enum Genre {
    Fiction,
    Histoire,
    Fantasy,
    Informatique,
}

#[derive(Debug, Clone)]
enum ResultatDivision {
    DivisionParZero,
    DivisionCorrecte(f32),
}

#[derive(Debug, Clone)]
struct Livre {
    titre: String,
    annee_publication: i32,
    genre: Genre,
}

fn mad(a: i32, b: i32, c: i32) -> i32 {
    a * b + c
}

fn sum_from_to_While(a: i32, b: i32) -> i32 {
    let mut resultat = 0;
    let mut i = a;

    while i <= b {
        resultat += i;
        i += 1;
    }

    resultat
}

fn sum_from_to_For(a: i32, b: i32) -> i32 {
    let mut resultat = 0;

    for i in a..b + 1 {
        resultat += i;
    }

    resultat
}

fn sum_from_to_Recursive(a: i32, b: i32) -> i32 {
    if a == b {
        a
    } else {
        a + sum_from_to_Recursive(a + 1, b)
    }
}

fn age_livre(livre: Livre) -> i32 {
    chrono::Utc::now().year() - livre.annee_publication
}

fn note_livre(livre: Livre) -> i32 {
    let note = livre.titre.len() as i32 + livre.annee_publication;
    match livre.genre {
        Genre::Fiction => note * 12,
        Genre::Histoire => note * 2,
        Genre::Fantasy => note * 36,
        Genre::Informatique => note * 41,
    }
}

fn division_avec_enum(a: f32, b: f32) -> ResultatDivision {
    if b == 0.0 {
        ResultatDivision::DivisionParZero
    } else {
        ResultatDivision::DivisionCorrecte(a / b)
    }
}

fn division_avec_Option(a: f32, b: f32) -> Option<f32> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    println!("\n----- Fonctions -----");
    println!("mad(2,3,4): {}", mad(2, 3, 4));
    println!("sum_from_to_While(2, 5): {}", sum_from_to_While(2, 5));
    println!("sum_from_to_For(2, 5): {}", sum_from_to_For(2, 5));
    println!(
        "sum_from_to_Recursive(2, 5): {}",
        sum_from_to_Recursive(2, 5)
    );

    println!("\n----- Creation de struct et d'enum -----");
    let book: Livre = Livre {
        titre: "One Piece Tome 01".to_string(),
        annee_publication: 2004,
        genre: Genre::Fantasy,
    };
    println!("Livre: {:?}", book);
    println!("Age du livre: {}", age_livre(book.clone()));
    println!("Note du livre: {}", note_livre(book.clone()));

    println!("\n----- Enum avancé -----");
    println!(
        "Division avec enum par zéro : {:?}",
        division_avec_enum(4.0, 0.0)
    );
    println!(
        "Division avec Option par zéro : {:?}",
        division_avec_Option(4.0, 0.0)
    );

    println!("\nDivision avec enum: {:?}", division_avec_enum(4.0, 2.0));
    println!("Division avec Option: {:?}", division_avec_Option(4.0, 2.0));
}
