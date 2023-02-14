use super::data::Data;
use mysql::{self, prelude::Queryable};
pub struct Db{
    conn: mysql::PooledConn
}

impl Db{
    pub fn new() -> Self{
        let pool = mysql::Pool::new("mysql://root:xx1121!!!!@localhost:3306/rust").expect("실패");
        Self{
            conn: pool.get_conn().unwrap()
        }
    }

    pub fn new_user(&mut self, id: &String, dp_name: &String){
        let k = format!("insert into cnt values ({:?}, {:?}, 0)", id, dp_name);
        self.conn.exec_drop(k.trim(), ()).unwrap();
    }

    pub fn get_user(&mut self, id: &String, dp_name: &String) -> Vec<Data>{
        let k = format!("select * from cnt where id={:?} dp_name={:?}", id, dp_name);

        self.conn.query_map(k.trim(), |(id, dp_name, a)|Data{id, dp_name, a}).unwrap()
    }
    pub fn cnt_up(&mut self, id: &String, dp_name: &String, up: usize){
        let data = self.get_user(id, dp_name);
        let k = format!("update cnt set a={} where id={id:?} dp_name={dp_name:?}", data[0].a + up);
        self.conn.exec_drop(k.trim(), ()).unwrap();
    }
}