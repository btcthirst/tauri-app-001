#[derive(Debug)]
//////// SIMPLE WORK///////////////////////////////////////////////////////



///////////////////////////////////////////////////////////////////////////
pub enum SheetData {
    String(String),
    Float(f64)
}

#[derive(Debug,Default)]
pub struct Wsheet {
    pub name:String,
    pub code:f64,
    pub info:String,
    pub indate:Vec<String>,
    pub thead: [String;17]
}

///////////IN THEORY////////////

/* 
#[warn(dead_code)]
#[derive(Debug,Default)]
pub struct BookW {
    name: String,
    path: String, //in mind
    sheets_name: Vec<String>,
    sheets: Vec<ListW>
}

#[derive(Debug)]
pub enum ListW {
    Accrual(AccrualSh),
    Outcoms(OutcomsSh)
}

#[derive(Debug)]
pub struct AccrualSh{
    head: Vec<String>,
    thead: Vec<String>,
    table: Vec<CellData>
}

#[derive(Debug)]
pub struct OutcomsSh{}

#[derive(Debug)]
enum CellData {
    String(String),
    Float(f64),
    Formula(String),
    Emty
}
*/