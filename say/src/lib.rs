pub fn conculte_divisor_and_reminder_with_1000(number: u64) -> (u64, u64) {

    let reaminder = number.rem_euclid(1000);
    //println!{"Reminder is {}" , reaminder};
    let divisor = number.div_euclid(1000);
    //println!{"Divisor is {}" , divisor};
    (reaminder, divisor)
}

pub fn exchange_num_to_string(number :u64) -> String {

    println!{"Exchange number is {} .", number};
    let hundred = number.div_euclid(100);
    let ten = number.rem_euclid(100).div_euclid(10);
    let digit = number.rem_euclid(10);
    println!("Hundred is {}, tens is {}, digits is {} .", hundred, ten, digit);

    let mut return_string: String = String::new();
    if hundred > 0 {
        match hundred {
            1 => return_string.push_str("one"),
            2 => return_string.push_str("two"),
            3 => return_string.push_str("three"),
            4 => return_string.push_str("four"),
            5 => return_string.push_str("five"),
            6 => return_string.push_str("six"),
            7 => return_string.push_str("seven"),
            8 => return_string.push_str("eight"),
            9 => return_string.push_str("nine"),
            _ => (),
        }
        return_string.push_str(" hundred");
    }

    if ten > 0 {
        if ten != 1 {
            if hundred > 0 {
                return_string.push_str(" ");
            }
            match ten {
            2 => return_string.push_str("twenty"),
            3 => return_string.push_str("thirty"),
            4 => return_string.push_str("forty"),
            5 => return_string.push_str("fifty"),
            6 => return_string.push_str("sixty"),
            7 => return_string.push_str("seventy"),
            8 => return_string.push_str("eighty"),
            9 => return_string.push_str("ninty"),
            _ => (),
            }

            match digit {
            1 => return_string.push_str("-one"),
            2 => return_string.push_str("-two"),
            3 => return_string.push_str("-three"),
            4 => return_string.push_str("-four"),
            5 => return_string.push_str("-five"),
            6 => return_string.push_str("-six"),
            7 => return_string.push_str("-seven"),
            8 => return_string.push_str("-eight"),
            9 => return_string.push_str("-nine"),
            _ => (),
            }
        } else {
            
            if hundred > 0 {
                return_string.push_str(" ");
            }
            println!("Reminder is {} ." , number.rem_euclid(100));
            match number.rem_euclid(100) {
            1 => return_string.push_str("one"),
            2 => return_string.push_str("two"),
            3 => return_string.push_str("three"),
            4 => return_string.push_str("four"),
            5 => return_string.push_str("five"),
            6 => return_string.push_str("six"),
            7 => return_string.push_str("seven"),
            8 => return_string.push_str("eight"),
            9 => return_string.push_str("nine"),
            10 => return_string.push_str("ten"),
            11 => return_string.push_str("eleven"),
            12 => return_string.push_str("twelve"),
            13 => return_string.push_str("thirteen"),
            14 => return_string.push_str("fourteen"),
            15 => return_string.push_str("fifteen"),
            16 => return_string.push_str("sixteen"),
            17 => return_string.push_str("seventeen"),
            18 => return_string.push_str("eighteen"),
            19 => return_string.push_str("nineteen"),
            _ => (),
            }
        }
    } else if digit > 0 {
        if hundred > 0 {
            return_string.push_str(" ");
        }
            match digit {
            1 => return_string.push_str("one"),
            2 => return_string.push_str("two"),
            3 => return_string.push_str("three"),
            4 => return_string.push_str("four"),
            5 => return_string.push_str("five"),
            6 => return_string.push_str("six"),
            7 => return_string.push_str("seven"),
            8 => return_string.push_str("eight"),
            9 => return_string.push_str("nine"),
            _ => (),
            }
    }

    println!{"Exchange num {} to string is : {}", number, return_string};
    return_string
}

pub fn encode(n: u64) -> String {

    let mut return_string  = String::new();

    if n == 0 {
        return_string.push_str("zero");
    }

    if (n < u64::MIN) || (n > u64::MAX) {
        println!("Input para out of range !");
        //return_string.push_str("won't compile");
    }

    //Blow thousand
    let (reminder, divisor) = conculte_divisor_and_reminder_with_1000(n);
    let blow_thousan = reminder;

    //Thousnd
    let (reminder, divisor) = conculte_divisor_and_reminder_with_1000(divisor);
    let thousand = reminder;

    //Million
    let (reminder, divisor) = conculte_divisor_and_reminder_with_1000(divisor);
    let million = reminder;

    //Billion
    let (reminder, divisor) = conculte_divisor_and_reminder_with_1000(divisor);
    let billion = reminder;

    //Trillion
    let (reminder, divisor) = conculte_divisor_and_reminder_with_1000(divisor);
    let trillion = reminder;

    //Quardillion
    let (reminder, divisor) = conculte_divisor_and_reminder_with_1000(divisor);
    let quadrillion = reminder;

    //Quintillion
    let (reminder, _divisor) = conculte_divisor_and_reminder_with_1000(divisor);
    let quintillion = reminder;

    if quintillion > 0 {
        return_string.push_str(&(exchange_num_to_string(quintillion)));
        return_string.push_str(" quintillion");
    }

    if quadrillion > 0 {
        if quintillion > 0 {
            return_string.push_str(" ");
        }
        return_string.push_str(&(exchange_num_to_string(quadrillion)));
        return_string.push_str(" quadrillion");
    }

    if trillion > 0 {
        if quadrillion | quadrillion > 0 {
            return_string.push_str(" ");
        }
        return_string.push_str(&(exchange_num_to_string(trillion)));
        return_string.push_str(" trillion");
    }

    if billion > 0 {
        if quintillion | quadrillion | trillion > 0 {
            return_string.push_str(" ");
        }
        return_string.push_str(&(exchange_num_to_string(billion)));
        return_string.push_str(" billion");
    }

    if million > 0 {
        if quintillion | quadrillion | trillion | billion > 0 {
            return_string.push_str(" ");
        }
        return_string.push_str(&(exchange_num_to_string(million)));
        return_string.push_str(" million");
    }

    if thousand > 0 {
        if quintillion | quadrillion | trillion | billion | million > 0 {
            return_string.push_str(" ");
        }
        return_string.push_str(&(exchange_num_to_string(thousand)));
        return_string.push_str(" thousand");
    }

    if blow_thousan > 0 {
        if quintillion | quadrillion | trillion | billion | million | thousand > 0 {
            return_string.push_str(" ");
        }
        return_string.push_str(&(exchange_num_to_string(blow_thousan)));
    }

    return return_string;
}
