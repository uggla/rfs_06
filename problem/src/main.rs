//                 scope -----v
fn imil_say(sentence: String) {
    println!("{}", sentence);
}

fn main() {
    let msg = String::from("RFS rules !");

    imil_say(msg);

    // Say it again loud !
    imil_say(msg);
    // ^-- le problème est le même que dans rfs_05, msg est
    // moved (perte de l'ownership) dans la fonction imil_say() lors du premiers appel
    // la variable msg ne peut donc plus être utilisé pour le second appel de la fonction.
}
