use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct SaleWithProduct {
    category: String,
    name: String,
    quantity: f64,
    unit: String,
    date: i64,
}

fn create_db() -> Result<Connection> {
    let db_file = "sales.db";
    let conn = Connection::open(db_file)?;
    conn.execute("drop table if exists Sales", params![])?;
    conn.execute("drop table if exists Products", params![])?;
    conn.execute(
        "create table Products (
        id interger primary key,
        category text not null,
        name text not null unique
    )",
        params![],
    )?;
    conn.execute(
        "create table Sales (
            id text primary key,
            product_id integer not null references Products,
            sale_date bigint not null,
            quantity double precision not null,
            unit text not null
    )",
        params![],
    )?;
    Ok(conn)
}

fn populate_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "insert into Products (
        id, category, name
    ) values ($1, $2, $3)",
        params![1, "fruit", "pears"],
    )?;
    conn.execute(
        "insert into Sales (
        id, product_id, sale_date, quantity, unit
    ) values ($1, $2, $3, $4, $5)",
        params!["2020-183", 1, 1_234_567_890_i64, 7.439, "Kg"],
    )?;
    Ok(())
}

fn print_db(conn: &Connection) -> Result<()> {
    let mut comm = conn.prepare(
        "select p.name, s.unit, s.quantity, s.sale_date
        from Sales s
        left join Products p
        on p.id = s.product_id
        order by s.sale_date",
    )?;
    for sale_with_product in comm.query_map(params![], |row| {
        Ok(SaleWithProduct {
            category: "".to_string(),
            name: row.get(0)?,
            quantity: row.get(2)?,
            unit: row.get(1)?,
            date: row.get(3)?,
        })
    })? {
        if let Ok(item) = sale_with_product {
            println!(
                "At instant {}, {} {} of {} were sold.",
                item.date, item.quantity, item.unit, item.name
            );
        }
    }
    Ok({})
}

fn main() -> Result<()> {
    let conn = create_db()?;
    populate_db(&conn)?;
    print_db(&conn)?;
    Ok(())
}
