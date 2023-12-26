use super::operation::Operation;

pub trait OperationWithKey<Coord>: Operation<Coord>
where
    u32: From<Coord>,
    u64: From<Coord>,
{
    fn get_using_key(&self) -> u64;

    fn get_need_deleted_key(&self) -> u64;

    // Porting note: renamed to avoid shadowing the trait method
    fn to_unique_key_with_delete_key(&self) -> u64 {
        const MASK_LOW: u64 = (1 << 30) - 1;
        const MASK_HIGH: u64 = MASK_LOW << 30;

        let need_deleted_key = self.get_need_deleted_key();
        let unique_deleted_key =
            (need_deleted_key & MASK_HIGH) | (need_deleted_key & MASK_LOW) << 35;

        unique_deleted_key + self.to_unique_key()
    }
}
