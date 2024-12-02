struct NameChange(String, String);

impl From<[&str; 2]> for NameChange {
    fn from(value: [&str; 2]) -> Self {
        Self(value[0].to_string(), value[1].to_string())
    }
}

fn find_idx_str(bytes: &[u8], target: &str) -> Option<usize> {
    let target_bytes = target.as_bytes();
    let mut idx = 0;

    for i in 0..bytes.len() {
        if bytes[i] == target_bytes[idx] {
            idx += 1;
        } else {
            idx = 0;
        }

        if idx == target_bytes.len() {
            return Some(i - target_bytes.len() + 1);
        }
    }
    None
}

fn replace_inplace(bytes: &mut [u8], at: usize, size: usize, target: &str) {
    let t_bytes = target.as_bytes();
    assert!(
        t_bytes.len() <= 11,
        "{target} ({}) has more than 11 chars",
        t_bytes.len()
    );
    for i in at..(at + size) {
        bytes[i] = 0;
    }
    for i in 0..t_bytes.len() {
        let idx = at + i;
        bytes[idx] = t_bytes[i];
    }
}

fn main() {
    let bytes = include_bytes!("savefile");
    let mut bytes = bytes.clone();
    let names: Vec<NameChange> = vec![
        ["Equipo Che", "Valencia CF"],
        ["Culees", "Barcelona"],
        ["Depor", "Deportivo"],
        ["Merengues", "Real Madrid"],
        ["Atletic", "Bilbao"],
        ["Sevillistas", "Sevilla FC"],
        ["El Rayo", "Atletico"],
        ["Submarino", "Villarreal"],
        ["Beticos", "Real Betis"],
        ["Malagusitas", "Malaga CF"],
        ["Mallorquines", "Mallorca"],
        ["Equipo Mano", "Zaragoza"],
        ["Club Navarro", "CA Osasuna"],
        ["Club Alba", "Balompie"],
        ["Sanse", "Sociedad"],
        ["Periquitos", "Espanyol"],
        ["Racinguistas", "Santander"],
        ["Ponenti", "Levante UD"],
        ["Lefade", "Getafe CF"],
        ["Foira", "Huelva"],
        ["Pucelanos", "Valladolid"],
        ["Celtinas", "Vigo"],
        ["Equipo Pimentonero", "Real Murcia"],
        ["Lombardia", "AC Milan"],
        ["Lupacchiotti", "AS Roma"],
        ["Piemonte", "Juventus"],
        ["Milanese", "Inter Milan"],
        ["Biancoscudati", "Parma"],
        ["Aquilotti", "Lazio"],
        ["Friulani", "Udinese"],
        ["Cerchiati", "Sampdoria"],
        ["Scaligeri", "Chievo"],
        ["Leccesi", "Lecce"],
        ["Rondinelle", "Brescia"],
        ["Dotti", "Bologna"],
        ["Calabresi", "Reggina"],
        ["Senensi", "Siena"],
        ["Giaquinti", "Palermo"],
        ["Mori", "Cagliari"],
        ["Fortini", "Livorno"],
        ["Punticci", "Messina"],
        ["Orobici", "Atalanta"],
        ["Gigliati", "Fiorentina"],
        ["Grifoni", "Perugia"],
        ["Canarini", "Modena"],
        ["Azzurrini", "Empoli"],
        ["Azregum", "Arsenal"],
        ["Tefley", "Chelsea"],
        ["McKesta U", "Man Utd"],
        ["Rigaloose", "Liverpool"],
        ["Neocastin", "Newcastle"],
        ["Arlonvinnan", "Aston Villa"],
        ["Darlson", "Charlton"],
        ["Bobron", "Bolton"],
        ["Faldam", "Fulham"],
        ["Balgminidas", "Birmingham"],
        ["Minolsika", "Middlesboro"],
        ["Sambankton", "Southampton"],
        ["Boltksas", "Portsmouth"],
        ["Tordram H", "Tottenham"],
        ["Bracets", "Blackburn"],
        ["McKesta C", "Man City"],
        ["Egerson", "Everton"],
        ["Tordmich", "Norwich"],
        ["West Tordmich", "West Brom"],
        ["Bethrus", "C.Palace"],
        ["Lekger", "Leicester"],
        ["Ribble", "Leeds Utd"],
        ["Mulderverton", "Wolves"],
        ["W.Glaizen", "W.Bremen"],
        ["B.Moolten", "B.Munich"],
        ["B.Rekrangboomen", "Bayer"],
        ["VfB Stolgzten", "Stuttgart"],
        ["VfL Bolzun", "VfL Bochum"],
        ["B.Droglmkutt", "B.Dortmund"],
        ["Damke 03", "Schalke"],
        ["Hamglzmer SV", "HSV"],
        ["H.Conkstock", "H.Rostock"],
        ["VfL Firisburg", "Wolfsburg"],
        ["Burdendladbach", "Gladbach"],
        ["H. Bekwin", "H. Berlin"],
        ["SC Zenktzburg", "SC Freiburg"],
        ["Hamgrozz", "Hannover"],
        ["Krezzken", "Kaiserslaut"],
        ["1FC Nubrun", "Nuremberg"],
        ["A Bligzen", "A Bielefeld"],
        ["Retiz", "Mainz"],
        ["E.Flankfoot", "E.Frankfurt"],
        ["1783 Multen", "TSV Munich"],
        ["1FC Kelfem", "1.FC Koln"],
        ["Pinon", "Lyon"],
        ["Baldankelmen", "ParisSG"],
        ["Ronario", "Monaco"],
        ["Oterunne", "Auxerre"],
        ["Seissu", "Souchaux"],
        ["Billantamos", "Nantes"],
        ["O. Maussessun", "Marseille"],
        ["Grandes", "Lens"],
        ["Mirenux", "Rennais"],
        ["Roele", "Lille"],
        ["Tils", "Nice"],
        ["Nalieaux", "Bordeaux"],
        ["Kalibrug", "Strasbourg"],
        ["Essmell", "Metz"],
        ["AC Sassion", "AC Ajaccio"],
        ["Loomove", "Toulouse"],
        ["Bastillas", "Bastia"],
        ["Saint-Didoux", "S-Etienne"],
        ["Contez", "Caen"],
        ["Tirsen", "Istres"],
        ["Gagaloon", "Guingamp"],
        ["Meln", "LeMans"],
        ["Rontelia", "Montpellier"],
    ]
    .into_iter()
    .map(|x| x.into())
    .collect();

    for name in names {
        if let Some(idx) = find_idx_str(&bytes, &name.0) {
            replace_inplace(&mut bytes, idx, name.0.len(), &name.1);
        } else {
            println!("Cannot find {}", name.0);
        }
    }

    std::fs::write("BESLES-53846N0", &bytes).unwrap();
    println!("Done");
}
