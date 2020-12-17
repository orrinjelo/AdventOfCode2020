use regex::Regex;
use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use crate::util::RetType;

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

///  Passport
struct Passport {
   byr: String, // Birth Year
   iyr: String, // Issue year
   eyr: String, // Expiration Year
   hgt: String, // Height
   hcl: String, // Hair Color
   ecl: String, // Eye Color
   pid: String, // Passport ID
   cid: String, // Country ID
   is_valid: bool
}

impl Passport {
    fn new(string_map: Vec<String>, validate: bool) -> Passport {
        let mut passport = Passport {
           byr: "".to_string(),
           iyr: "".to_string(),
           eyr: "".to_string(),
           hgt: "".to_string(),
           hcl: "".to_string(),
           ecl: "".to_string(),
           pid: "".to_string(),
           cid: "".to_string(),
           is_valid: false,
        };

        let mut field_count = 0;
        for line in string_map {
            let re = Regex::new(r"([a-z]{3}):(#?[a-z,0-9]*)\s?").unwrap();
            let cap = re.find_iter(&line);

            for entry in cap {
                let parsed_entry: Vec<&str> = entry.as_str().trim().split(":").collect();
                trace!("{},{}", parsed_entry[0], parsed_entry[1]);
                field_count += 1;
                match parsed_entry[0] {
                    "byr" => passport.byr = parsed_entry[1].to_string(),
                    "iyr" => passport.iyr = parsed_entry[1].to_string(),
                    "eyr" => passport.eyr = parsed_entry[1].to_string(),
                    "hgt" => passport.hgt = parsed_entry[1].to_string(),
                    "hcl" => passport.hcl = parsed_entry[1].to_string(),
                    "ecl" => passport.ecl = parsed_entry[1].to_string(),
                    "pid" => passport.pid = parsed_entry[1].to_string(),
                    "cid" => passport.cid = parsed_entry[1].to_string(),
                    _ => {}
                }
            }
        }

        if field_count == 8 {
            passport.is_valid = true;
        } else if field_count == 7 && passport.cid == "" {
            passport.is_valid = true;
        }

        if validate {
            passport.validate();
        }

        return passport;
    }

    /// byr (Birth Year) - four digits; at least 1920 and at most 2002.
    /// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    /// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    /// hgt (Height) - a number followed by either cm or in:
    ///     If cm, the number must be at least 150 and at most 193.
    ///     If in, the number must be at least 59 and at most 76.
    /// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    /// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    /// pid (Passport ID) - a nine-digit number, including leading zeroes.
    /// cid (Country ID) - ignored, missing or not.
    fn validate(&mut self) {
        // Birth year
        if self.byr.len() != 4 || self.byr.parse::<u32>().unwrap() < 1920 || self.byr.parse::<u32>().unwrap() > 2002 {
            self.is_valid = false;
        }
        // Issue year
        if self.iyr.len() != 4 || self.iyr.parse::<u32>().unwrap() < 2010 || self.iyr.parse::<u32>().unwrap() > 2020 {
            self.is_valid = false;
        }
        // Exp. Year
        if self.eyr.len() != 4 || self.eyr.parse::<u32>().unwrap() < 2010 || self.eyr.parse::<u32>().unwrap() > 2030 {
            self.is_valid = false;
        }
        // Height
        let he_re = Regex::new(r"(\d+)(cm|in)").unwrap();
        if !he_re.is_match(&self.hgt) {
            self.is_valid = false;
        } else {
            let cap = he_re.captures(&self.hgt).unwrap();
            if &cap[2] == "cm"  {
                if cap[1].parse::<u32>().unwrap() < 150 || cap[1].parse::<u32>().unwrap() > 193 {
                    self.is_valid = false;
                }
            } else if &cap[2] == "in" {
                if cap[1].parse::<u32>().unwrap() < 59 || cap[1].parse::<u32>().unwrap() > 76 {
                    self.is_valid = false;
                }       
            } else {
                self.is_valid = false;
            }
        }
        // Hair Color
        let hc_re = Regex::new(r"#[0-9a-f]{6}").unwrap();
        if !hc_re.is_match(&self.hcl) || self.hcl.len() != 7 {
            self.is_valid = false;
        }
        // Eye Color
        match &self.ecl[..] {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {},
            _ => self.is_valid = false
        }
        // PID
        let pid_re = Regex::new(r"\d{9}").unwrap();
        if !pid_re.is_match(&self.pid) || self.pid.len() != 9 {
            self.is_valid = false;
        }
    }
}

/// This may or may not be "on the books"
///  Look, all we do here is take information from here and
///  put it over there.  Nothing shady, just a business man trying--
///  I mean, a government official--trying to make his way in this
///  world.
fn semi_questionable_passport_factory(string_map: Vec<String>, validate: bool) -> Vec<Passport> {
    let mut temp_vec: Vec<String> = Vec::new();
    let mut passport_vec: Vec<Passport> = Vec::new();

    for line in string_map {
        if line.len() == 0 {
            passport_vec.push(Passport::new(temp_vec.clone(), validate));
            temp_vec.clear();
        } else {
            temp_vec.push(line);
        }
    }
    passport_vec.push(Passport::new(temp_vec.clone(), validate));

    return passport_vec;
}

