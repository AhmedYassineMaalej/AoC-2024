fn det(a: i64, b: i64, c: i64, d: i64) -> i64 {
    // calculate
    // |a c|
    // |b d|
    a * d - b * c
}

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut result = 0;
    for section in input.split("\r\n\r\n") {
        let mut lines_iter = section.lines();
        let line1 = lines_iter.next().unwrap();
        let line2 = lines_iter.next().unwrap();
        let line3 = lines_iter.next().unwrap();

        let xa: i64 = line1[12..14].parse().unwrap();
        let ya: i64 = line1[18..20].parse().unwrap();

        let xb: i64 = line2[12..14].parse().unwrap();
        let yb: i64 = line2[18..20].parse().unwrap();

        let comma_idx = line3.find(',').unwrap();
        let xp: i64 = line3[9..comma_idx].parse().unwrap();
        let yp: i64 = line3[(comma_idx + 4)..].parse().unwrap();

        let d = det(xa, xb, ya, yb);

        if d == 0 {
            continue;
        }

        let d1 = det(xp, yp, xb, yb);

        if d1 % d != 0 {
            continue;
        }
        // times button A is pushed
        let a = d1 / d;

        let d2 = det(xa, ya, xp, yp);

        if d2 % d != 0 {
            continue;
        }
        // times button B is pushed
        let b = d2 / d;

        result += a * 3 + b;
    }

    result
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let mut result = 0;
    for section in input.split("\r\n\r\n") {
        let mut lines_iter = section.lines();
        let line1 = lines_iter.next().unwrap();
        let line2 = lines_iter.next().unwrap();
        let line3 = lines_iter.next().unwrap();

        let xa: i64 = line1[12..14].parse().unwrap();
        let ya: i64 = line1[18..20].parse().unwrap();

        let xb: i64 = line2[12..14].parse().unwrap();
        let yb: i64 = line2[18..20].parse().unwrap();

        let comma_idx = line3.find(',').unwrap();
        let xp: i64 = line3[9..comma_idx].parse::<i64>().unwrap() + 10_000_000_000_000;
        let yp: i64 = line3[(comma_idx + 4)..].parse::<i64>().unwrap() + 10_000_000_000_000;

        let d = det(xa, xb, ya, yb);

        if d == 0 {
            continue;
        }

        let d1 = det(xp, yp, xb, yb);

        if d1 % d != 0 {
            continue;
        }
        // times button A is pushed
        let a = d1 / d;

        let d2 = det(xa, ya, xp, yp);

        if d2 % d != 0 {
            continue;
        }
        // times button B is pushed
        let b = d2 / d;

        result += a * 3 + b;
    }

    result
}
