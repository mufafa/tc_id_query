

pub mod tcid {
    use std::collections::VecDeque;
    use rand::prelude::*;


    fn parse_number_into_vector(value: u64) -> Vec<u32> {
        let mut values :VecDeque<u64> = VecDeque::new();
        let mut x = value;
        while {
            values.push_front(x % 10);
            x /= 10;
            x != 0
        } { }
        return values.iter().map(|&x| x as u32).collect::<Vec<u32>>();
    }

    fn parse_vector_into_number(v: Vec<u32>) -> u64 {
        let length = v.len();
        let mut x = 0;
        for n in 0..length  {
            let a = u64::pow(10, n as u32);
             x += a*(v[length-(n+1)] as u64);
        }
        return x as u64;
    }

    pub fn validate_idnumber(value: u64) -> Result<bool, String> {

        let values = parse_number_into_vector(value);

        //11 basamk olmalı
        if &values.len() != &11 {
            return Err("11. basamaklı olmalı".to_string());
        } 
        // İlk basamağı 0 olamaz.
        if &values[0] == &0 {
            return Err("ilk rakam 0  olamaz".to_string());
        }
        //1, 3, 5, 7 ve 9. hanenin rakamları toplamı A olsun. 2,4,6,8. hanenin rakamları toplamı da B olsun. Ax7 - B 'yi 10'a böldüğümüzde kalan (yani mod10) bize 10. basamağı verir.

        let a : u32 = values[0..values.len()-1].iter().step_by(2).sum();
        let b : u32 = values[1..values.len()-3].iter().step_by(2).sum();

        let first_checker = (a * 7 - b) % 10 ;
        
        if &values[9] != &(first_checker) {
            return Err("10. basamaktaki rakam doğru değil".to_string());
        } 

        //1, 2, 3, 4, 5, 6, 7, 8, 9 ve 10. hanelerin toplamını 10'a böldüğümüzde ise kalan (yine mod10) bize 11. basamağı verir.
        let c : u32 = values[0..values.len()-1].iter().sum();
        let second_checker = c % 10;

        if &values[10] != &second_checker {
            return Err("11. basamaktaki sayı doğru değil".to_string());
        }
        
        return Ok(true);
    }

    pub fn generate_id_number() -> u64 {
        let mut rng = rand::thread_rng();
        let mut values : Vec<u32> = Vec::new();

        values.push(rng.gen_range(1..9));
        for _ in 1..9  {
            values.push(rng.gen_range(0..9))
        }

        let a : u32 = values[0..values.len()].iter().step_by(2).sum();
        let b : u32 = values[1..values.len()-1].iter().step_by(2).sum();

        let first_checker = (a * 7 - b) % 10 ;

        values.push(first_checker);

        let c : u32 = values[0..values.len()].iter().sum();
        let second_checker = c % 10;
        values.push(second_checker);

        return parse_vector_into_number(values.clone());
    }


}