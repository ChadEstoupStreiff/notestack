

pub mod notes {
    use mysql::*;
    use mysql::prelude::*;

    pub struct Note {
        pub id: String,
        pub note: String,
        pub date: String
    }

    impl FromRow for Note {
        fn from_row_opt(row: Row) -> std::result::Result<Self, FromRowError>
        where
            Self: Sized {
            Ok(Note {
                id: row.get::<String, _>(0).unwrap(),
                note: row.get::<String, _>(1).unwrap(),
                date: row.get::<String, _>(2).unwrap()
            })
        }
    }

    
    pub fn get_db_conn() -> PooledConn {
        let url: &str = "mysql://chadtest:testpwd@localhost:33061/test";
        let pool: Pool = Pool::new(url).unwrap();
        let mut conn: PooledConn = pool.get_conn().unwrap();
        init_db(&mut conn);
        conn
    }

    fn init_db(conn: &mut PooledConn) {
        conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS Notes (
            id VARCHAR(30),
            note VARCHAR(65000),
            date VARCHAR(19),
            CONSTRAINT pk_id PRIMARY KEY (id)
        );").unwrap()
    }

    pub fn insert_note(conn: &mut PooledConn, id: &String, note: String) -> Result<()> {
        conn.exec_drop(r"INSERT INTO Notes (id, note, date) VALUES (:note_id, :note, NOW())", params!{
            "note_id" => id,
            "note" => note
        })
    }

    pub fn get_note(conn: &mut PooledConn, id: String) -> Result<Note> {

        let stmt: Statement = conn.prep("SELECT id, note, date FROM Notes WHERE id=:note_id").unwrap();
        match conn.exec_map::<Note, _, _, _, Note>(&stmt, params! {
            "note_id" => &id
        }, |note: Note| {
            note
        },) {
            Ok(v) => {
                if v.len() > 0 {
                    let note: &Note = v.get(0).unwrap();
                    Ok(Note{
                        id: note.id.clone(),
                        note: note.note.clone(),
                        date: note.date.clone()
                    })
                } else {
                    Err(mysql::Error::MySqlError(mysql::MySqlError{
                        state: String::from("No value"),
                        message: String::from("No value available on this query"),
                        code: 1
                    }))
                }
            },
            Err(e) => Err(e)
        }

        // let note: std::option::Option<String> = conn.exec_first(&stmt, params! {
        //     "note_id" => id
        // }).unwrap();
        // format!("{:?}", note)
    }

    pub fn get_free_id(conn: &mut PooledConn) -> String {
        let mut i: i64 = 0;
        loop {
            let stmt: Statement = conn.prep("SELECT id FROM Notes WHERE id=:note_id").unwrap();
            match conn.exec_first::<String, &Statement, Params>(&stmt, params! {
                "note_id" => i
            }) {
                Ok(t) => match t {
                    None => return format!("{}", i),
                    _ => ()
                },
                Err(_) => ()
            }

            i += 1;
        }
    }
}