fn count_valid_passports(passports: Vec<Passport>) -> u32 {
    let mut count = 0;
    for passport in passports {
        if passport.is_valid {
            count += 1;
        }
    }
    return count;
}

/**
 *  Problem #04, part 1
 *  You arrive at the airport only to realize that you grabbed your 
 *  North Pole Credentials instead of your passport. While these documents 
 *  are extremely similar, North Pole Credentials aren't issued by a country 
 *  and therefore aren't actually valid documentation for travel in most of 
 *  the world.
 * It seems like you're not the only one having problems, though; a very long 
 *  line has formed for the automatic passport scanners, and the delay could 
 *  upset your travel itinerary.
 * Due to some questionable network security, you realize you might be able 
 *  to solve both of these problems at the same time.
 * The automatic passport scanners are slow because they're having trouble 
 *  detecting which passports have all required fields.
 */
pub fn problem_041(input: Vec<String>) -> RetType {
    return RetType::U32(count_valid_passports(semi_questionable_passport_factory(input, false)));
}

/**
 *  Problem #04, part 2
 *  The line is moving more quickly now, but you overhear airport 
 *  security talking about how passports with invalid data are getting 
 *  through. Better add some data validation, quick!
 */
pub fn problem_042(input: Vec<String>) -> RetType {
    return RetType::U32(count_valid_passports(semi_questionable_passport_factory(input, true)));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        match env_logger::try_init() {
            Ok(_) => {
                info!("Initializing logging...");
            },
            Err(_) => {

            }
        }
    }

    #[test]
    fn test_passport_creation() {
        init();
        let passport_string = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
            "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string()
        ];

        let passport = Passport::new(passport_string, false);
        assert!(passport.is_valid);

        trace!("Second entry");

        let passport_string2 = vec![
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884".to_string(),
            "hcl:#cfa07d byr:1929".to_string()
        ];
        let passport2 = Passport::new(passport_string2, false);
        assert!(!passport2.is_valid);
    }

    #[test]
    /// Test multiple passports parsing
    /// The first passport is valid - all eight fields are present. The second passport is 
    ///  invalid - it is missing hgt (the Height field).
    ///
    /// The third passport is interesting; the only missing field is cid, so it looks like 
    ///  data from North Pole Credentials, not a passport at all! Surely, nobody would mind 
    ///  if you made the system temporarily ignore missing cid fields. Treat this "passport" 
    ///  as valid.
    ///
    /// The fourth passport is missing two fields, cid and byr. Missing cid is fine, but 
    ///  missing any other field is not, so this passport is invalid.     
    fn test_passport_factory() {
        init();
        let passports_strings = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
            "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
            "".to_string(),
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884".to_string(),
            "hcl:#cfa07d byr:1929".to_string(),
            "".to_string(),
            "hcl:#ae17e1 iyr:2013".to_string(),
            "eyr:2024".to_string(),
            "ecl:brn pid:760753108 byr:1931".to_string(),
            "hgt:179cm".to_string(),
            "".to_string(),
            "hcl:#cfa07d eyr:2025 pid:166559648".to_string(),
            "iyr:2011 ecl:brn hgt:59in".to_string(),
        ];

        let passports = semi_questionable_passport_factory(passports_strings, false);
        assert!(passports[0].is_valid);
        assert!(!passports[1].is_valid);
        assert!(passports[2].is_valid);
        assert!(!passports[3].is_valid);
        assert_eq!(count_valid_passports(passports), 2);
    }

    #[test]
    fn test_better_passport_validation() {
        init();
        let invalid_passport_strings = vec![
            "eyr:1972 cid:100".to_string(),
            "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926".to_string(),
            "".to_string(),
            "iyr:2019".to_string(),
            "hcl:#602927 eyr:1967 hgt:170cm".to_string(),
            "ecl:grn pid:012533040 byr:1946".to_string(),
            "".to_string(),
            "hcl:dab227 iyr:2012".to_string(),
            "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277".to_string(),
            "".to_string(),
            "hgt:59cm ecl:zzz".to_string(),
            "eyr:2038 hcl:74454a iyr:2023".to_string(),
            "pid:3556412378 byr:2007".to_string(),
        ];

        let invalid_passports = semi_questionable_passport_factory(invalid_passport_strings, true);
        assert_eq!(count_valid_passports(invalid_passports), 0);

        let valid_passport_strings = vec![
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980".to_string(),
            "hcl:#623a2f".to_string(),
            "".to_string(),
            "eyr:2029 ecl:blu cid:129 byr:1989".to_string(),
            "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm".to_string(),
            "".to_string(),
            "hcl:#888785".to_string(),
            "hgt:164cm byr:2001 iyr:2015 cid:88".to_string(),
            "pid:545766238 ecl:hzl".to_string(),
            "eyr:2022".to_string(),
            "".to_string(),
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719".to_string(),
        ];

        let valid_passports = semi_questionable_passport_factory(valid_passport_strings, true);
        assert_eq!(count_valid_passports(valid_passports), 4);
    }    
}