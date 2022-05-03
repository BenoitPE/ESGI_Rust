use std::result;

fn isbig(input: &String, taille: usize) -> bool {
    input.len() > taille
}

fn inverse(v: f32) -> f32 {
    1.0 / v
}

fn safe_inverse(v: f32) -> Option<f32> {
    if v == 0.0 {
        None
    } else {
        Some(1.0 / v)
    }
}

fn somme(values: &Vec<u32>) -> u32 {
    let mut resultat = 0;
    for i in values {
        resultat += i;
    }
    resultat
}

fn maximum(values: &Vec<u32>) -> Option<&u32> {
    values.iter().max()
}

fn main() {
    let string1 = "Hello, world!".to_string();

    println!("\n----- String & Option -----");
    println!(
        "\"{}\" 's length is bigger than {} ? = {}",
        &string1,
        10,
        isbig(&string1, 10)
    );
    println!(
        "\"{}\" 's length is bigger than {} ? = {}",
        &string1,
        40,
        isbig(&string1, 40)
    );

    println!("Inverse de 2.0, {:?}", inverse(2.0));
    println!("Inverse de 0, {:?}", inverse(0.0));
    println!("Inverse safe de 2.0, {:?}", safe_inverse(2.0));
    println!("Inverse safe de 0.0, {:?}", safe_inverse(0.0));

    println!("\n----- Vec -----");
    let mut v = vec![1, 2, 3, 4];
    println!("Afficher un vecteur: {:?}", &v);

    println!("Afficher un vecteur avec une boucle For");
    for i in &v {
        println!("-\t{}", i);
    }

    v.push(5);
    println!("Vecteur après ajout d'une donnée: {:?}", &v);
    println!(
        "Fonction somme(v), la somme des valeurs du vecteur est: {}",
        somme(&v)
    );

    match maximum(&v) {
        None => println!("Fonction maximum impossible sur le vecteur spécifié"),
        Some(x) => println!("Fonction maximum(v), la valeur max du vecteur est: {}", x),
    }
}
