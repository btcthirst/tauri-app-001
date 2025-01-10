use calamine::{open_workbook, Data, Reader, Xlsx};
use tauri::Error;



pub fn read(path:String)-> Result<Vec<Vec<Vec<String>>>, Error> {
    let mut data: Vec<Vec<Vec<String>>> = vec![];
    let mut sheet_formulas: Vec<Vec<Vec<String>>> = vec![];
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
    let sheet_names = workbook.sheet_names();
    //println!("{:?}",sheet_names.clone());
    for sheet_name in &sheet_names {
        let mut tdata: Vec<Vec<String>> = vec![];
       
        if sheet_name == &sheet_names[12]{
            break;
        }
        // get full sheet data
        let sheet = workbook
            .worksheet_range(&sheet_name)
            .unwrap();
        // itter rows of sheet

        let mut i = 0;
        for row in sheet.rows() {
             let mut head: Vec<String> = vec![];
            if i ==5 {
                tdata.push(row_to_vec(row));
            }
            if i == 2 {
                head.push(row[1].to_string());
                
            }
            if i ==1 {
                //println!("{:?}", row);
                head.push(row[1].to_string());
                
            }
            if i == 0 {

                head.push(row[0].to_string());
                head.push(row[13].to_string());
                //println!("{:?}", tdata);
            }
            if i < 5 {
                tdata.push(head.clone());
            }
            
            if i > 5 && i< 222 {
                tdata.push(row_to_vec(row));
            }
            i+=1;
        }
        let mut formulas: Vec<Vec<String>> = vec![];
        let f_sheet = workbook
            .worksheet_formula(&sheet_name).expect("sheet not found");
        // itter rows get formulas row
        for row in f_sheet.rows() {
            let mut res: Vec<String> = vec![];
            
            if row.len() < 17 && row[0].trim() == "" {
                continue;
            }
            
            for el in row {
                res.push(el.to_string());
            }
            formulas.push(res);
        }
        //println!("{:#?}", tdata.clone());
        sheet_formulas.push(formulas);
        data.push(tdata);
    }
    
    shaker(&mut data, sheet_formulas);
    
    Ok(data)
}

fn shaker(data: &mut Vec<Vec<Vec<String>>>, sheet_formulas: Vec<Vec<Vec<String>>>) {
    //println!("sheet f {:?}", sheet_formulas[8].clone());
    let mut sheet_number = 0;
    let reuse = data[11].clone();
    for sheet_data in  data {
        let mut row_number = 0;
        for row in sheet_data {
            if sheet_number == 0 {
                if row_number > 5 && row_number < 221 {                    
                    first_shake(row,reuse[row_number].clone(),&sheet_formulas[sheet_number][row_number-6])
                }
            } else {                
                if row_number > 5 && row_number < 221 {
                    
                    shake(row,&sheet_formulas[sheet_number][row_number-6])
                }
            }
            
            row_number +=1;
        }
        sheet_number +=1;
    }

}

fn first_shake(row: &mut Vec<String>, rerow: Vec<String>, f_row: &Vec<String>) {
    let mut i = 0;
    
    for el in row {
        if i < 7 {
            *el = rerow[i].clone();
        }
        if i > 7 {
            if i == 14 {
                *el = 0.to_string();
            } else {
                *el =  f_row[i-3].clone();
            }
            
        }
        i +=1;
        if i > 16 {
            break;
        }
    }
}

fn shake(row: &mut Vec<String>,f_row: & Vec<String>) {
    let mut i = 0;
    
    for el in row {
        if i > 0 {
            if i == 14 {
                *el = 0.to_string();
            } else {
                *el =  f_row[i-1].clone();
            }
            
        }
        
        i +=1;
        if i > 16 {
            break;
        }
    }
}

fn row_to_vec(row: &[Data])-> Vec<String>{
    let mut res:Vec<String> = vec![];
    for el in row {
        match el {
            Data::String(value) => {
                res.push(value.to_string());
            },
            Data::Float(value) => {
                
                res.push(format!("{:.2}", value));
            },
            Data::Empty => {
                let value = "";
                res.push(value.to_string());
            },
            _ => {}
        }
    }
    
    res
}
/* 
fn row_to_array(row: &[Data])-> [String;17] {
    let mut res: [String;17] = Default::default();
    let mut res_vec:Vec<String> = vec![];
    for el in row {
        match el {
            Data::String(value) => {
                res_vec.push(value.to_string());
            },
            _ => {}
        }
    }
    if res_vec.len() == 17 {
        res.clone_from_slice(&res_vec);        
    }
    
    res
}*/
/* 
fn data_to_f64(data: &Data)-> f64 {
    let mut res = 0f64;
    match data {
        Data::Float(val) => {
            res = *val
        },
        _ => {
            println!("Not a float value!")
        }
    }
    res
}
*/