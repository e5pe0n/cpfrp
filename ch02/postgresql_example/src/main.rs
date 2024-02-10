use postgres::{error::Error, Client, NoTls};

#[derive(Debug)]
struct SaleWithProduct {
    category: String,
    name: String,
    quantity: f64,
    unit: String,
    date: i64,
}

fn create_db() -> Result<Client, Error> {
    let username = "postgres";
    let password = "password";
    let host = "postgres";
    let port = "5432";
    let database = "postgres";
    let mut conn = Client::connect(
        &format!(
            "postgres://{}:{}@{}:{}/{}",
            username, password, host, port, database
        ),
        NoTls,
    )?;
    conn.execute("drop table if exists Sales", &[])?;
    conn.execute("drop table if exists Products", &[])?;
    conn.execute(
        "create table Products (
        id integer primary key,
        category text not null,
        name text not null unique
        )",
        &[],
    )?;
    conn.execute(
        "create table Sales (
        id text primary key,
        product_id integer not null references Products,
        sale_date bigint not null,
        quantity double precision not null,
        unit text not null
        )",
        &[],
    )?;
    Ok(conn)
}

fn populate_db(conn: &mut Client) -> Result<(), Error> {
    conn.execute(
        "insert into Products (
        id, category, name
    ) values ($1, $2, $3)",
        &[&1_i32, &"fruit", &"pears"],
    )?;
    conn.execute(
        "insert into Sales (
        id, product_id, sale_date, quantity, unit
    ) values ($1, $2, $3, $4, $5)",
        &[&"2020-183", &1i32, &1_234_567_890_i64, &7.439_f64, &"Kg"],
    )?;
    Ok(())
}

fn print_db(conn: &mut Client) -> Result<(), Error> {
    for row in &conn.query(
        "select p.name, s.unit, s.quantity, s.sale_date
    from Sales s
    left join Products p
    on p.id = s.product_id
    order by s.sale_date",
        &[],
    )? {
        let sale_with_product = SaleWithProduct {
            category: "".to_string(),
            name: row.get(0),
            quantity: row.get(2),
            unit: row.get(1),
            date: row.get(3),
        };
        println!(
            "At instant {}, {} {} of {} were sold.",
            sale_with_product.date,
            sale_with_product.quantity,
            sale_with_product.unit,
            sale_with_product.name
        );
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    let mut conn = create_db()?;
    populate_db(&mut conn)?;
    print_db(&mut conn)?;
    Ok(())
}
