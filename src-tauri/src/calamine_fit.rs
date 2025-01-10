use calamine::{open_workbook, Data, Reader, Xlsx};
use tauri::Error;
use crate::data_types::*;


pub fn read(path:String)-> Result<Vec<Vec<String>>, Error> {
    let mut tdata: Wsheet = Default::default();
    
    let mut formulas: Vec<Vec<String>> = vec![];
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
    let sheet_names = workbook.sheet_names();

    for sheet_name in &sheet_names {
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
            
            if  i< 6 {
                println!("i = {}, row{:?}",i, row);
            }
            if i ==5 {
                tdata.thead = row_to_array(row)
            }
            if i == 2 {
                tdata.indate.push(row[1].to_string());
                
            }
            if i ==1 {
                //println!("{:?}", row);
                tdata.info =row[1].to_string();
                
            }
            if i == 0 {

                tdata.name = row[0].to_string();
                tdata.code = data_to_f64(&row[13]);
                //println!("{:?}", tdata);
            }
            
           
            if i>5 {
                break;
            }
            i+=1;
        }
        println!("{:?}",tdata);
        let f_sheet = workbook
            .worksheet_formula(&sheet_name).expect("sheet not found");
        // itter rows get formulas row
        for row in f_sheet.rows() {
            let mut res: Vec<String> = vec![];
            for el in row {
                res.push(el.to_string());
            }
            formulas.push(res);
        }
    }
    
    Ok(formulas)
}

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
}