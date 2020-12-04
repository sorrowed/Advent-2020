use crate::common;

struct DocumentField {
    name: String,
    value: String,
}

impl DocumentField {
    pub fn new(name: &str, value: &str) -> DocumentField {
        DocumentField {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

struct Document {
    fields: Vec<DocumentField>,
}

impl std::fmt::Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for field in &self.fields {
            write!(f, "({}:{})", field.name, field.value).unwrap();
        }
        writeln!(f, "")
    }
}

fn between<T: PartialOrd>(v: T, l: T, h: T) -> bool {
    v >= l && v <= h
}

impl Document {
    pub fn new() -> Document {
        Document { fields: vec![] }
    }

    pub fn is_valid(&self) -> bool {
        let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

        required
            .iter()
            .all(|s| self.fields.iter().any(|f| f.name == *s))
    }

    pub fn field(&self, name: &str) -> Option<&DocumentField> {
        self.fields.iter().find(|v| v.name == name)
    }

    pub fn is_strict_valid(&self) -> bool {
        self.is_valid()
            && Self::validate_byr(&self.field("byr").unwrap().value)
            && Self::validate_iyr(&self.field("iyr").unwrap().value)
            && Self::validate_eyr(&self.field("eyr").unwrap().value)
            && Self::validate_hgt(&self.field("hgt").unwrap().value)
            && Self::validate_hcl(&self.field("hcl").unwrap().value)
            && Self::validate_ecl(&self.field("ecl").unwrap().value)
            && Self::validate_pid(&self.field("pid").unwrap().value)
    }

    fn validate_byr(v: &str) -> bool {
        v.len() == 4 && between(v.parse::<i32>().unwrap(), 1920, 2002)
    }

    fn validate_iyr(v: &str) -> bool {
        v.len() == 4 && between(v.parse::<i32>().unwrap(), 2010, 2020)
    }

    fn validate_eyr(v: &str) -> bool {
        v.len() == 4 && between(v.parse::<i32>().unwrap(), 2020, 2030)
    }

    fn validate_hgt(v: &str) -> bool {
        let n = v[0..v.len() - 2].parse::<i32>().unwrap();
        let u = &v[v.len() - 2..v.len()];

        (u == "cm" && between(n, 150, 193)) || (u == "in" && between(n, 59, 76))
    }

    fn validate_hcl(v: &str) -> bool {
        v.len() == 7 && v.chars().enumerate().all(|(ix, c)| {
            (ix == 0 && c == '#')
                || (ix != 0
                    && c.is_ascii_hexdigit()
                    && (c.is_ascii_digit() || c.is_ascii_lowercase()))
        })
    }

    fn validate_ecl(v: &str) -> bool {
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v)
    }

    fn validate_pid(v: &str) -> bool {
        v.len() == 9 && v.chars().all(|c| c.is_ascii_digit())
    }
}

fn parse_document(input: &[String]) -> Document {
    let mut result = Document::new();

    for token in input {
        for field in token.split_whitespace().collect::<Vec<&str>>() {
            let f = field.split(':').collect::<Vec<&str>>();

            result.fields.push(DocumentField::new(f[0], f[1]));
        }
    }
    result
}

fn parse_documents(input: &Vec<String>) -> Vec<Document> {
    input
        .split(|s| s == "")
        .map(|s| parse_document(s))
        .collect::<Vec<Document>>()
}

#[test]
pub fn test() {
    let documents = parse_documents(&common::import("day4/test.txt"));

    let valid = documents.iter().filter(|d| d.is_valid()).count();
    println!("Valid documents in test: {}", valid);
}

pub fn part1() {
    let documents = parse_documents(&common::import("day4/input.txt"));

    let valid = documents.iter().filter(|d| d.is_valid()).count();
    println!("Valid documents in part 1: {}", valid);
}

pub fn part2() {
    let documents = parse_documents(&common::import("day4/input.txt"));

    let valid = documents.iter().filter(|d| d.is_strict_valid()).count();
    println!("Valid documents in part 2: {}", valid);
}
