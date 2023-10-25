// use std::fmt;

// #[derive(Clone, Debug, PartialEq)]
// pub struct Table {
//     pub headers: Vec<String>,
//     pub body: Vec<Vec<String>>,
// }

// impl fmt::Display for Table {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         if self.headers.is_empty() || self.body.is_empty() {
//             // if the table is empty, don't print anything
//             return Ok(());
//         }
        
//         let num_cols = self.headers.len();
//         let col_widths: Vec<usize> = (0..num_cols)
//             .map(|i| {
//                 // find the max width of each column
//                 let col = self.body.iter().map(|row| row[i].len()).chain(std::iter::once(self.headers[i].len()));
//                 col.max().unwrap()
//             })
//             .collect();

//         // calculate the total width of the table
//         let total_width = col_widths.iter().sum::<usize>() + num_cols + 1;

//         // print the top border
//         writeln!(f, "{}", "+".repeat(total_width))?;

//         // print the header row
//         write!(f, "|")?;
//         for (i, header) in self.headers.iter().enumerate() {
//             write!(f, " ")?;
//             let padding = col_widths[i] - header.len();
//             let pad_left = padding / 2;
//             let pad_right = padding - pad_left;
//             write!(f, "{}{}{}", " ".repeat(pad_left), header, " ".repeat(pad_right))?;
//             write!(f, " |")?;
//         }
//         writeln!(f)?;

//         // print the separator row
//         writeln!(f, "{}", "+".repeat(total_width))?;

//         // print the body rows
//         for row in &self.body {
//             write!(f, "|")?;
//             for (i, col) in row.iter().enumerate() {
//                 write!(f, " ")?;
//                 let padding = col_widths[i] - col.len();
//                 let pad_left = padding / 2;
//                 let pad_right = padding - pad_left;
//                 write!(f, "{}{}{}", " ".repeat(pad_left), col, " ".repeat(pad_right))?;
//                 write!(f, " |")?;
//             }
//             writeln!(f)?;
//         }

//         // print the bottom border
//         writeln!(f, "{}", "+".repeat(total_width))
//     }
// }

// impl Table {
//     pub fn new() -> Table {
//         Table {
//             headers: Vec::new(),
//             body: Vec::new(),
//         }
//     }

//     pub fn add_row(&mut self, row: &[String]) {
//         self.body.push(row.to_vec());
//     }
// }

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.headers.is_empty() {
            // if the table is empty, don't print anything
            return Ok(());
        
    }
        // Calculate the maximum width for each column
        let mut col_widths: Vec<usize> = self.headers.iter().map(|header| header.len()).collect();
        for row in self.body.iter() {
            for (i, col) in row.iter().enumerate() {
                col_widths[i] = col_widths[i].max(col.len());
            }
        }

        // Print the header row
        write!(f, "| ")?;
        for (i, header) in self.headers.iter().enumerate() {
            write!(f, "{:^1$}", header, col_widths[i])?;
            if i == self.headers.len()-1{
                write!(f, " |")?;
            }else{
               write!(f, " | ")?; 
            }
            
        }
        writeln!(f)?;

        // Print the column separator
        write!(f, "|")?;
        for (i,width) in col_widths.iter().enumerate() {
            write!(f, "{}", "-".repeat(width+2))?;
            if i == &col_widths.len()-1{
                write!(f, "|")?;
            }else{
            write!(f, "+")?;
            }
        }
        writeln!(f)?;

        // Print the body rows
        for (j, row) in self.body.iter().enumerate() {
            if j == self.body.len()-1{
                write!(f, "| ")?;
            }else{
               write!(f, "| ")?; 
            }
            for (i, col) in row.iter().enumerate() {
                write!(f, "{:^1$}", col, col_widths[i])?;
                    if i == row.len()-1{
                       write!(f, " |")?;
                    }else{
                       write!(f, " | ")?;  
                    }
                }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Table {
    pub fn new() -> Table {
        Table {
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: &[String]) {
        self.body.push(row.to_vec());
    }
}



fn main() {
	let mut table = Table::new();
	println!("{}", table);
	table.headers = vec![
		String::from("Model"),
		String::from("Piece N°"),
		String::from("In Stock"),
		String::from("Description"),
	];
	table.add_row(&[
		String::from("model 1"),
		String::from("43-EWQE304"),
		String::from("30"),
		String::from("Piece for x"),
	]);
	table.add_row(&[
		String::from("model 2"),
		String::from("98-QCVX5433"),
		String::from("100000000"),
		String::from("-"),
	]);
	table.add_row(&[
		String::from("model y"),
		String::from("78-NMNH"),
		String::from("60"),
		String::from("nothing"),
	]);
	println!("{}", table);
}

#[cfg(test)]
mod tests {
    // use display_table::*;
    use super::*;

    #[test]
    fn it_displays() {
        let mut table = Table::new();
        table.headers = vec![
            "Name".to_string(),
            "Last Name".to_string(),
            "ID Number".to_string(),
        ];
        table.add_row(&[
            "Ackerley".to_string(),
            "Fillips".to_string(),
            "123456789".to_string(),
        ]);
        table.add_row(&[
            "Adamaris".to_string(),
            "Fillips".to_string(),
            "1111123456789".to_string(),
        ]);
        table.add_row(&[
            "Ackerley".to_string(),
            "Fillips".to_string(),
            "123456789".to_string(),
        ]);
        assert_eq!(
			table.to_string(),
			"|   Name   | Last Name |   ID Number   |\n|----------+-----------+---------------|\n| Ackerley |  Fillips  |   123456789   |\n| Adamaris |  Fillips  | 1111123456789 |\n| Ackerley |  Fillips  |   123456789   |\n"
		);
    }

    // An empty table must not display anything
    #[test]
    fn display_table_with_no_headers() {
        let table = Table::new();
        assert_eq!(table.to_string(), "");
    }

    #[test]
    fn display_table_with_headers_only() {
        let mut table = Table::new();
        table.headers = vec![
            "Name".to_string(),
            "Last Name".to_string(),
            "ID Number".to_string(),
        ];
        assert_eq!(
            table.to_string(),
            "| Name | Last Name | ID Number |\n|------+-----------+-----------|\n"
        );
    }

    #[test]
    fn display_second() {
        let mut table = Table::new();
        table.headers = vec![
            "ID".to_string(),
            "Car Brand".to_string(),
            "Model".to_string(),
            "Is Electric".to_string(),
        ];
        table.add_row(&[
            "1".to_string(),
            "Tesla".to_string(),
            "Model 3".to_string(),
            "True".to_string(),
        ]);
        table.add_row(&[
            "2".to_string(),
            "Ford".to_string(),
            "Focus".to_string(),
            "False".to_string(),
        ]);
        assert_eq!(
			table.to_string(),
			"| ID | Car Brand |  Model  | Is Electric |\n|----+-----------+---------+-------------|\n| 1  |   Tesla   | Model 3 |    True     |\n| 2  |   Ford    |  Focus  |    False    |\n"
		);
    }
}
