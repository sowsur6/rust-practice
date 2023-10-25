#[derive(Clone, Debug, PartialEq)]
pub struct Table {
	pub headers: Vec<String>,
	pub body: Vec<Vec<String>>,
}

impl Table {
	pub fn new() -> Table {
		Self{
			headers:Vec::new(),
			body:Vec::new(),
		}
	}

	pub fn add_row(&mut self, row: &[String]) {
		let mut vec_str:Vec<String>=Vec::new();
		if row.len()==self.headers.clone().len(){
			for str in row{
			vec_str.push(str.to_string());
		}
		self.body.push(vec_str);
		}
	}

	pub fn filter_col<T:Fn(&str)->bool>(&self, filter: T) -> Option<Self> {
		let mut header_bool:Vec<bool>=Vec::new();
		let mut new_table=self.clone();
		new_table.headers.iter().for_each(|header|
				header_bool.push(filter(header))
		);
		
		let true_count= header_bool.clone().into_iter().filter(|b|*b==true).count();
		if true_count== header_bool.clone().len(){
			return Some(self.clone())
		}else if true_count== 0 {
			return None
		}
		let binding = header_bool.clone();
		let mut header_iter = binding.iter();
		new_table.headers.retain(|_| *header_iter.next().unwrap());
		for rows in &mut new_table.body{
		let mut row_iter=binding.iter();
			rows.retain(|_| *row_iter.next().unwrap());
		}
			
		Some(new_table)
	}

	pub fn filter_row<T:Fn(&str)->bool>(&self, col_name: &str, filter: T) -> Option<Self> {
		let mut column_index:usize=0;
		match col_name{
			"Name"=> column_index=0,
			"Last Name"=>column_index=1,
			"ID Number"=>column_index=2,
			_=>return None,
		}
		let mut table = Table::new();
		table.headers = self.headers.clone();
		for rows in self.body.clone(){
			if filter(rows[column_index].as_str()){
				table.body.push(rows);
			}
		}
		if table.body.len()!=0{
			Some(table)
		}else{
			None
		}
		
	}
}

fn main() {
	let mut table = Table::new();
	table.headers = vec![
		"Name".to_string(),
		"Last Name".to_string(),
		"ID Number".to_string(),
	];
	table.add_row(&[
		"Adam".to_string(),
		"Philips".to_string(),
		"123456789".to_string(),
	]);
	table.add_row(&[
		"Adamaris".to_string(),
		"Shelby".to_string(),
		"1111123456789".to_string(),
	]);
	table.add_row(&[
		"Ackerley".to_string(),
		"Philips".to_string(),
		"123456789".to_string(),
	]);
	let filter_names = |col: &str| col == "Name";
	println!("{:?}", table.filter_col(filter_names));

	let filter_philips = |lastname: &str| lastname == "Philips";
	println!("{:?}", table.filter_row("Last Name", filter_philips));
}

#[cfg(test)]
mod tests {
    // use filter_table::*;
    use crate::Table;

    #[test]
    fn filtering_columns() {
        let mut table = Table::new();
        table.headers = vec![
            "name".to_string(),
            "lastname".to_string(),
            "id number".to_string(),
        ];
        table.add_row(&[
            "Ackerley".to_string(),
            "Philips".to_string(),
            "123456789".to_string(),
        ]);
        table.add_row(&[
            "Adamaris".to_string(),
            "Philips".to_string(),
            "1111123456789".to_string(),
        ]);
        table.add_row(&[
            "Ackerley".to_string(),
            "Philips".to_string(),
            "123456789".to_string(),
        ]);

        let filter = |col: &str| col == "name";

        let new_table = Table {
            headers: vec!["name".to_string()],
            body: vec![
                vec!["Ackerley".to_string()],
                vec!["Adamaris".to_string()],
                vec!["Ackerley".to_string()],
            ],
        };
        assert_eq!(new_table, table.filter_col(filter).unwrap());
    }

    #[test]
    fn filtering_rows() {
        let tab = Table {
            headers: vec![
                "Name".to_string(),
                "Last Name".to_string(),
                "ID Number".to_string(),
            ],
            body: vec![
                vec![
                    "Adamaris".to_string(),
                    "Philips".to_string(),
                    "1111123456789".to_string(),
                ],
                vec![
                    "Thomas".to_string(),
                    "Shelby".to_string(),
                    "123456789".to_string(),
                ],
                vec![
                    "Ackerley".to_string(),
                    "Philips".to_string(),
                    "123456789".to_string(),
                ],
            ],
        };

        let get_fillips = |s: &str| s == "Philips";
        // filter the elements with last name Philips
        let expected_table = Table {
            headers: vec![
                "Name".to_string(),
                "Last Name".to_string(),
                "ID Number".to_string(),
            ],
            body: vec![
                vec![
                    "Adamaris".to_string(),
                    "Philips".to_string(),
                    "1111123456789".to_string(),
                ],
                vec![
                    "Ackerley".to_string(),
                    "Philips".to_string(),
                    "123456789".to_string(),
                ],
            ],
        };
        assert_eq!(
            tab.filter_row("Last Name", get_fillips).unwrap(),
            expected_table
        );
    }
}