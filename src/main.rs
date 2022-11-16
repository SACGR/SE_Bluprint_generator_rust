use std::fs::File;
use std::io::prelude::*;
use std::io::Write;

fn main(){
    //Öpnar alla filler som behöver öpnas
    let mut file = File::open("Untitled.txt").expect("F.1");
    let mut kordinater = String::new();
    file.read_to_string(&mut kordinater).expect("F.2");
    
    let mut file = File::open("Början.txt").expect("F.1");
    let mut början = String::new();
    file.read_to_string(&mut början).expect("F.2");

    let mut file = File::open("Mitten.txt").expect("F.1");
    let mut mitten = String::new();
    file.read_to_string(&mut mitten).expect("F.2");

    let mut file = File::open("Slutet.txt").expect("F.1");
    let mut slut = String::new();
    file.read_to_string(&mut slut).expect("F.2");

    //skapar fillen vi leger till alla ingredienser till 
    let mut result = String::new();

 
    //skapar fillen som ska genereras 
    let mut resultat_fill = File::create("bp.sbc").expect("fillen kunde inte skapas");
    //let mut positoner = Vec::new();

    result.push_str(&början);
    //iterear igenom alla rader i referens fillen 
    for _line in kordinater.lines() {
        
        let positoner:Vec<&str> = _line.split("_").collect();
        
        let mut steg = String::new();
        steg.push_str(&mitten);
        steg = steg.replace("xan", positoner[0]);
        steg = steg.replace("yan", positoner[1]);
        steg = steg.replace("zan", positoner[2]);

        result.push_str(&steg);
        //println!("{}",positoner[0]);
        println!("x{} y{} z{} \n",positoner[0],positoner[1],positoner[2]);
        
    }
    result.push_str(&slut);
    
//skriver till fillen allt vi har jort 
    resultat_fill.write_all(result.as_bytes()).expect("jag vet inte");

}