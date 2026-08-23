// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently, the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct `ReportCard` and the impl
// block to support alphabetical report cards in addition to numerical ones.

// TODO: Adjust the struct as described above.
use std::fmt::Display;
struct ReportCard<T: Display> {
    grade: T,
    student_name: String,
    student_age: u8,
}

// TODO: Adjust the impl block as described above.
impl<T: Display> ReportCard<T> {
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade,
        )
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+",
        );
    }
}

/*
WHat the problem was
 In the reportCard Struct, grade was only on a numeric scale and could not allow for alphabetical grades to be displayed in the report

 How does <T> fix this
 In the instance of ReportCard<T>, grade: T is allowing for any type to be reported per instance. so the numeric ReportCard and the borrowed String ReportCard can both exist in the same struc definition

 How does Display fix this problem
 firstly display formats specific values denoted with a {} into human readable text and Since T can be subsituted into any typem we cant assume thee compailer can implement display unless specifically told it can and in the example of T: Display, there is that explicit telling which lets the format's {} compile for the generic T 
*/