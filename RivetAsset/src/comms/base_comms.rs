pub trait communication
{
    fn GetData(&self);
    fn PostData(&self, data:Vec<u8>);
}