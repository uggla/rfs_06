// Solution 2, on clone la String. Ici peu de conséquences
// mais impact perf s'il y a beaucoup de données.
// Ex clone d'un Vec de 10M d'entrées...

fn imil_say(sentence: String) {
    println!("{}", sentence);
}

fn main() {
    let msg = String::from("RFS rules !");

    imil_say(msg.clone());
    //            ^-- on clone msg, clone retourne une copie de la valeur.

    // Say it again loud !
    imil_say(msg);
}
