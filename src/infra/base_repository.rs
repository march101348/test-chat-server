pub trait BaseRepository {
    type Data;
    type NewData;

    fn create_data(&self, data: NewData) -> Data;

    fn read_one_data(&self, id: i32) -> Result<Data, std::error::Error>;

    fn update_data(&self, id: i32) -> Result<Data, std::error::Error>;

    fn delete_data(&self, id: i32) -> Result<(), std::error::Error>;
}