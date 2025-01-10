use std::i32;

use chrono::{Datelike, Utc};
use rust_xlsxwriter::{Format, Formula, Workbook, XlsxError};



pub fn write(data: Vec<Vec<Vec<String>>>,path: &str) -> Result<(), XlsxError> {

    let mut book = Workbook::new();
    let sheet_names: Vec<String> = vec!["січень".to_string(), "лютий".to_string(), "березень".to_string(), "квітень".to_string(), "травень".to_string(), "червень".to_string(), "липень".to_string(), "серпень".to_string(), "вересень".to_string(), "жовтень".to_string(), "листопад".to_string(), "грудень".to_string()/* , "Лист1".to_string()*/];
    
    let mut i = 0;
    for name in &sheet_names {
        let sheet: &mut rust_xlsxwriter::Worksheet = book.add_worksheet();
        //// formats ///
        let first_format = Format::new().set_align(rust_xlsxwriter::FormatAlign::Center);
        let second_format = Format::new()
        .set_font_name("Arial Cyr").set_font_size(14)
        .set_align(rust_xlsxwriter::FormatAlign::VerticalCenter)
        .set_align(rust_xlsxwriter::FormatAlign::Center);
        let second_format_1 = Format::new()
        .set_font_name("Arial Cyr").set_font_size(14)
        .set_align(rust_xlsxwriter::FormatAlign::VerticalCenter)
        .set_border_bottom(rust_xlsxwriter::FormatBorder::Medium)
        .set_align(rust_xlsxwriter::FormatAlign::Center);
        let head_format = Format::new()
        .set_font_name("Tahoma").set_font_size(9)
        .set_align(rust_xlsxwriter::FormatAlign::Center)
        .set_align(rust_xlsxwriter::FormatAlign::VerticalCenter)
        .set_text_wrap()
        .set_border(rust_xlsxwriter::FormatBorder::Medium);

   
        set_row_hight(sheet);
        set_column_widths(sheet);
        

        let mut row = 0;
        
        for mut row_data in data[i].clone() {
            if i == 0 {
                if row == 0 {
                    
                    sheet.merge_range(row, 0, row, 2, &row_data[0], &first_format)?;
                    sheet.merge_range(row, 13, row, 16, &row_data[1], &first_format)?;
                    
                }
                if row == 1 {
                    
                    sheet.merge_range(row, 1, row, 16, &row_data[0], &second_format)?;
                }
                if row == 2 {
                    let result: String = row_data[0].split(" ").map(|el| {
                       return  match el.parse::<u32>() {
                            Ok(_val) =>{
                                let res = Utc::now().year();
                                format!("{} ",res)
                            },
                            Err(_) => {
                                format!("{} ", el)
                            }
                        }
                    }).collect();
                    sheet.merge_range(row, 1, row, 16, &result, &second_format_1)?;
                }
                if row == 5 {
                    row_data.retain(|el | el != "");
                    sheet.write_row_with_format(row, 0, &row_data, &head_format)?;
                }
                if row >5 && row < 221 {
                    write_table_row(sheet, row_data,row);
                    //sheet.write_row(row, 0, row_data)?;
                }
                
            } else {
                if row == 0 {
                    
                    sheet.merge_range(row, 0, row, 2, &row_data[0], &first_format)?;
                    sheet.merge_range(row, 13, row, 16, &row_data[1], &first_format)?;
                    
                }
                if row == 1 {
                    
                    sheet.merge_range(row, 1, row, 16, &row_data[0], &second_format)?;
                }
                if row == 2 {
                    let result: String = row_data[0].split(" ").map(|el| {
                        return  match el.parse::<u32>() {
                             Ok(_val) =>{
                                 let res = Utc::now().year();
                                 format!("{} ",res)
                             },
                             Err(_) => {
                                 format!("{} ", el)
                             }
                         }
                     }).collect();
                    sheet.merge_range(row, 1, row, 16, &result, &second_format_1)?;
                }
                if row == 5 {
                    row_data.retain(|el | el != "");
                    sheet.write_row_with_format(row, 0, &row_data, &head_format)?;
                }
                if row >5 && row < 221 {
                    write_table_row1(sheet, row_data,row);
                    //sheet.write_row(row, 0, row_data)?;
                }
            }
            

            row += 1;
        }
        table_bottom(sheet);
        sheet.set_name(name)?;
        i+=1;
    }    

    book.save(path)?;

    Ok(())
}

