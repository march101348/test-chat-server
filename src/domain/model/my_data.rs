use crate::schema::my_datas;

// TODO: pub修飾付けて良いものか？
#[derive(Serialize, Deserialize)]
pub struct MyDataForFront {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct MyData {
    pub id: i32,
    my_id: String,
    password: String,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "my_datas"]
pub struct NewMyData {
    my_id: String,
    password: String,
    name: String,
}
