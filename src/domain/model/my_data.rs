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

impl MyData {
    pub fn into_my_data_for_front(self) -> MyDataForFront {
        MyDataForFront {
            id: self.id,
            name: self.name,
        }
    }
}

#[derive(Insertable, Deserialize)]
#[table_name = "my_datas"]
pub struct NewMyData {
    my_id: String,
    password: String,
    name: String,
}

#[derive(Deserialize)]
pub struct SignInData {
    pub my_id: String,
    pub password: String,
}