fn table_bottom(sheet: &mut rust_xlsxwriter::Worksheet) {
    let format = Format::new()
        .set_font_name("Tahoma").set_font_size(8)
        .set_border(rust_xlsxwriter::FormatBorder::Medium)
        .set_align(rust_xlsxwriter::FormatAlign::Center);
    sheet.merge_range(221, 0, 222, 2, "", &format).unwrap();

    sheet.merge_range(222, 3, 222, 4, "", &format).unwrap();
    sheet.write_with_format(222, 3, Formula::new("=D222-E222"),&format).unwrap();
    sheet.merge_range(222, 15, 222, 16, "", &format).unwrap();
    sheet.write_with_format(222, 15, Formula::new("=P222-Q222"),&format).unwrap();

    sheet.write_with_format(221, 3,Formula::new("=SUM(D7:D221)"),&format).unwrap();    
    sheet.write_with_format(221, 4,Formula::new("=SUM(E7:E221)"),&format).unwrap();

    sheet.merge_range(221, 5, 222, 5, "", &format).unwrap();
    sheet.merge_range(221, 6, 222, 6, "", &format).unwrap();
    sheet.write_with_format(221, 6,Formula::new("=SUM(G7:G221)"), &format).unwrap();
    sheet.merge_range(221, 7, 222, 7, "", &format).unwrap();
    sheet.write_with_format(221, 7,Formula::new("=SUM(H7:H221)"), &format).unwrap();
    sheet.merge_range(221, 8, 222, 8, "", &format).unwrap();
    sheet.merge_range(221, 9, 222, 9, "", &format).unwrap();
    sheet.write_with_format(221, 9,Formula::new("=SUM(J7:J221)"), &format).unwrap();
    sheet.merge_range(221, 10, 222, 10, "", &format).unwrap();
    sheet.write_with_format(221, 10,Formula::new("=SUM(K7:K221)"), &format).unwrap();
    sheet.merge_range(221, 11, 222, 11, "", &format).unwrap();
    sheet.write_with_format(221, 11,Formula::new("=SUM(L7:L221)"), &format).unwrap();
    sheet.merge_range(221, 12, 222, 12, "", &format).unwrap();
    sheet.write_with_format(221, 12,Formula::new("=SUM(M7:M221)"), &format).unwrap();
    sheet.merge_range(221, 13, 222, 13, "", &format).unwrap();
    sheet.write_with_format(221, 13,Formula::new("=SUM(N7:N221)"), &format).unwrap();
    sheet.merge_range(221, 14, 222, 14, "", &format).unwrap();
    sheet.write_with_format(221, 14,Formula::new("=SUM(O7:O221)"), &format).unwrap();
    
    sheet.write_with_format(221, 15,Formula::new("=SUM(P7:P221)"),&format).unwrap();    
    sheet.write_with_format(221, 16,Formula::new("=SUM(Q7:Q221)"),&format).unwrap();
}

fn write_table_row(sheet: &mut rust_xlsxwriter::Worksheet, row_data: Vec<String>, row: u32) {
    let format0 = Format::new().set_font_name("Tahoma").set_font_size(7.5).set_border(rust_xlsxwriter::FormatBorder::Thin);
    let format1 = Format::new().set_font_name("Tahoma").set_font_size(9).set_border(rust_xlsxwriter::FormatBorder::Thin);
    let format2 = Format::new().set_font_name("Tahoma").set_font_size(10).set_border(rust_xlsxwriter::FormatBorder::Thin);
    sheet.write_with_format(row, 0, &row_data[0], &format2).unwrap();
    sheet.write_with_format(row, 1, &row_data[1], &format2).unwrap();
    let rd = match row_data[2].parse::<f64>(){
        Ok(res) => {
            format!("{}", res as i32)
        },
        Err(_) => {
            row_data[2].clone()
        }
    };
    sheet.write_with_format(row, 2, &rd, &format2).unwrap();
    sheet.write_with_format(row, 3, &row_data[3], &format2).unwrap();
    sheet.write_with_format(row, 4, &row_data[4], &format2).unwrap();
    sheet.write_with_format(row, 5, &row_data[5], &format0).unwrap();
    sheet.write_with_format(row, 6, &row_data[6], &format0).unwrap();
    sheet.write_with_format(row, 7, 0, &format0).unwrap();
    sheet.write_with_format(row, 8, Formula::new(&row_data[8]), &format0).unwrap();
    sheet.write_with_format(row, 9, Formula::new(&row_data[9]), &format1).unwrap();
    sheet.write_with_format(row, 10, 0, &format2).unwrap();
    sheet.write_with_format(row, 11, 0, &format2).unwrap();
    sheet.write_with_format(row, 12, 0, &format2).unwrap();
    sheet.write_formula_with_format(row, 13, Formula::new(&row_data[13]), &format0).unwrap();
    sheet.write_with_format(row, 14, 0, &format0).unwrap();
    sheet.write_formula_with_format(row, 15, Formula::new(&row_data[15]), &format0).unwrap();
    sheet.write_formula_with_format(row, 16, Formula::new(&row_data[16]), &format0).unwrap();

}

