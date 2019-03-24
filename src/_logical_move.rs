impl LogicalMove {
    pub fn new() -> LogicalMove {
        LogicalMove {
            source_file:0,
            source_rank:0,
            destination_file:0,
            destination_rank:0,
            promotion:false,
            drop:None,
        }
    }
}
