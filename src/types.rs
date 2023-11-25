enum ValTypes {
    Int
}

pub struct QueryVal{
    qType: ValTypes,
    val: String,
}

impl QueryVal {
    pub fn int() -> Self{
        return QueryVal { qType: (ValTypes::Int), val: String::from("1") }
    }
}