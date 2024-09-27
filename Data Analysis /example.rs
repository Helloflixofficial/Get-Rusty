use std::thread;
fn main() {
    let mut handles = vec![];
    let mut x = 0;
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            x += 1;
            println!("Hello from thread {} with x = {}", i, x);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}


use polars::prelude::*;
fn main() -> Result<(), PolarsError> {
    let df = CsvReader::from_path("data.csv")?
        .infer_schema(None)
        .has_header(true)
        .finish()?;
    println!("{:?}", df.head(Some(5)));
    let filtered_df = df.filter(&df.column("column_name")?.gt_eq(10)?)?;
    println!("{:?}", filtered_df);
    let selected_df = df.select(&["column1", "column2"])?;
    println!("{:?}", selected_df);
    Ok(())
}

