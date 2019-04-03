pub fn brawl(token: &str) -> Option<&str> {
    match token.as_ref() {
        "bowser"          => Some("Bowser"),
        "captain"         => Some("Captain%20Falcon"),
        "captainfalcon"   => Some("Captain%20Falcon"),
        "charizard"       => Some("Charizard"),
        "diddy"           => Some("Diddy%20Kong"),
        "diddykong"       => Some("Diddy%20Kong"),
        "donkey"          => Some("Donkey%20Kong"),
        "donkeykong"      => Some("Donkey%20Kong"),
        "dk"              => Some("Donkey%20Kong"),
        "falco"           => Some("Falco"),
        "fox"             => Some("Fox"),
        "game"            => Some("Game%20&%20Watch"),
        "game&watch"      => Some("Game%20&%20Watch"),
        "gameandwatch"    => Some("Game%20&%20Watch"),
        "gamewatch"       => Some("Game%20&%20Watch"),
        "g&w"             => Some("Game%20&%20Watch"),
        "gaw"             => Some("Game%20&%20Watch"),
        "gw"              => Some("Game%20&%20Watch"),
        "ganondorf"       => Some("Ganondorf"),
        "ganon"           => Some("Ganondorf"),
        "dorf"            => Some("Ganondorf"),
        "giga bowser"     => Some("Giga%20Bowser"),
        "gigabowser"      => Some("Giga%20Bowser"),
        "gb"              => Some("Giga%20Bowser"),
        "iceclimbers"     => Some("Ice%20Climbers"),
        "iceclimber"      => Some("Ice%20Climbers"),
        "ice"             => Some("Ice%20Climbers"),
        "ic"              => Some("Ice%20Climbers"),
        "ics"             => Some("Ice%20Climbers"),
        "ike"             => Some("Ike"),
        "ivysaur"         => Some("Ivysaur"),
        "ivy"             => Some("Ivysaur"),
        "jigglypuff"      => Some("Jigglypuff"),
        "jiggly"          => Some("Jigglypuff"),
        "jiggs"           => Some("Jigglypuff"),
        "puff"            => Some("Jigglypuff"),
        "kingdedede"      => Some("King%20Dedede"),
        "king"            => Some("King%20Dedede"),
        "dedede"          => Some("King%20Dedede"),
        "d3"              => Some("King%20Dedede"),
        "ddd"             => Some("King%20Dedede"),
        "kd"              => Some("King%20Dedede"),
        "kirby"           => Some("Kirby"),
        "link"            => Some("Link"),
        "lucario"         => Some("Lucario"),
        "lucas"           => Some("Lucas"),
        "luigi"           => Some("Luigi"),
        "mario"           => Some("Mario"),
        "marth"           => Some("Marth"),
        "meta"            => Some("Meta%20Knight"),
        "metaknight"      => Some("Meta%20Knight"),
        "mk"              => Some("Meta%20Knight"),
        "ness"            => Some("Ness"),
        "olimar"          => Some("Olimar"),
        "peach"           => Some("Peach"),
        "pikachu"         => Some("Pikachu"),
        "pit"             => Some("Pit"),
        "rob"             => Some("R.O.B"),
        "r.o.b"           => Some("R.O.B"),
        "samus"           => Some("Samus"),
        "sheik"           => Some("Sheik"),
        "solid"           => Some("Snake"),
        "solidsnake"      => Some("Snake"),
        "snake"           => Some("Snake"),
        "sonic"           => Some("Sonic"),
        "squirtle"        => Some("Squirtle"),
        "toon"            => Some("Toon%20Link"),
        "toonlink"        => Some("Toon%20Link"),
        "tink"            => Some("Toon%20Link"),
        "tl"              => Some("Toon%20Link"),
        "wario"           => Some("Wario"),
        "wario-man"       => Some("Wario-Man"),
        "warioman"        => Some("Wario-Man"),
        "wolf"            => Some("Wolf"),
        "yoshi"           => Some("Yoshi"),
        "zelda"           => Some("Zelda"),
        "zero"            => Some("Zero%20Suit%20Samus"),
        "zerosuitsamus"   => Some("Zero%20Suit%20Samus"),
        "zss"             => Some("Zero%20Suit%20Samus"),
        _                 => None,
    }
}

pub fn pm(token: &str) -> Option<&str> {
    match token.as_ref() {
        "mewtwo" => Some("Mewtwo"),
        "mew2"   => Some("Mewtwo"),
        "m2"     => Some("Mewtwo"),
        "roy"    => Some("Roy"),
        _        => None,
    }
}

pub fn lxp(token: &str) -> Option<&str> {
    match token.as_ref() {
        "doctor"        => Some("DoctorMario"),
        "doctormario"   => Some("DoctorMario"),
        "doc"           => Some("DoctorMario"),
        "dr"            => Some("DoctorMario"),
        "drmario"       => Some("DoctorMario"),
        "dm"            => Some("DoctorMario"),
        "mage"          => Some("Ganon-Mage"),
        "mageganon"     => Some("Ganon-Mage"),
        "mg"            => Some("Ganon-Mage"),
        "geno"          => Some("Geno"),
        "lucina"        => Some("Lucina"),
        "metal"         => Some("MetalSonic"),
        "metalsonic"    => Some("MetalSonic"),
        "ms"            => Some("MetalSonic"),
        "pichu"         => Some("Pichu"),
        "ridley"        => Some("Ridley-Classic"),
        "classic"       => Some("Ridley-Classic"),
        "classicridley" => Some("Ridley-Classic"),
        "ridleyclassic" => Some("Ridley-Classic"),
        "modern"        => Some("Ridley-Modern"),
        "modernridley"  => Some("Ridley-Modern"),
        "ridleymodern"  => Some("Ridley-Modern"),
        "shadow"        => Some("Shadow"),
        "waluigi"       => Some("Waluigi"),
        "yl"            => Some("YoungLink"),
        "young"         => Some("YoungLink"),
        "younglink"     => Some("YoungLink"),
        "yink"          => Some("YoungLink"),
        _               => None,
    }
}
