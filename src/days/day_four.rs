pub fn day_four() {
    println!(
        "{:?}",
        include_str!(r"..\..\inputs\day_four.txt")
            .split("\r\n")
            .map(|line| line.split(|x| x == '-' || x == ',').collect::<Vec<&str>>())
            .map(
                |x| ((x[0].parse::<u16>().unwrap() >= x[2].parse::<u16>().unwrap()
                    && x[1].parse::<u16>().unwrap() <= x[3].parse::<u16>().unwrap())
                    || (x[0].parse::<u16>().unwrap() <= x[2].parse::<u16>().unwrap()
                        && x[1].parse::<u16>().unwrap() >= x[3].parse::<u16>().unwrap()))
                    as u16
            )
            .sum::<u16>()
    );
}
