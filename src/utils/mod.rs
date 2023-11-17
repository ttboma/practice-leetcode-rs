pub fn parse_list_i32(input: &str) -> Vec<i32> {
    nom_parser::parse_list(input)
        .expect(error_msg::LIST_FORMAT)
        .1
        .into_iter()
        .map(|x| x.parse::<i32>().expect(error_msg::ITEM_OF_I32))
        .collect::<Vec<_>>()
}

pub fn parse_2d_list_i32(input: &str) -> Vec<Vec<i32>> {
    nom_parser::parse_2d_list(input)
        .expect(error_msg::TWO_DIMENSION_LIST_FORMAT)
        .1
        .into_iter()
        .map(|x| {
            x.into_iter()
                .map(|y| y.parse::<i32>().expect(error_msg::ITEM_OF_I32))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

mod error_msg;
mod nom_parser;
