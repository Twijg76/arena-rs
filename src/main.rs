use std::io;
//use rand::Rng;

struct Persoon {
    naam: String,
    inleven: bool,
    leven: u8,
    kracht: u8,
    snelheid: u8,
    verstand: u8,
    ervaring: u8,
}

fn make_persoon(naam: String) -> Persoon {
    Persoon {
        naam,
        inleven: true,
        leven: 10,
        kracht: 0,
        snelheid: 0,
        verstand: 0,
        ervaring: 0,
    }
}

fn main() {
    println!("Welkom in de arena, zo meteen moogt ge gaan vechten maar eerst even nog wat papierwerk. Wat is uw naam?");
    let mut naam = String::new();
    io::stdin().read_line(&mut naam).expect("Kan input niet lezen.");
    let naam = naam.trim();
    
    let mut mc = make_persoon(naam.to_string());


    println!("Ok, {}, hebt ge een bepaalde kwaliteit?", mc.naam);
    println!("gehard: 0, sterk: 1, snel: 2, slim: 3");
    
    let mut kwal = String::new();
    io::stdin().read_line(&mut kwal).expect("Kan input niet lezen.");
    let kwal: u8 = kwal.trim().parse().expect("Een getal a.u.b.");
    
    match kwal{
        0=>mc.leven+= 5,
        1=>mc.kracht+= 1,
        2=>mc.snelheid+= 1,
        3=>mc.verstand+= 1,
        _=>println!("Geen geldig getal"),
    }

    //let willekeurig = rand::thread_rng().gen_range(1..101);
}
