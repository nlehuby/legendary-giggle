extern crate osmpbfreader;
extern crate rustc_serialize;
extern crate csv;

#[allow(dead_code)]
/// only used for debugging purposes
fn wanted(obj: &osmpbfreader::OsmObj) -> bool {
    match *obj {
        osmpbfreader::OsmObj::Relation(ref rel) => rel.id == 1066570,//id of relation for bus 67
        //osmpbfreader::OsmObj::Relation(ref rel) => rel.id == 1257187,//id of relation for bus 57
        _ => false,
    }
}

fn is_route_master(obj: &osmpbfreader::OsmObj) -> bool {
    obj.relation()
        .and_then(|r| r.tags.get("type"))
        .map_or(false, |v| ["route_master"].contains(&v.as_str()))
}

fn extract_routes_from_route_master(route_master: &osmpbfreader::Relation) -> Option<Vec<String>> {
    let mut all_routes_for_this_line: Vec<String> = vec![];

    for member in &route_master.refs {
        match member.member {
            osmpbfreader::OsmId::Relation(rel_id) => {
                all_routes_for_this_line.push(rel_id.to_string());
            }
            _ => {}
        }
    }

    Some(all_routes_for_this_line)
}



fn main() {
    let filename = std::env::args_os().nth(1).unwrap();
    let path = std::path::Path::new(&filename);
    let r = std::fs::File::open(&path).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::new(r);
    let objects = osmpbfreader::get_objs_and_deps(&mut pbf, wanted).unwrap();

    let result = objects.values()
        .filter(|x| is_route_master(*x))
        .filter_map(osmpbfreader::OsmObj::relation)
        .filter_map(|relation| extract_routes_from_route_master(relation))
        .into_iter()
        .flat_map(|x| x)
        .collect::<Vec<String>>();

    println!("{:?}", result);


}