fn write_table_row1(sheet: &mut rust_xlsxwriter::Worksheet, row_data: Vec<String>, row: u32) {
    let format0 = Format::new().set_font_name("Tahoma").set_font_size(7.5).set_border(rust_xlsxwriter::FormatBorder::Thin);
    let format1 = Format::new().set_font_name("Tahoma").set_font_size(9).set_border(rust_xlsxwriter::FormatBorder::Thin);
    let format2 = Format::new().set_font_name("Tahoma").set_font_size(10).set_border(rust_xlsxwriter::FormatBorder::Thin);
    sheet.write_with_format(row, 0, &row_data[0], &format2).unwrap();
    sheet.write_with_format(row, 1, Formula::new(&row_data[1]), &format2).unwrap();
    
    sheet.write_with_format(row, 2, Formula::new(&row_data[2]), &format2).unwrap();
    sheet.write_with_format(row, 3, Formula::new(&row_data[3]), &format2).unwrap();
    sheet.write_with_format(row, 4, Formula::new(&row_data[4]), &format2).unwrap();
    sheet.write_with_format(row, 5, Formula::new(&row_data[5]), &format0).unwrap();
    sheet.write_with_format(row, 6, Formula::new(&row_data[6]), &format0).unwrap();
    sheet.write_with_format(row, 7, 0, &format0).unwrap();
    sheet.write_with_format(row, 8, Formula::new(&row_data[8]), &format0).unwrap();
    sheet.write_with_format(row, 9, Formula::new(&row_data[9]), &format1).unwrap();
    sheet.write_with_format(row, 10, 0, &format2).unwrap();
    sheet.write_with_format(row, 11, 0, &format2).unwrap();
    sheet.write_with_format(row, 12, 0, &format2).unwrap();
    sheet.write_formula_with_format(row, 13, Formula::new(&row_data[13]), &format0).unwrap();
    sheet.write_with_format(row, 14, 0, &format0).unwrap();
    sheet.write_formula_with_format(row, 15, Formula::new(&row_data[15]), &format0).unwrap();
    sheet.write_formula_with_format(row, 16, Formula::new(&row_data[16]), &format0).unwrap();

}

fn set_row_hight(sheet: &mut rust_xlsxwriter::Worksheet) {
    sheet.set_row_height(3, 10).unwrap();
    sheet.set_row_height(4, 10).unwrap();
    sheet.set_row_height(5, 54).unwrap();
}

fn set_column_widths(sheet: &mut rust_xlsxwriter::Worksheet) {
    sheet.set_column_width(0, 5.6).unwrap();
    sheet.set_column_width(1, 19.85).unwrap();
    sheet.set_column_width(2, 7.15).unwrap();
    sheet.set_column_width(3, 8.5).unwrap();
    sheet.set_column_width(4, 8.5).unwrap();
    sheet.set_column_width(5, 4.6).unwrap();
    sheet.set_column_width(6, 7.5).unwrap();
    sheet.set_column_width(7, 5.72).unwrap();
    
    sheet.set_column_width(8, 7).unwrap();
    sheet.set_column_width(9, 8.5).unwrap();
    sheet.set_column_width(10, 7.2).unwrap();
    sheet.set_column_width(11, 7.5).unwrap();
    sheet.set_column_width(12, 6).unwrap();
    sheet.set_column_width(13, 8.5).unwrap();
    sheet.set_column_width(14, 8.5).unwrap();
    sheet.set_column_width(15, 10).unwrap();
    sheet.set_column_width(16, 10).unwrap();
}