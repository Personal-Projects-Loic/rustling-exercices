// TODO: Multiply each element in the `input` slice by 2 and push it to
// the `output` vector.

fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for &element in input {
        output.push(element * 2);
    }
    output
}

fn vec_map(input: &[i32], coefficient: i32) -> Vec<i32> {
    input.iter().map(|element| element * coefficient).collect()
}

fn vec_del(mut input: Vec<i32>) -> Vec<i32> {
    input.pop(); // pop supprime le dernier element du vecteur
                 // si je veux supprimer un element en particulier, utiliser la methode remove
    input
}

fn vec_add(mut input: Vec<i32>, num: i32) -> Vec<i32> {
    input.push(num);
    input
}

fn main() {
    let input = [1, 2, 3, 4, 5];
    let res = vec_loop(&input);
    let res1 = vec_map(&input, 5);

    println!("Ici j'ajoute {:?}", vec_add(res.clone(), 32)); // N'a pas d'effet sur les vecteurs res et res1
    println!("Ici je supprime {:?}", vec_del(res1.clone())); // On clone le vecteur en fait

    println!("{:?}", res);
    println!("{:?}", res1);
}
